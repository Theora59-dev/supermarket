use crate::player::Player;
use sdl2::video::Window;
use sdl2::{render::Canvas, EventPump, Sdl};

pub struct GlobalVariable {
    pub dt: f32,               // Déclaration du delta temps
    pub event_pump: EventPump, // Sert à gérer les entrées utilisateurs
    pub i: i32,                // Compteur utilisé pour les tests
    pub player: Player,
}

impl GlobalVariable {
    pub fn new(sdl_context: Sdl, _canvas: &Canvas<Window>) -> Result<Self, String> {
        Ok(Self {
            dt: 0.0, // Initialisation de dt à 0.0 pour éviter de futures bugs
            event_pump: sdl_context.event_pump().unwrap(),
            i: 0,
            player: Player {
                x: 20,
                y: 20,
                dir_vector: (0, 0),
                speed: 900,
            },
        })
    }
}
