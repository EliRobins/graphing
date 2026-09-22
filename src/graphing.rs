use macroquad::prelude::*;
use crate::vec2f64::Vec2f64;

pub fn screen_dimensions() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

pub fn camera_coords(camera: &Camera, coords: &Vec2f64) -> Vec2 {
    Into::<Vec2>::into((*coords - camera.center) / camera.zoom) + screen_dimensions()
}

pub fn camera_coords_y(camera: &Camera, y: f64) -> f32 {
    ((y - camera.center.y) / camera.zoom) as f32 + screen_height() / 2.0
}

pub fn graph_coords(camera: &Camera, coords: &Vec2) -> Vec2f64 {
    Vec2f64::from(*coords - screen_dimensions()) * camera.zoom + camera.center
}

pub fn graph_coords_x(camera: &Camera, x: f32) -> f64 {
    f64::from(x - screen_width() / 2.0) * camera.zoom + camera.center.x
}

fn is_onscreen_y(y: f32) -> bool {
    0f32 <= y && y <= screen_height()
}

pub struct Camera {
    pub center: Vec2f64,
    pub zoom: f64,
}

impl Camera {
    const ZOOM_FACTOR: f64 = 1.1;
    const DEFAULT_ZOOM_LEVEL: f64 = 5.0;

    pub fn new() -> Self {
        Self {
            center: Vec2f64::default(),
            zoom: 2.0 * Self::DEFAULT_ZOOM_LEVEL / screen_height() as f64,
        }
    }

    pub fn process_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            self.center += Into::<Vec2f64>::into(mouse_delta_position()) * Vec2f64::from(screen_dimensions()) * self.zoom;
        }

        let mouse_scroll = mouse_wheel().1;
        if mouse_scroll != 0f32 {
            let mouse_pos = graph_coords(self, &Vec2::from(mouse_position()));
            let diff = self.center - mouse_pos;
            if mouse_scroll > 0f32 {
                self.zoom /= Self::ZOOM_FACTOR;
                self.center = mouse_pos + diff / Self::ZOOM_FACTOR;
            } else if mouse_scroll < 0f32 {
                self.zoom *= Self::ZOOM_FACTOR;
                self.center = mouse_pos + diff * Self::ZOOM_FACTOR;
            }
        }

        if is_key_down(KeyCode::H) {
            self.center = Vec2f64::default();
            self.zoom = 2.0 * Self::DEFAULT_ZOOM_LEVEL / screen_height() as f64;
        }
    }
}

#[allow(dead_code)]
pub enum GeneralFunction {
    Function(Box<dyn Fn(f64) -> f64>),
    ImplicitFunction(Box<dyn Fn(f64, f64) -> f64>),
}

pub struct Graph {
    pub function: GeneralFunction,
    pub color: Color,
}

impl Graph {
    const GRAPH_THICKNESS: f32 = 2.0;

    #[allow(dead_code)]
    pub fn new(function: impl Fn(f64) -> f64 + 'static, color: Color) -> Self {
        Self {
            function: GeneralFunction::Function(Box::new(function)),
            color: color,
        }
    }

    #[allow(dead_code)]
    pub fn new_implicit(function: impl Fn(f64, f64) -> f64 + 'static, color: Color) -> Self {
        Self {
            function: GeneralFunction::ImplicitFunction(Box::new(function)),
            color: color,
        }
    }

