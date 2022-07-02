use std::f64::consts::{FRAC_PI_2, PI};
use graphics::{clear, color, Context, Rectangle, rectangle, Transformed};
use graphics::math::overlap_rectangle;
use graphics::rectangle::centered_square;
use opengl_graphics::{GlyphCache, Texture};
use graphics::types::Color;
use opengl_graphics::{GlGraphics};
use crate::{Map, Point, RenderArgs, Snake};
use crate::layer::snake::objects::food::Food;
use crate::util::PointDirection;

pub struct SnakeRenderer<'a> {
    c: Context,
    gl: &'a mut GlGraphics,
    args: RenderArgs
}

impl<'a> SnakeRenderer<'a> {
    pub fn new(c: Context, gl: &'a mut GlGraphics, args: RenderArgs) -> Self {
        SnakeRenderer {
            c,
            gl,
            args
        }
    }

    pub fn translate_center(&mut self, head: Point, center: Point, scale: f64) {
        self.c = self.c.scale(scale, scale).trans(center[0] / scale, center[1] / scale).trans(-head[0], -head[1]);
    }

    pub fn reset_translation(&mut self) {
        self.c = Context::new_viewport(self.c.viewport.unwrap());
    }

    pub fn render_background(&mut self) {
        clear(graphics::color::BLACK, self.gl);
    }

    pub fn render_stage(&mut self, map: &Map) {
        const COLOR: Color = [0.2, 0.2, 0.2, 1.0];
        let rect = Rectangle::new_round(COLOR, 5.);

        rect.draw(
            rectangle::rectangle_by_corners(
                0., 0.,
                map.width, map.height
            ),
            &Default::default(),
            self.c.transform,
            self.gl
        );
    }

    pub fn render_score(&mut self, score: u32, cache: &mut GlyphCache) {
        graphics::text(
            color::WHITE,
            40,
            format!("{}", score).as_str(),
            cache,
            self.c.trans(20., 60.).transform,
            self.gl
        ).unwrap();
    }

    pub fn render_border(&mut self, map: &Map) {
        const COLOR: Color = color::WHITE;
        let rect = Rectangle::new_round_border(COLOR, 5., map.border_width);

        rect.draw(
            rectangle::rectangle_by_corners(
                0., 0.,
                map.width, map.height
            ),
            &Default::default(),
            self.c.transform,
            self.gl
        );
    }


    pub fn render_snake(&mut self, snake: &Snake) {
        let c = self.c;

        let transform = c;
        let size = snake.get_size();
        let length = snake.get_node_length() as f64;

        const RAINBOW: [Color; 8] = [color::RED, [1.0, 0.5, 0., 1.0], color::YELLOW, color::GREEN, [0., 1., 1., 1.], color::BLUE, color::PURPLE, [1.0, 0.5, 0.7, 1.]];

        let head_point = snake.get_head();

        let bodies = snake.get_body();
        let amount = bodies.len();

        let mut i = amount;
        while i > 1 {
            let body = *bodies.get(i - 1).unwrap();

            self.render_snake_part(RAINBOW[(i - 1) % 8], PointDirection::new(body.loc, body.direction), length, size);

            i-= 1;
        }

        self.render_snake_head(RAINBOW[0], PointDirection::new(head_point.loc, head_point.direction), size);
    }

    fn render_snake_head(&mut self, color: Color, point: PointDirection, size: f64) {
        use graphics::*;

        let transform = self.c.trans(point.loc[0], point.loc[1]).rot_rad(point.direction[1].atan2(point.direction[0]) - FRAC_PI_2);

        // TONGUE
        {
            let rect = Rectangle::new_round(color::MAGENTA, size / 8.);

            rect.draw(
                rectangle::rectangle_by_corners(
                    -size / 8., (size / 2.) - 10.,
                    size / 8., size * 0.7
                ),
                &Default::default(),
                transform.transform,
                self.gl
            );
        }

        // HEAD
        {
            ellipse(
                color,
                rectangle::centered_square(0., 0., size / 2.),
                transform.transform,
                self.gl
            );
        }

        // EYES
        {
            let x_eye_offset = size / 4.;
            let y_eye_offset = size / 8.;
            let eye_rad = size / 8.;

            let mut i = 0;
            while i < 2 {
                let mut x_eye_offset = x_eye_offset;

                if i == 0 {
                    x_eye_offset *= -1.;
                }

                ellipse(
                    color::WHITE,
                    rectangle::centered_square(x_eye_offset, y_eye_offset, eye_rad),
                    transform.transform,
                    self.gl
                );

                ellipse(
                    color::BLACK,
                    rectangle::centered_square(x_eye_offset, y_eye_offset, eye_rad / 2.),
                    transform.transform,
                    self.gl
                );

                i += 1;
            }
        }
    }

    fn render_snake_part(&mut self, color: Color, point: PointDirection, length: f64, size: f64) {
        use graphics::*;
        let transform = self.c;

        let angle = point.direction[1].atan2(point.direction[0]) + (PI / 2.);

        let rect = Rectangle::new_round(color, size / 3.);

        rect.draw(
            rectangle::rectangle_by_corners(
                size / 2., 0.,
                size / -2., length
            ),
            &Default::default(),
            transform.trans(point.loc[0], point.loc[1]).rot_rad(angle).transform,
            self.gl
        );
    }

    pub fn render_apple(&mut self, food: Food, texture: &mut Texture) {
        use graphics::*;

        let point = food.point;
        let size = food.size;
        
        image(texture, self.c.trans(point[0] - (size / 2.), point[1] - (size / 2.)).scale(size / 200., size / 200.).transform, self.gl);
    }
}