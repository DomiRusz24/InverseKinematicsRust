use vecmath::Vector2;

pub type Point = Vector2<f64>;

#[derive(Clone, Copy)]
pub struct PointDirection {
    pub loc: Point,
    pub direction: Point
}

impl PointDirection {
    pub fn new(loc: Point, direction: Point) -> Self {
        PointDirection {
            loc, direction
        }
    }

}

