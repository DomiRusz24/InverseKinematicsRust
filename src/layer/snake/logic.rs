use graphics::{color, Context};
use graphics::ellipse::centered;
use graphics::math::overlap_rectangle;
use graphics::rectangle::rectangle_by_corners;
use opengl_graphics::GlGraphics;
use piston::MouseButton;
use rand::Rng;
use crate::layer::Layer;
use crate::{Button, ButtonState, Map, Point, RenderArgs, RendererCache, Snake};
use crate::layer::snake::render::SnakeRenderer;
use crate::layer::snake::objects::food::{Food, FoodTypes};

pub struct SnakeGameLayer {
    mouse_pressed: Option<MouseButton>,
    mouse_offset: Point,
    resolution: Point,
    map: Map,
    snake: Snake,
    food_types: FoodTypes,
    apples: Vec<Food>,
    tick: u64
}

impl Layer for SnakeGameLayer {
    fn mouse_click(&mut self, button: Button, button_state: ButtonState) {
        if let Button::Mouse(mouse) = button {
            if button_state == ButtonState::Press {
                self.mouse_pressed = Some(mouse)
            } else {
                self.mouse_pressed = None;
            }
        }
    }

    fn mouse_update(&mut self, point: Point) {
        let center = vecmath::vec2_mul(self.resolution, [0.5, 0.5]);
        self.mouse_offset = vecmath::vec2_sub(point, center);
    }

    fn resolution_update(&mut self, x: f64, y: f64) {
        self.resolution = [x, y];
    }

    fn render(&self, c: Context, gl: &mut GlGraphics, args: RenderArgs, cache: &mut RendererCache) {
        let mut renderer = SnakeRenderer::new(c, gl, args);

        let center = vecmath::vec2_mul(self.resolution, [0.5, 0.5]);

        renderer.translate_center(self.snake.get_head().loc, center, 1. / self.snake.get_scale());
        renderer.render_background();
        renderer.render_stage(&self.map);
        renderer.render_snake(&self.snake);
        renderer.render_border(&self.map);
        for apple in &self.apples {
            renderer.render_apple(apple.clone(), cache.food_texture.get_mut(apple.texture.as_str()).unwrap());
        }
        renderer.reset_translation();
        renderer.render_score(self.snake.power, &mut cache.glyph);
    }

    fn game_logic(&mut self) {
        if let Some(mouse) = self.mouse_pressed {
            match mouse {
                MouseButton::Left => { self.snake.power+= 1 }
                MouseButton::Right => if self.snake.power != 0 { self.snake.power-= 1 }
                _=>{}
            }
        }

        if self.tick == 0 {
            self.spawn_apple();
        }

        self.tick = (self.tick + 1) % 300;


        self.snake.move_towards(self.mouse_offset);
        self.snake.border_collision(self.map.width, self.map.height, self.map.border_width);
        self.snake.update_locations();
        self.apple_collision();
    }

}

struct Apple {
    point: Point,
    hp: u32
}

impl Apple {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            hp: 3
        }
    }
}

impl SnakeGameLayer {
    pub fn new(size: Point, food_types: FoodTypes) -> Self {
        SnakeGameLayer {
            mouse_pressed: Option::None,
            mouse_offset: [0., 0.],
            resolution: [0., 0.],
            map: Map::new(size[0], size[1], 6.),
            food_types,
            snake: Snake::new([size[0] / 2., size[1] / 2.], color::LIME),
            apples: Vec::new(),
            tick: 0
        }
    }

    fn spawn_apple(&mut self) {
        use rand::{thread_rng};

        let mut rng = thread_rng();

        let texture = self.food_types.get(rng.gen_range(0 .. self.food_types.len())).unwrap().clone();
        println!("{}", texture);
        let size = rng.gen_range(70. .. 100.);
        let amount = rng.gen_range(4 .. 7);

        let x = rng.gen_range(size .. self.resolution[0] - size);
        let y = rng.gen_range(size .. self.resolution[1] - size);

        self.apples.push(Food {
            texture,
            size,
            amount: amount * 20,
            point: [x, y]
        });
    }

    fn apple_collision(&mut self) {
        let apples = self.apples.clone();

        let snake_loc = self.snake.get_head().loc;
        let snake_size = self.snake.get_size();
        let snake_hit_box = rectangle_by_corners(snake_loc[0], snake_loc[1], snake_loc[0] + snake_size, snake_loc[1] + snake_size);

        for (i, apple) in apples.iter().enumerate() {
            let point = apple.point;
            let apple_hit_box = rectangle_by_corners(point[0], point[1], point[0] + apple.size / 2., point[1] + apple.size / 2.);

            let overlap = overlap_rectangle(snake_hit_box, apple_hit_box);
            if overlap.is_some() {
                self.apples.remove(i);
                self.snake.power += apple.amount;
            }

        }
    }



}