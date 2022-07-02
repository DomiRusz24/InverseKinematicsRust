use std::cmp;
use graphics::types::Color;
use vecmath::vec2_sub;
use crate::util::{Point, PointDirection};

pub struct Snake {
    body: Vec<PointDirection>,
    pub power: u32,
    color: Color
}

impl Snake {
    pub fn new(head: Point, color: Color) -> Self {
        let mut s = Snake {
            body: Vec::new(),
            power: 0,
            color
        };

        s.reset_body(head);

        s
    }

    pub fn get_speed(&self) -> f64 {
        (self.power as f64 * 0.035) + 5.0
    }

    pub fn get_nodes(&self) -> u32 {
        ((self.power as f64 / 10.) as u32) + 50
    }

    pub fn get_node_length(&self) -> f64 {
        (self.power as f64 * 0.1) + 12.
    }

    pub fn get_size(&self) -> f64 {
        (self.power as f64 * 0.1) + 10.
    }

    pub fn get_scale(&self) -> f64 {
        (self.power as f64 * 0.0015) + 1.
    }

    pub fn get_body(&self) -> Vec<PointDirection> {
        self.body.clone()
    }

    pub fn border_collision(&mut self, width: f64, height: f64, border_width: f64) {
        use vecmath::*;

        let head = self.get_head();
        let new_location = vec2_add(head.loc, head.direction);

        let size = (self.get_size() / 2.) + border_width;

        let min = size;
        let max_x = width - size;
        let max_y = height - size;

        if new_location[0] < min || new_location[0] > max_x || new_location[1] < min || new_location[1] > max_y {
            let x = new_location[0].max(min).min(max_x) - head.loc[0];
            let y = new_location[1].max(min).min(max_y) - head.loc[1];

            self.replace_body(0, PointDirection::new(head.loc, [x, y]));
        }
    }

    pub fn move_towards(&mut self, direction: Point) {
        use vecmath::*;
        let speed = self.get_speed();
        let direction = vec2_mul(vec2_normalized(direction), [speed, speed]);

        let location = self.get_head().loc;
        self.replace_body(0, PointDirection::new(location, direction));
    }

    pub fn update_locations(&mut self) {
        use vecmath::*;

        let point = self.get_head();
        let new_location = vec2_add(point.loc, point.direction);
        self.replace_body(0, PointDirection::new(new_location, vec2_normalized(point.direction)));

        self.update_body();
    }

    fn update_body(&mut self) {
        use vecmath::*;

        let length: f64 = self.get_node_length() as f64;

        let mut i = 1;
        let size = self.body.len();
        while i < size {

            let parent = *self.body.get(i - 1).unwrap();
            let child = *self.body.get(i).unwrap();

            let direction = vec2_normalized_sub(child.loc, parent.loc);

            let new_location = vec2_sub(parent.loc, vec2_mul(direction, [-length, -length]));

            self.replace_body(i, PointDirection::new(new_location, direction));

            i+= 1;
        }
    }

    fn replace_body(&mut self, index: usize, point: PointDirection) {
        self.body.insert(index, point);
        self.body.remove(index + 1);
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_head(&self) -> PointDirection {
        *self.body.get(0).unwrap()
    }

    pub fn reset_body(&mut self, head: Point) {
        self.body.clear();

        let mut i = 0;
        let nodes = self.get_nodes();
        let length = self.get_node_length();



        while i < nodes {
            self.body.push(PointDirection::new(vec2_sub(head, [length * i as f64, 0.]), [1., 0.]));
            i+= 1;
        }

    }
}