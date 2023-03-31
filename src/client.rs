use {
  crate::{Client, GAME_HEIGHT, GAME_WIDTH, WHITE},
  sdl2::{render::Canvas, video::Window, EventPump},
};

impl Client {
  pub fn new() -> Result<Client, String> {
    let sdl = sdl2::init()?;
    let mut canvas: Canvas<Window> = sdl
      .video()?
      .window("pong", GAME_WIDTH as u32, GAME_HEIGHT as u32)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|e| e.to_string())?;
    let event_pump: EventPump = sdl.event_pump()?;

    canvas.set_draw_color(WHITE);
    canvas.clear();

    Ok(Client { canvas, event_pump })
  }

  pub fn present(&mut self) { self.canvas.present(); }

  pub fn draw_white_background(&mut self) {
    self.canvas.set_draw_color(WHITE);
    self.canvas.clear();
  }
}
