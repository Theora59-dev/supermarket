use crate::player::*;
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
    pub fn update(&mut self, canvas: &mut Canvas<Window>, player_data: &Player) {
        (self.w, self.h) = canvas.window().size();

        self.x = player_data.x; // + (self.w / 2) as i32;
        self.y = player_data.y; // + (self.h / 2) as i32;

        //println!("Camera x: {} y: {}", self.x, self.y);

        // Affiche les objets du joueur
        // player_data.draw_player(canvas, ((self.w / 2) as i32, (self.h / 2) as i32));
    }
}
