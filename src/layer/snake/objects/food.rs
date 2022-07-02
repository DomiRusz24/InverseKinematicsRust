use crate::Point;

pub type FoodTypes = Vec<String>;

#[derive(Clone)]
pub struct Food {
    pub texture: String,
    pub size: f64,
    pub amount: u32,
    pub point: Point
}