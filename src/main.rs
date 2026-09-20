mod functions;
mod vec2f64;
mod graphing;

use macroquad::prelude::*;
use functions::*;
use graphing::*;

#[macroquad::main("Graphing Calculator")]
async fn main() {
    let mut camera = graphing::Camera::new();
    let graphs = vec![
        Graph::new(weierstrass, RED),
        Graph::new(f64::sin, BLUE),
        Graph::new(square_wave, GREEN),
        Graph::new(|x| 1.0 / x, PINK),
        //Graph::new_implicit(circle, YELLOW),
        //Graph::new_implicit(folium, PURPLE),
    ];

    let mut show_fps = false;

    draw_frame(&camera, &graphs, show_fps);
    loop {
        if is_key_pressed(KeyCode::Q) { break; }
        if is_key_pressed(KeyCode::F) { show_fps = !show_fps }
        camera.process_input();
        draw_frame(&camera, &graphs, show_fps);
        next_frame().await;
    }
}
