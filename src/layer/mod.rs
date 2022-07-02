pub mod snake;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonState, RenderArgs};
use crate::{Point, RendererCache};

pub trait Layer {
    fn mouse_click(&mut self, button: Button, button_state: ButtonState);
    fn mouse_update(&mut self, point: Point);
    fn resolution_update(&mut self, x: f64, y: f64);

    fn render(&self, c: Context, gl: &mut GlGraphics, args: RenderArgs, cache: &mut RendererCache);

    fn game_logic(&mut self);
}