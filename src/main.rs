extern crate sdl2;

mod constructor;
mod events;
mod main_loop;
mod player;

use constructor::*;
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
        .window("Supermarket", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Création de l'écran de jeu
    let mut canvas = window.clone().into_canvas().build().unwrap();

    // Initialisation de la couleur de fond
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Le constructeur contient et doit contenir toutes les variables globales du programme
    let mut constructor = Constructor::new(sdl_context).unwrap();

    // Boucle de jeu
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time);
        last_time = now;
        constructor.dt = delta_time.as_secs_f32();

        // Effacement de l'écran de jeu
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Gestion des événements
        for event in constructor.event_pump.poll_iter() {
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
        println!("Delta Secs: {}", constructor.dt);

        // Exécution de la boucle de jeu
        main_loop::game_loop(&mut canvas, &mut constructor);

        // Mise à jour de l'écran de jeu
        canvas.present();
    }
}
