use crate::GlobalVariable;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn game_loop(canvas: &mut Canvas<Window>, construct: &mut GlobalVariable) {
    // Appelle la fonction de déplacement du joueur

    // Affiche les FPS
    println!("FPS: {}", (1.0 / construct.dt) as u32);

    // Dessine tous les objets du monde (contenu dans le groupe de textures)
    construct
        .world
        .draw_all_objects(canvas, (construct.player.x, construct.player.y));
    construct
        .player
        .move_player(construct.dt, &construct.event_pump);

    // Met à jour le joueur par rapport à la caméra
    let camera = &mut construct.camera;
    camera.update(canvas, &construct.player);
}
