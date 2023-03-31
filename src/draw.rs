use {
  crate::{Client, Game, BLACK, UNIT_SIZE},
  sdl2::rect::Rect,
};

pub fn draw_game(client: &mut Client, game: Game) -> Result<(), String> {
  client.canvas.set_draw_color(BLACK);
  let x = game.unit_position.x as i32 - UNIT_SIZE as i32;
  let y = game.unit_position.y as i32 - UNIT_SIZE as i32;
  let width = UNIT_SIZE * 2 + 1;
  let height = width.clone();
  client.canvas.fill_rect(Rect::new(x, y, width, height))?;
  Ok(())
}
