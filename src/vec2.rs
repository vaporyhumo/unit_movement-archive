use crate::Vec2;

impl Vec2 {
  pub fn unit_vector(&self, other: Vec2) -> Vec2 {
    let x = other.x - self.x;
    let y = other.y - self.y;
    let length = (x.powi(2) + y.powi(2)).sqrt();
    Vec2 { x: x / length, y: y / length }
  }

  pub fn clamp(&self, other: Vec2) -> Vec2 {
  }
}
