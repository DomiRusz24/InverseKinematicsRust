use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use opengl_graphics::{GlyphCache, Texture, TextureSettings};
use crate::layer::snake::objects::food::FoodTypes;

pub struct RendererCache {
    pub glyph: GlyphCache<'static>,
    pub food_texture: HashMap<String, Texture>
}

struct AssetsFolder {
    base: PathBuf,
    settings: TextureSettings
}

impl AssetsFolder {
    fn glyph_cache(&self, file: &str) -> GlyphCache<'static> {
        GlyphCache::new(self.base.join(file), (), TextureSettings::new()).unwrap()
    }

    fn texture(&mut self, file: &str) -> Texture {
        Texture::from_path(self.base.join(file), &self.settings).unwrap()
    }
}

impl RendererCache {
    pub fn get_from_assets() -> Self {

        let mut base = env::current_dir().unwrap();
        base.push("assets");

        let settings = TextureSettings::new();

        let mut assets = AssetsFolder {
            base,
            settings
        };

        let glyph_cache = assets.glyph_cache("Roboto-Medium.ttf");



        let mut food_texture: HashMap<String, Texture> = HashMap::new();

        food_texture.insert("apple".to_string(), assets.texture("apple.png"));
        food_texture.insert("grapes".to_string(), assets.texture("grapes.png"));
        food_texture.insert("banana".to_string(), assets.texture("banana.png"));

        Self {
            glyph: glyph_cache,
            food_texture
        }
    }
}