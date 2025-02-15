use sdl2::video::{FullscreenType, Window};

pub fn switch_fullscreen_mode(window: &mut Window) {
    // Vérifie l'état actuel de plein écran
    match window.fullscreen_state() {
        FullscreenType::True => {
            // Désactive le plein écran
            window
                .set_fullscreen(FullscreenType::Off)
                .expect("Impossible de désactiver le plein écran")
        }
        FullscreenType::Off => {
            // Active le plein écran
            window
                .set_fullscreen(FullscreenType::True)
                .expect("Impossible d'activer le plein écran")
        }
        _ => println!(""),
    };
    // Affiche l'état actuel de plein écran
    println!(
        "Mode plein écran mis à jour : \"{:#?}\"",
        window.fullscreen_state()
    );
}
