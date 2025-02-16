use crate::CustomObjectGroup;

use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Player<'a> {
    pub x: i32,
    pub y: i32,
    pub dir_vector: (i8, i8),
    pub speed: i32,
    pub textures: CustomObjectGroup<'a>,
}

impl<'a> Player<'a> {
    pub fn move_player(&mut self, dt: f32, event_pump: &EventPump) {
        let keyboard_state = event_pump.keyboard_state();
        self.dir_vector = (0, 0);

        // Ajout au vecteur des mouvements horizontaux. Si les deux touches sont pressées, le mouvement s'annule dans l'équation et le vecteur horizontal est donc nul.
        self.dir_vector.0 = -(keyboard_state.is_scancode_pressed(Scancode::A) as i8)
            + (keyboard_state.is_scancode_pressed(Scancode::D) as i8);

        // Ajout au vecteur des mouvements verticaux. Si les deux touches sont pressées, le mouvement s'annule dans l'équation et le vecteur vertical est donc nul.
        self.dir_vector.1 = -(keyboard_state.is_scancode_pressed(Scancode::W) as i8)
            + (keyboard_state.is_scancode_pressed(Scancode::S) as i8);

        // Normaliser le vecteur directionnel
        let length = ((self.dir_vector.0.pow(2) + self.dir_vector.1.pow(2)) as f32).sqrt();

        if length > 0.0 {
            let normalized_dir = (
                (self.dir_vector.0 as f32 / length) as f32,
                (self.dir_vector.1 as f32 / length) as f32,
            );

            // Mettre à jour les positions du joueur en fonction de la vitesse et du vecteur normalisé
            self.x += (normalized_dir.0 * self.speed as f32 * dt) as i32;
            self.y += (normalized_dir.1 * self.speed as f32 * dt) as i32;
        }
    }

    pub fn draw_player(&self, canvas: &mut Canvas<Window>, screen_offset: (i32, i32)) {
        for object in &self.textures.objects {
            canvas
                .copy(
                    &self.textures.textures[0],
                    None,
                    Rect::new(
                        screen_offset.0,    // Position X à offset
                        screen_offset.1,    // Position Y à offset
                        object.size as u32, // Largeur de l'objet
                        object.size as u32, // Hauteur de l'objet
                    ),
                )
                .unwrap();
        }
    }
}
