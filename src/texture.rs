use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::rc::Rc;

pub struct CustomObject<'a> {
    pub x: i32,                   // Position X du personnage
    pub y: i32,                   // Position Y du personnage
    pub z: usize,                 // Ordre d'affichage
    pub size: (i32, i32),         // Taille du personnage
    pub texture: Rc<Texture<'a>>, // Texture du personnage
}

pub struct CustomObjectGroup<'a> {
    pub objects: Vec<CustomObject<'a>>, // Liste de tous les objets
    pub textures: Vec<Rc<Texture<'a>>>, // Liste de toutes les textures
}

impl<'a> CustomObjectGroup<'a> {
    pub fn new() -> Self {
        CustomObjectGroup {
            objects: Vec::new(),  // Initialisation de la liste d'objets
            textures: Vec::new(), // Initialisation de la liste de textures
        }
    }

    pub fn add_object(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        x: i32,
        y: i32,
        z: usize,
        size: (i32, i32),
        texture_from_bytes: &[u8],
    ) {
        let texture = Rc::new(
            texture_creator
                .load_texture_bytes(texture_from_bytes)
                .unwrap(),
        );
        self.textures.push(Rc::clone(&texture)); // Ajoute la texture à la liste de textures

        self.objects.push(CustomObject {
            x,                            // Position X de l'objet
            y,                            // Position Y de l'objet
            z,                            // Ordre d'affichage
            size,                         // taille de l'objet
            texture: Rc::clone(&texture), // Texture de l'objet
        });
    }

    pub fn draw_all_objects(&self, canvas: &mut Canvas<Window>, offset: (i32, i32)) {
        for object in &self.objects {
            canvas
                .copy(
                    &object.texture,
                    None,
                    Rect::new(
                        object.x - offset.0 + canvas.window().size().0 as i32 / 2, // Position X à offset
                        object.y - offset.1 + canvas.window().size().1 as i32 / 2, // Position Y à offset
                        object.size.0 as u32, // Largeur de l'objet
                        object.size.1 as u32, // Hauteur de l'objet
                    ),
                )
                .unwrap();
        }
    }
}
