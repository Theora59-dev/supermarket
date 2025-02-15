mod camera;
mod constructor;
mod consts;
mod events;
mod main_loop;
mod player;
mod texture;

use constructor::*;
use texture::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use std::time::Instant;

pub fn main() {
    let mut last_time = Instant::now();

    // Initialisation de la bibliothèque SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Création de la fenêtre du jeu
    let mut window: Window = video_subsystem
        .window("Supermarket", 700, 700)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Création de l'écran de jeu
    let mut canvas = window
        .clone()
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();

    // Initialisation de la couleur de fond
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Création un nouvel objet pour créer des textures
    let texture_creator = canvas.texture_creator();

    let player_coord = (
        (canvas.window().size().0 / 2) as i32,
        (canvas.window().size().1 / 2) as i32,
    );

    // Le constructeur contient et doit contenir toutes les variables globales du programme
    let mut global_variable =
        GlobalVariable::new(sdl_context, &canvas, player_coord, &texture_creator).unwrap();

    // Boucle de jeu
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time);
        last_time = now;
        global_variable.dt = delta_time.as_secs_f32();

        // Effacement de l'écran de jeu
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Gestion des événements
        for event in global_variable.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                } => events::switch_fullscreen_mode(&mut window),
                _ => {}
            }
        }

        // Exécution de la boucle de jeu
        main_loop::game_loop(&mut canvas, &mut global_variable);

        // Mise à jour de l'écran de jeu
        canvas.present();
    }
}
