use crate::constructor::Constructor;

use sdl2::keyboard::Scancode;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub dir_vector: (i8, i8),
    pub speed: i32,
}

pub fn move_player(constructor: &mut Constructor) {
    let keyboard_state = constructor.event_pump.keyboard_state();
    constructor.player.dir_vector = (0, 0);

    // Calculer la direction horizontale
    if keyboard_state.is_scancode_pressed(Scancode::A)
        && keyboard_state.is_scancode_pressed(Scancode::D)
    {
        // Les deux touches "A" et "D" sont enfoncées, donc la direction horizontale est nulle
        constructor.player.dir_vector.0 = 0
    } else {
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            // La touche "A" est enfoncée, donc la direction horizontale est vers la gauche
            constructor.player.dir_vector.0 = -1;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            // La touche "D" est enfoncée, donc la direction horizontale est vers la droite
            constructor.player.dir_vector.0 = 1;
        }
    }

    // Calculer la direction verticale
    if keyboard_state.is_scancode_pressed(Scancode::W)
        && keyboard_state.is_scancode_pressed(Scancode::S)
    {
        // Les deux touches "W" et "S" sont enfoncées, donc la direction verticale est nulle
        constructor.player.dir_vector.1 = 0
    } else {
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            // La touche "W" est enfoncée, donc la direction verticale est vers le haut
            constructor.player.dir_vector.1 = -1;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            // La touche "S" est enfoncée, donc la direction verticale est vers le bas
            constructor.player.dir_vector.1 = 1;
        }
    }

    // Normaliser le vecteur directionnel
    let length = ((constructor.player.dir_vector.0.pow(2) + constructor.player.dir_vector.1.pow(2))
        as f32)
        .sqrt();
    if length > 0.0 {
        let normalized_dir = (
            (constructor.player.dir_vector.0 as f32 / length) as f32,
            (constructor.player.dir_vector.1 as f32 / length) as f32,
        );

        // Mettre à jour les positions du joueur en fonction de la vitesse et du vecteur normalisé
        constructor.player.x +=
            (normalized_dir.0 * constructor.player.speed as f32 * constructor.dt) as i32;
        constructor.player.y +=
            (normalized_dir.1 * constructor.player.speed as f32 * constructor.dt) as i32;
    }
}
