use crate::player::Player;
use crate::CustomObject;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct Renderer {
    pub canvas_size: (u32, u32),
    pub player_size: (u32, u32),
    pub camera_position: (i32, i32),
    pub center_offset: (i32, i32),
}

impl Renderer {
    pub fn calculate_offset(&mut self) {
        self.center_offset = (
            (self.canvas_size.0 as i32 - self.camera_position.0 as i32) / 2,
            (self.canvas_size.1 as i32 - self.camera_position.1 as i32) / 2,
        )
    }

    pub fn draw(&mut self, objects: &Vec<CustomObject<'_>>, canvas: &mut Canvas<Window>) {
        for object in objects {
            canvas
                .copy(
                    &object.texture,
                    None,
                    Rect::new(
                        object.x - object.size.0 / 2 + self.center_offset.0, // - offset.0 as i32 + canvas.window().size().0 as i32 / 2, // Position X à offset
                        object.y - object.size.1 / 2 + self.center_offset.1, // - offset.1 as i32 + canvas.window().size().1 as i32 / 2, // Position Y à offset
                        object.size.0 as u32,                                // Largeur de l'objet
                        object.size.1 as u32,                                // Hauteur de l'objet
                    ),
                )
                .unwrap();
        }
    }

    pub fn draw_player(
        &self,
        canvas: &mut Canvas<Window>,
        player: &Player<'_>,
        screen_offset: (i32, i32),
    ) {
        for object in &player.textures.objects {
            canvas
                .copy(
                    &player.textures.textures[0],
                    None,
                    Rect::new(
                        screen_offset.0 - self.player_size.0 as i32 / 2, // Position X à offset
                        screen_offset.1 - self.player_size.1 as i32 / 2, // Position Y à offset
                        object.size.0 as u32,                            // Largeur de l'objet
                        object.size.1 as u32,                            // Hauteur de l'objet
                    ),
                )
                .unwrap();
        }
    }

    pub fn draw_ui(&mut self, objects: Vec<CustomObject>, canvas: &mut Canvas<Window>) {
        for object in objects {
            canvas
                .copy(
                    &object.texture,
                    None,
                    Rect::new(
                        object.x,             // Position X à offset
                        object.y,             // Position Y à offset
                        object.size.0 as u32, // Largeur de l'objet
                        object.size.1 as u32, // Hauteur de l'objet
                    ),
                )
                .unwrap();
        }
    }
}
