use macroquad::prelude::*;

#[macroquad::main("Macroquad hello world")]
async fn main() {
    loop {
        clear_background(GREEN);

        draw_text("Hello, world", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 52.0, DARKGRAY);
        next_frame().await
    }
}