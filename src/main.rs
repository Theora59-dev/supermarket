mod camera;
mod constructor;
mod consts;
mod events;
mod main_loop;
mod player;
mod render;
mod texture;

use constructor::*;
use sdl2::sys::{SDL_Delay, SDL_RendererFlags, Uint32, SDL_HINT_RENDER_VSYNC};
use texture::*;

use sdl2::event::{Event, WindowEvent};
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
        // .present_vsync() // Synchronisation verticale : limité au taux de rafraîchissement de l'écran
        .accelerated() // Accélération matérielle
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

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    let last_fps_size: usize = 5;
    let mut last_fps: Vec<u32> = vec![0; last_fps_size];

    fn avg_fps(fps_list: &mut Vec<u32>, fps_last: u32) -> u32 {
        if fps_list.len() == 5 {
            fps_list.remove(0);
        }
        fps_list.push(fps_last);
        return (fps_list.iter().sum::<u32>() as f32 / fps_list.len() as f32) as u32;
    }

    let fps_cap = 60;
    let dt_cap = 1_f32 / fps_cap as f32;

    // Boucle de jeu
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time);
        last_time = now;
        global_variable.dt = delta_time.as_secs_f32();

        println!(
            "ΔT: {}\nLAST FPS: {}",
            global_variable.dt,
            avg_fps(&mut last_fps, (1_f32 / global_variable.dt) as u32)
        );

        // Effacement de l'écran de jeu
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
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    global_variable.renderer.canvas_size = (width as u32, height as u32);
                    global_variable.renderer.calculate_offset();
                }
                _ => {}
            }
        }

        // Exécution de la boucle de jeu
        main_loop::game_loop(&mut canvas, &mut global_variable);

        // Mise à jour de l'écran de jeu
        canvas.present();

        // Si la synchronisation verticale est activée, on ne cherche pas à limiter les fps car c'est déjà fait automatiquement
        if canvas.info().flags & SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32 != 0 {
            continue;
        }

        // Limitateur de fps

        // Temps écoulé durant l'itération
        let elapsed = now.elapsed().as_secs_f32();

        // Temps d'attente jusqu'à la prochaine itération afin de respecter le limitateur
        //
        // "(elapsed < dt_cap) as u32" permet de remplacer un "if statement" en considérant la condition comme un nombre avec lequel
        // on pourra faire de l'arithmétique. En ce cas, le préprocesseur peut prévoir ce calcul - ce qui n'est pas le cas lors d'un
        // "if statement" où on ne sait pas quel chemin emprunter à moins d'avoir les conditions, ce qui n'est calculé qu'à la dernière
        // minute, optimisant donc les performances
        //
        // "(dt_cap - elapsed) * 1000_f32" représente le calcul du temps d'attente en milliseconde (d'où le * 1000_f32)
        let wait = (elapsed < dt_cap) as u32 * ((dt_cap - elapsed) * 1000_f32) as Uint32;

        println!(
            "\nUNSAFE BLOCK: \nElapsed: {}\nDT CAP:  {}\nWAIT:    {}\nWAIT ms: {}",
            elapsed,
            dt_cap,
            dt_cap - elapsed,
            wait
        );

        unsafe {
            SDL_Delay(wait);
        }
    }
}
