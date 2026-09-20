mod functions;
mod vec2f64;

use macroquad::prelude::*;
use functions::*;
use vec2f64::Vec2f64;

fn screen_dimensions() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

fn camera_coords(camera: &Camera, coords: &Vec2f64) -> Vec2 {
    Into::<Vec2>::into((*coords - camera.center) / camera.zoom) + screen_dimensions()
}

fn camera_coords_y(camera: &Camera, y: f64) -> f32 {
    ((y - camera.center.y) / camera.zoom) as f32 + screen_height() / 2.0
}

fn graph_coords(camera: &Camera, coords: &Vec2) -> Vec2f64 {
    Vec2f64::from(*coords - screen_dimensions()) * camera.zoom + camera.center
}

fn graph_coords_x(camera: &Camera, x: f32) -> f64 {
    f64::from(x - screen_width() / 2.0) * camera.zoom + camera.center.x
}

struct Camera {
    center: Vec2f64,
    zoom: f64,
}

impl Camera {
    const ZOOM_FACTOR: f64 = 1.1;

    fn new() -> Self {
        Self {
            center: Vec2f64::default(),
            zoom: 5.0 / screen_height() as f64,
        }
    }

    fn process_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            self.center += Into::<Vec2f64>::into(mouse_delta_position()) * Vec2f64::from(screen_dimensions()) * self.zoom;
        }

        let mouse_scroll = mouse_wheel().1;
        if mouse_scroll != 0.0 {
            let mouse_pos = graph_coords(self, &Vec2::from(mouse_position()));
            let diff = self.center - mouse_pos;
            if mouse_scroll > 0.0 {
                self.zoom /= Self::ZOOM_FACTOR;
                self.center = mouse_pos + diff / Self::ZOOM_FACTOR;
            } else if mouse_scroll < 0.0 {
                self.zoom *= Self::ZOOM_FACTOR;
                self.center = mouse_pos + diff * Self::ZOOM_FACTOR;
            }
        }
    }
}

enum GeneralFunction {
    Function(Box<dyn Fn(f64) -> f64>),
    //ImplicitFunction(Box<dyn Fn(f64, f64) -> f64>),
}

struct Graph {
    function: GeneralFunction,
    color: Color,
}

impl Graph {
    fn new(function: impl Fn(f64) -> f64 + 'static, color: Color) -> Self {
        Self {
            function: GeneralFunction::Function(Box::new(function)),
            color: color,
        }
    }

    fn graph(&self, camera: &Camera) {
        const GRAPH_THICKNESS: f32 = 2.0;
        match &self.function {
            GeneralFunction::Function(function) => {
                const CURVE_SEGMENTS: u16 = 10000;
                let delta_x = screen_width() / f32::from(CURVE_SEGMENTS);
                let mut next_start = (0.0, camera_coords_y(camera, -function(graph_coords_x(camera, 0.0))));
                for _ in 1..CURVE_SEGMENTS {
                    let start = next_start;
                    next_start = (next_start.0 + delta_x, camera_coords_y(camera, -function(graph_coords_x(camera, next_start.0 + delta_x))));
                    draw_line(start.0, start.1, next_start.0, next_start.1, GRAPH_THICKNESS, self.color);
                }
            }
            //GeneralFunction::ImplicitFunction(function) => {},
        }
    }
}

fn draw_frame(camera: &Camera, graphs: &[Graph], show_fps: bool) {
    const AXIS_THICKNESS: f32 = 2.0;
    const AXIS_COLOR: Color = WHITE;

    clear_background(BLACK);

    if show_fps { draw_fps(); }

    let camera_origin = camera_coords(camera, &Vec2f64::new(0.0, 0.0));

    draw_line(camera_origin.x, 0.0, camera_origin.x, screen_height(), AXIS_THICKNESS, AXIS_COLOR);
    draw_line(0.0, camera_origin.y, screen_width(), camera_origin.y, AXIS_THICKNESS, AXIS_COLOR);

    for graph in graphs {
        graph.graph(camera);
    }
}

#[macroquad::main("Graphing Calculator")]
async fn main() {
    let mut camera = Camera::new();
    let graphs = vec![
        Graph::new(f64::exp, BLUE),
        Graph::new(f64::sin, RED),
        Graph::new(f64::atan, PURPLE),
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
