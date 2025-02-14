use crate::player::*;
use crate::texture::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Camera {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Camera { x, y, w, h }
    }
    pub fn update(
        &mut self,
        custom_object_group: &CustomObjectGroup,
        canvas: &mut Canvas<Window>,
        player_data: Player,
    ) {
        // Déplacement de la caméra par rapport au joueur
        self.x = player_data.x + (self.w / 2) as i32;
        self.y = player_data.y + (self.h / 2) as i32;

        println!("Player position: ({}, {})", player_data.x, player_data.y);

        custom_object_group.draw_all_objects(canvas, (0, 0)); // Affiche les objets du joueur
    }
}
