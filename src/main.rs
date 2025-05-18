use macroquad::prelude::*;

mod bezedit;

use bezedit::*;

fn conf() -> Conf {
    Conf {
        window_title: "Bézier curves".to_string(),
        window_width: 900,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut bez_editor = BezEditor::new();
    loop {
        bez_editor.update();
        clear_background(WHITE);
        bez_editor.draw();
        next_frame().await
    }
}
