use crate::player::move_player;
use crate::GlobalVariable;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::video::{Window, WindowContext};

pub fn game_loop(
    canvas: &mut Canvas<Window>,
    construct: &mut (
        &mut GlobalVariable,
        (Vec<Texture>, &TextureCreator<WindowContext>),
    ),
) {
    construct.0.i += 1;

    move_player(&mut construct.0); // Déplace le joueur en fonction des inputs
    let texture = &construct.1 .0[0];
    // Dessine la texture sur le canvas par rapport aux changements de coordonnés effectués avec move_player()
    canvas
        .copy(
            &texture,
            None,
            Rect::new(
                construct.0.player.x,
                construct.0.player.y,
                ((texture.query().width) as f32 * 0.1) as u32,
                ((texture.query().height) as f32 * 0.1) as u32,
            ),
        )
        .unwrap();
}
