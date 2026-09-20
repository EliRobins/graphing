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
    const DEFAULT_ZOOM_LEVEL: f64 = 5.0;

    fn new() -> Self {
        Self {
            center: Vec2f64::default(),
            zoom: Self::DEFAULT_ZOOM_LEVEL / screen_height() as f64,
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

        if is_key_down(KeyCode::H) {
            self.center = Vec2f64::default();
            self.zoom = Self::DEFAULT_ZOOM_LEVEL / screen_height() as f64;
        }
    }
}

#[allow(dead_code)]
enum GeneralFunction {
    Function(Box<dyn Fn(f64) -> f64>),
    ImplicitFunction(Box<dyn Fn(f64, f64) -> f64>),
}

struct Graph {
    function: GeneralFunction,
    color: Color,
}

impl Graph {
    const GRAPH_THICKNESS: f32 = 2.0;

    #[allow(dead_code)]
    fn new(function: impl Fn(f64) -> f64 + 'static, color: Color) -> Self {
        Self {
            function: GeneralFunction::Function(Box::new(function)),
            color: color,
        }
    }

    #[allow(dead_code)]
    fn new_implicit(function: impl Fn(f64, f64) -> f64 + 'static, color: Color) -> Self {
        Self {
            function: GeneralFunction::ImplicitFunction(Box::new(function)),
            color: color,
        }
    }

    fn graph(&self, camera: &Camera) {
        match &self.function {
            GeneralFunction::Function(function) => {
                const CURVE_SEGMENTS: u16 = 1000;
                let delta_x = screen_width() / f32::from(CURVE_SEGMENTS);
                let mut next_start = (0.0, camera_coords_y(camera, -function(graph_coords_x(camera, 0.0))));
                for _ in 0..CURVE_SEGMENTS {
                    let start = next_start;
                    next_start = (next_start.0 + delta_x, camera_coords_y(camera, -function(graph_coords_x(camera, next_start.0 + delta_x))));
                    draw_line(start.0, start.1, next_start.0, next_start.1, Self::GRAPH_THICKNESS, self.color);
                }
            }
            GeneralFunction::ImplicitFunction(function) => {
                const INITIAL_GRID_LENGTH: usize = 400;

                let delta_x = screen_width() / INITIAL_GRID_LENGTH as f32;
                let delta_y = screen_height() / INITIAL_GRID_LENGTH as f32;

                let mut grid: Vec<Vec<bool>> = Vec::new();
                
                for i in 0..INITIAL_GRID_LENGTH {
                    grid.push(Vec::new());
                    for j in 0..INITIAL_GRID_LENGTH {
                        let coords = graph_coords(camera, &Vec2::from((i as f32 * delta_x, j as f32 * delta_y)));
                        grid[i].push(function(coords.x, -coords.y) > 0.0);
                    }
                }

                for i in 0..INITIAL_GRID_LENGTH-1 {
                    for j in 0..INITIAL_GRID_LENGTH-1 {
                        draw_contour((i, j), &grid, (delta_x, delta_y), self.color);
                    }
                }
            },
        }
    }
}

fn draw_contour(index: (usize, usize), grid: &Vec<Vec<bool>>, dimensions: (f32, f32), color: Color) {
    let (i, j) = index;
    let (i_f, j_f) = (i as f32, j as f32);
    let (delta_x, delta_y) = dimensions;
    let (half_delta_x, half_delta_y) = (delta_x / 2.0, delta_y / 2.0);

    let (point1, point2);
    if grid[i][j] {
        if grid[i + 1][j] {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    return;
                } else {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                } else {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                }
            }
        } else {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                } else {
                    point1 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                    let point3 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    let point4 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                    draw_line(point3.0, point3.1, point4.0, point4.0, Graph::GRAPH_THICKNESS, color)
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    point2 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                } else {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                }
            }
        }
    } else {
        if grid[i + 1][j] {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                } else {
                    point1 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    point2 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                    let point3 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    let point4 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    draw_line(point3.0, point3.1, point4.0, point4.0, Graph::GRAPH_THICKNESS, color)
                } else {
                    point1 = (i_f * delta_x + half_delta_x, j_f * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                }
            }
        } else {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                } else {
                    point1 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                    point2 = ((i_f + 1.0) * delta_x, j_f * delta_y + half_delta_y);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta_x, j_f * delta_y + half_delta_y);
                    point2 = (i_f * delta_x + half_delta_x, (j_f + 1.0) * delta_y);
                } else {
                    return;
                }
            }
        }
    }

    draw_line(point1.0, point1.1, point2.0, point2.1, Graph::GRAPH_THICKNESS, color)
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
        //Graph::new(weierstrass, RED),
        //Graph::new(f64::sin, BLUE),
        //Graph::new_implicit(circle, YELLOW),
        Graph::new_implicit(folium, PURPLE),
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
