use crate::GlobalVariable;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn game_loop(canvas: &mut Canvas<Window>, construct: &mut GlobalVariable) {
    // Appelle la fonction de déplacement du joueur

    // Ancienne méthode // Affiche les FPS
    // println!("FPS: {}", (1.0 / construct.dt) as u32);

    // Ancienne méthode
    // construct.world.draw_all_objects(canvas, (construct.player.x, construct.player.y));
    construct
        .player
        .move_player(construct.dt, &construct.event_pump);

    // Met à jour le joueur par rapport à la caméra
    construct.camera.update(canvas, &construct.player);
    construct.renderer.camera_position = (construct.camera.x, construct.camera.y);

    // Dessine tous les objets du monde (contenu dans le groupe de textures)
    construct.renderer.calculate_offset();
    construct.renderer.draw(&construct.world.objects, canvas);
    construct.renderer.draw_player(
        canvas,
        &construct.player,
        (
            (construct.renderer.canvas_size.0 / 2) as i32,
            (construct.renderer.canvas_size.1 / 2) as i32,
        ),
    );
}
