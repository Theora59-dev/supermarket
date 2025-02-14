use crate::constructor::GlobalVariable;
use crate::CustomObjectGroup;

use sdl2::keyboard::Scancode;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub dir_vector: (i8, i8),
    pub speed: i32,
}

pub fn move_player(
    constructor: &mut (
        &mut GlobalVariable,
        (
            CustomObjectGroup<'_>,
            &TextureCreator<WindowContext>,
            CustomObjectGroup<'_>,
        ),
    ),
) {
    let keyboard_state = constructor.0.event_pump.keyboard_state();
    constructor.0.player.dir_vector = (0, 0);

    // Ajout au vecteur des mouvements horizontaux. Si les deux touches sont pressées, le mouvement s'annule dans l'équation et le vecteur horizontal est donc nul.
    constructor.0.player.dir_vector.0 = -(keyboard_state.is_scancode_pressed(Scancode::A) as i8)
        + (keyboard_state.is_scancode_pressed(Scancode::D) as i8);

    // Ajout au vecteur des mouvements verticaux. Si les deux touches sont pressées, le mouvement s'annule dans l'équation et le vecteur vertical est donc nul.
    constructor.0.player.dir_vector.1 = -(keyboard_state.is_scancode_pressed(Scancode::W) as i8)
        + (keyboard_state.is_scancode_pressed(Scancode::S) as i8);

    // Normaliser le vecteur directionnel
    let length = ((constructor.0.player.dir_vector.0.pow(2)
        + constructor.0.player.dir_vector.1.pow(2)) as f32)
        .sqrt();

    if length > 0.0 {
        let normalized_dir = (
            (constructor.0.player.dir_vector.0 as f32 / length) as f32,
            (constructor.0.player.dir_vector.1 as f32 / length) as f32,
        );

        // Mettre à jour les positions du joueur en fonction de la vitesse et du vecteur normalisé
        constructor.0.player.x +=
            (normalized_dir.0 * constructor.0.player.speed as f32 * constructor.0.dt) as i32;
        constructor.0.player.y +=
            (normalized_dir.1 * constructor.0.player.speed as f32 * constructor.0.dt) as i32;
    }
}
