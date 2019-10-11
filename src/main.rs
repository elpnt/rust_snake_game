use piston_window::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game", (WIDTH, HEIGHT)
        )
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_, g, _| {
            clear([1.0, 1.0, 0.5, 1.0], g);
        });
    }
}