    pub fn graph(&self, camera: &Camera) {
        match &self.function {
            GeneralFunction::Function(function) => {
                const CURVE_DENSITY: f32 = 1f32;
                const DELTA_X: f32 = 1f32 / CURVE_DENSITY;
                let curve_segments = (screen_width() * CURVE_DENSITY) as usize;
                let mut next_start = (0f32, camera_coords_y(camera, -function(graph_coords_x(camera, 0f32))));
                for _ in 0..=curve_segments {
                    let start = next_start;
                    next_start = (next_start.0 + DELTA_X, camera_coords_y(camera, -function(graph_coords_x(camera, next_start.0 + DELTA_X))));
                    if is_onscreen_y(start.1) || is_onscreen_y(next_start.1) {
                        draw_line(start.0, start.1, next_start.0, next_start.1, Self::GRAPH_THICKNESS, self.color);
                    }
                }
            }
            GeneralFunction::ImplicitFunction(function) => {
                const GRID_DENSITY: f32 = 0.5;
                const DELTA: f32 = 1f32 / GRID_DENSITY;

                let grid_length_x = (screen_width() * GRID_DENSITY) as usize + 1;
                let grid_length_y = (screen_height() * GRID_DENSITY) as usize + 1;

                let mut grid: Vec<Vec<bool>> = Vec::new();
                
                for i in 0..=grid_length_x {
                    grid.push(Vec::new());
                    for j in 0..=grid_length_y {
                        let coords = graph_coords(camera, &Vec2::from((i as f32 * DELTA, j as f32 * DELTA)));
                        grid[i].push(function(coords.x, -coords.y) > 0f64);
                    }
                }

                for i in 0..grid_length_x {
                    for j in 0..grid_length_y {
                        draw_contour((i, j), &grid, DELTA, self.color);
                    }
                }
            },
        }
    }
}

fn draw_contour(index: (usize, usize), grid: &Vec<Vec<bool>>, delta: f32, color: Color) {
    let (i, j) = index;
    let (i_f, j_f) = (i as f32, j as f32);
    let half_delta = delta / 2f32;

    let (point1, point2);
    if grid[i][j] {
        if grid[i + 1][j] {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    return;
                } else {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                } else {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                }
            }
        } else {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta + half_delta, j_f * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                } else {
                    point1 = (i_f * delta + half_delta, j_f * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                    let point3 = (i_f * delta, j_f * delta + half_delta);
                    let point4 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                    draw_line(point3.0, point3.1, point4.0, point4.0, Graph::GRAPH_THICKNESS, color)
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta + half_delta, j_f * delta);
                    point2 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                } else {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = (i_f * delta + half_delta, j_f * delta);
                }
            }
        }
    } else {
        if grid[i + 1][j] {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = (i_f * delta + half_delta, j_f * delta);
                } else {
                    point1 = (i_f * delta + half_delta, j_f * delta);
                    point2 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                    let point3 = (i_f * delta, j_f * delta + half_delta);
                    let point4 = (i_f * delta + half_delta, j_f * delta);
                    draw_line(point3.0, point3.1, point4.0, point4.0, Graph::GRAPH_THICKNESS, color)
                } else {
                    point1 = (i_f * delta + half_delta, j_f * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                }
            }
        } else {
            if grid[i + 1][j + 1] {
                if grid[i][j + 1] {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                } else {
                    point1 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                    point2 = ((i_f + 1f32) * delta, j_f * delta + half_delta);
                }
            } else {
                if grid[i][j + 1] {
                    point1 = (i_f * delta, j_f * delta + half_delta);
                    point2 = (i_f * delta + half_delta, (j_f + 1f32) * delta);
                } else {
                    return;
                }
            }
        }
    }

    draw_line(point1.0, point1.1, point2.0, point2.1, Graph::GRAPH_THICKNESS, color)
}

pub fn draw_frame(camera: &Camera, graphs: &[Graph], show_fps: bool) {
    const AXIS_THICKNESS: f32 = 2.0;
    const AXIS_COLOR: Color = WHITE;
    const BACKGROUND_COLOR: Color = BLACK;

    clear_background(BACKGROUND_COLOR);

    if show_fps { draw_fps(); }

    let camera_origin = camera_coords(camera, &Vec2f64::new(0f64, 0f64));

    draw_line(camera_origin.x, 0f32, camera_origin.x, screen_height(), AXIS_THICKNESS, AXIS_COLOR);
    draw_line(0f32, camera_origin.y, screen_width(), camera_origin.y, AXIS_THICKNESS, AXIS_COLOR);

    for graph in graphs {
        graph.graph(camera);
    }
}
