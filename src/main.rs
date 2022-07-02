pub mod util;
mod renderer;
mod layer;

use std::env;
use glutin_window::{GlutinWindow};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::{RenderArgs, RenderEvent};
use piston::event_loop::{EventSettings, Events};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, MouseCursorEvent};
use piston::window::WindowSettings;

use layer::snake::objects::map::Map;

use layer::snake::objects::snake::Snake;
use crate::layer::Layer;
use layer::snake::logic::SnakeGameLayer;
use crate::renderer::RendererCache;
use crate::util::{Point};

struct Game {
    gl: GlGraphics,
    layers: Vec<Box<dyn Layer>>
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        Game {
            gl,
            layers: Vec::new()
        }
    }

    fn mouse_update(&mut self, point: Point) {
        for layer in &mut self.layers {
            layer.mouse_update(point);
        }
    }

    fn mouse_click(&mut self, button: Button, button_state: ButtonState) {
        for layer in &mut self.layers {
            layer.mouse_click(button, button_state);
        }
    }

    fn resolution_update(&mut self, res: Point) {
        for layer in &mut self.layers {
            layer.resolution_update(res[0], res[1]);
        }
    }

    fn render(&mut self, args: &RenderArgs, cache: &mut RendererCache) {
        self.resolution_update(args.window_size);
        self.game_logic();

        let layers = &mut self.layers;

        self.gl.draw(args.viewport(), |c, gl| {

            for layer in layers {
                layer.render(c, gl, *args, cache);
            }

        });
    }

    fn game_logic(&mut self) {
        for layer in &mut self.layers {
            layer.game_logic();
        }
    }
}

fn main() {

    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("Snake", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl));

    let mut cache = RendererCache::get_from_assets();



    game.layers.push(Box::new(SnakeGameLayer::new(
        [1500., 1500.],
        vec!["apple".to_string(), "grapes".to_string(), "banana".to_string()]
    )));

    let mut settings = EventSettings::new();
    settings.set_max_fps(60);

    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| {
            game.mouse_update([pos[0], pos[1]])
        });

        if let Some(args) = e.button_args() {
            game.mouse_click(args.button, args.state)
        }

        if let Some(args) = e.render_args() {
            game.render(&args, &mut cache);
        }

    }





}
