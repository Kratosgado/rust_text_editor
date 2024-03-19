mod editor_handlers;
mod imput_handlers;
mod  text_handlers;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Editor", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event ) = window.next() {
        window.draw_2d(&event, |context, gfx, _device|{
            clear([1.0; 4], gfx);
        });
    }
}
