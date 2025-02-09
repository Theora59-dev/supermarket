use crate::player::move_player;
use crate::Constructor;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn game_loop(canvas: &mut Canvas<Window>, mut construct: &mut Constructor) {
    construct.i += 1;

    let texture_creator = canvas.texture_creator(); // Crée un nouvel objet pour créer des textures
    let texture = texture_creator
        .load_texture("assets/placeholder.png") // Charge une texture depuis un fichier
        .unwrap();

    move_player(&mut construct); // Déplace le joueur en fonction des inputs

    // Dessine la texture sur le canvas par rapport aux changements de coordonnés effectués avec move_player()
    canvas
        .copy(
            &texture,
            None,
            Rect::new(
                construct.player.x,
                construct.player.y,
                ((texture.query().width) as f32 * 0.1) as u32,
                ((texture.query().height) as f32 * 0.1) as u32,
            ),
        )
        .unwrap();
}
