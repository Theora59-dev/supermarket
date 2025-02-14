use crate::camera::*;
use crate::player::Player;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct GlobalVariable {
    pub dt: f32,               // Déclaration du delta temps
    pub event_pump: EventPump, // Sert à gérer les entrées utilisateurs
    pub player: Player,
    pub camera: Camera,
}

impl GlobalVariable {
    pub fn new(
        sdl_context: Sdl,
        canvas: &Canvas<Window>,
        player_pos: (i32, i32),
    ) -> Result<Self, String> {
        Ok(Self {
            dt: 0.0, // Initialisation de dt à 0.0 pour éviter de futures bugs
            event_pump: sdl_context.event_pump().unwrap(),
            player: Player {
                x: player_pos.0 + 800 / 2,
                y: player_pos.1 + 600 / 2,
                dir_vector: (0, 0), // Vecteur directionnel du joueur
                speed: 5200,        // Vitesse du joueur
            },
            camera: Camera::new(
                player_pos.0,
                player_pos.1,
                canvas.window().size().0,
                canvas.window().size().1,
            ),
        })
    }
}
