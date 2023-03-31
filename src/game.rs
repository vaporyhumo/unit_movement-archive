use crate::Vec2;

use {
  crate::{Game, UnitDestination, UnitPosition},
  sdl2::{event::Event, mouse::MouseButton},
};

impl From<UnitDestination> for Vec2 {
  fn from(unit_destination: UnitDestination) -> Vec2 {
    Vec2 { x: unit_destination.x, y: unit_destination.y }
  }
}

impl From<UnitPosition> for Vec2 {
  fn from(unit_position: UnitPosition) -> Vec2 {
    Vec2 { x: unit_position.x, y: unit_position.y }
  }
}

impl Game {
  pub fn new() -> Game {
    Game {
      unit_position: UnitPosition { x: 0.0, y: 0.0 },
      unit_destination: None,
    }
  }

  pub fn process_events(&self, events: Vec<Event>) -> Game {
    let game = self.clone();
    events.iter().fold(game, |game, event| game.process_event(event.clone()))
  }

  fn process_event(&self, event: Event) -> Game {
    let game = self.clone();
    match event {
      Event::MouseButtonUp { mouse_btn: MouseButton::Right, x, y, .. } => {
        Game {
          unit_destination: Some(UnitDestination { x: x as f64, y: y as f64 }),
          ..game
        }
      }
      _ => game,
    }
  }

  pub fn next(&self) -> Game {
    let game = self.clone();
    game
      .unit_destination.clone()
      .map(|destination| {
        if Vec2::from(destination.clone()) == Vec2::from(game.clone().unit_position) {
          Game { unit_destination: None, ..game }
        } else {
          let unit_position = game.unit_position.next(destination);
          Game { unit_position, ..game }
        }
      })
      .unwrap_or(self.clone())
  }
}

impl UnitPosition {
  fn next(&self, destination: UnitDestination) -> UnitPosition {
    let unit_position = self.clone();
    let unit_destination = Vec2::from(destination);
    let unit_position_vec2 = Vec2::from(unit_position.clone());
    let unit_vector = unit_position_vec2.unit_vector(unit_destination);
    UnitPosition {
      x: unit_position.x + unit_vector.x,
      y: unit_position.y + unit_vector.y,
    }
  }
}
