use {
  draw::draw_game,
  sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window, EventPump,
  },
  std::time::Duration,
};

mod client;
mod draw;
mod game;
mod vec2;

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;
const UNIT_SIZE: u32 = 10;

const BLACK: Color = Color::RGBA(0, 0, 0, 255);
const WHITE: Color = Color::RGBA(255, 255, 255, 255);

#[derive(Debug, Clone)]
pub struct UnitPosition {
  x: f64,
  y: f64,
}

pub struct Client {
  pub canvas: Canvas<Window>,
  pub event_pump: EventPump,
}

#[derive(Debug, Clone)]
pub struct UnitDestination {
  x: f64,
  y: f64,
}

#[derive(Debug, Clone)]
pub struct Game {
  unit_position: UnitPosition,
  unit_destination: Option<UnitDestination>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Vec2 {
  x: f64,
  y: f64,
}

fn game_loop(
  mut client: Client,
  mut game: Game,
  mut exit_game: bool,
) -> Result<(), String> {
  if exit_game {
    return Ok(());
  }

  print!("\x1B[2J\x1B[1;1H");
  println!("game: {:?}", game);
  client.draw_white_background();

  let events: Vec<Event> = client
    .event_pump
    .poll_iter()
    .take_while(|event| match event {
      Event::Quit { .. }
      | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        exit_game = true;
        false
      }
      _ => true,
    })
    .collect();

  if exit_game {
    return Ok(());
  }

  game = game.process_events(events);
  game = game.next();

  draw_game(&mut client, game.clone())?;
  client.canvas.present();
  std::thread::sleep(Duration::from_millis(100));

  game_loop(client, game, exit_game)
}

fn main() -> Result<(), String> {
  let exit_game = false;
  let mut client: Client = Client::new()?;
  let game: Game = Game::new();

  draw_game(&mut client, game.clone())?;
  client.present();

  game_loop(client, game, exit_game)?;
  Ok(())
}
