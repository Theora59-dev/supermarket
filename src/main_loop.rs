use crate::player::move_player;
use crate::texture::*;
use crate::GlobalVariable;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub fn game_loop(
    canvas: &mut Canvas<Window>,
    mut construct: &mut (
        &mut GlobalVariable,
        (
            CustomObjectGroup<'_>,
            &TextureCreator<WindowContext>,
            CustomObjectGroup<'_>,
        ),
    ),
) {
    // Appelle la fonction de déplacement du joueur
    move_player(&mut construct);

    // Affiche les FPS
    println!("{}", (1.0 / construct.0.dt) as u32);

    // Dessine tous les objets du monde (contenu dans le groupe de textures)
    let all_objects_group = &construct.1 .0;
    all_objects_group.draw_all_objects(canvas, (construct.0.player.x, construct.0.player.y));

    // Met à jour le joueur par rapport à la caméra
    let camera = &mut construct.0.camera;
    camera.update(&construct.1 .2, canvas, construct.0.player.clone());
}
