pub struct Map {
    pub width: f64,
    pub height: f64,
    pub border_width: f64
}

impl Map {
    pub fn new(width: f64, height: f64, border_width: f64) -> Self {
        Map {
            width, height, border_width
        }
    }
}