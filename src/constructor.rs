use crate::camera::*;
use crate::player::Player;
use crate::texture::CustomObjectGroup;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

use crate::consts::*;
use crate::render::Renderer;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct GlobalVariable<'a> {
    pub dt: f32,               // Déclaration du delta temps
    pub event_pump: EventPump, // Sert à gérer les entrées utilisateurs
    pub player: Player<'a>,
    pub camera: Camera,
    pub _texture_creator: &'a TextureCreator<WindowContext>,
    pub world: CustomObjectGroup<'a>,
    pub renderer: Renderer,
}

impl<'a> GlobalVariable<'a> {
    pub fn new(
        sdl_context: Sdl,
        canvas: &Canvas<Window>,
        player_pos: (i32, i32),
        main_texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let mut player_textures = CustomObjectGroup::new();
        player_textures.add_object(
            main_texture_creator,
            player_pos.0,
            player_pos.1,
            1,
            (50, 50),
            TEXTURE,
        );

        let mut world_textures = CustomObjectGroup::new();
        world_textures.add_object(&main_texture_creator, 0, 0, 0, (5000, 5000), MAP);

        Ok(Self {
            dt: 0.0, // Initialisation de dt à 0.0 pour éviter de futures bugs
            event_pump: sdl_context.event_pump().unwrap(),
            _texture_creator: main_texture_creator,
            world: world_textures,
            player: Player {
                x: player_pos.0,
                y: player_pos.1,
                dir_vector: (0, 0), // Vecteur directionnel du joueur
                speed: 5200,        // Vitesse du joueur
                textures: player_textures,
            },
            camera: Camera::new(
                player_pos.0,
                player_pos.1,
                canvas.window().size().0,
                canvas.window().size().1,
            ),
            renderer: Renderer {
                canvas_size: (canvas.window().size().0, canvas.window().size().1),
                player_size: (50, 50),
                camera_position: (0, 0),
                center_offset: (0, 0),
            },
        })
    }
}
