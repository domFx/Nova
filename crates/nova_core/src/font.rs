use crate::image::Image;

use rusttype::{point, Scale};

pub struct Font {
    font_data: rusttype::Font<'static>,
}

impl Font {
    pub fn load_from_path(path: &str) -> Self {
        let file = std::fs::read(path).expect("Failed to read font file");

        let font_data = rusttype::Font::try_from_vec(file).expect("Failed to load font");

        Self {
            font_data
        }
    }

    /// Returns a texture atlas with grayscale format - Less memory, but requires a different graphics pipeline and shader
    pub fn get_graysclale_texture_atlas(&self, text: &str, font_size: f32) -> Image {
        let scale = Scale::uniform(font_size as f32);

        let v_metrics = self.font_data.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
    
        let glyphs: Vec<_> = self.font_data.layout(text, scale, offset).collect();
        let width = glyphs.iter()
            .rev()
            .map(|g| g.position().x as u32 + g.unpositioned().h_metrics().advance_width as u32)
            .next()
            .unwrap_or(0);
        
        let height = (v_metrics.ascent - v_metrics.descent) as u32;
    
        let mut bitmap = vec![0u8; (width * height) as usize];
        
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let px = x as i32 + bb.min.x;
                    let py = y as i32 + bb.min.y;
                    if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                        bitmap[(py as u32 * width + px as u32) as usize] = (v * 255.0) as u8;
                    }
                });
            }
        }
    
        let bitmap_len = bitmap.len() as u64;
        Image::new(bitmap, bitmap_len, width as u32, height as u32)
    }
    
    /// Returns a texture atlas with RGBA format - More memory, but can use the same graphics pipeline and shader
    pub fn get_rgba_texture_atlas(&self, text: &str, font_size: f32) -> Image {
        let scale = Scale::uniform(font_size as f32);

        let v_metrics = self.font_data.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
    
        let glyphs: Vec<_> = self.font_data.layout(text, scale, offset).collect();
        let width = glyphs.iter()
            .rev()
            .map(|g| g.position().x as u32 + g.unpositioned().h_metrics().advance_width as u32)
            .next()
            .unwrap_or(0);
        
        let height = (v_metrics.ascent - v_metrics.descent) as u32;
        
        let mut bitmap = vec![0u8; (width * height * 4) as usize];

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let px = x as i32 + bb.min.x;
                    let py = y as i32 + bb.min.y;
                    if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                        let idx = ((py as u32 * width + px as u32) * 4) as usize;
                        let alpha = (v * 255.0) as u8;
                        bitmap[idx] = alpha;
                        bitmap[idx + 1] = alpha;
                        bitmap[idx + 2] = alpha;
                        bitmap[idx + 3] = alpha;
                    }
                });
            }
        }

        let bitmap_len = bitmap.len() as u64;         

        Image::new(bitmap, bitmap_len, width as u32, height as u32)
    }
}
