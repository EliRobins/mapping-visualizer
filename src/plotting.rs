use macroquad::prelude::*;
use crate::complex::Complex;

pub fn screen_dimensions() -> Vec2 {
    Vec2::new(screen_width() / 2f32, screen_height() / 2f32)
}

pub fn camera_coords(camera: &Camera, coords: &Complex) -> Vec2 {
    Into::<Vec2>::into((*coords - camera.center) / camera.zoom) + screen_dimensions()
}

pub fn graph_coords(camera: &Camera, coords: &Vec2) -> Complex {
    Complex::from(*coords - screen_dimensions()) * camera.zoom + camera.center
}

/*
fn is_onscreen(coords: &Vec2) -> bool {
    (0f32 <= coords.x && coords.x <= screen_width())
        && (0f32 <= coords.y && coords.y <= screen_height())
}
*/

pub struct Camera {
    pub center: Complex,
    pub zoom: f64,
}

impl Camera {
    const ZOOM_FACTOR: f64 = 1.1;
    const DEFAULT_ZOOM_LEVEL: f64 = 5.0;

    pub fn new() -> Self {
        Self {
            center: Complex::default(),
            zoom: 2f64 * Self::DEFAULT_ZOOM_LEVEL / screen_height() as f64,
        }
    }

}

pub struct Mapping {
    pub mapping: Box<dyn Fn(Complex, f64) -> Complex>,
    pub animation: Box<dyn Fn(f32) -> f32>,
    pub color: Color,
}

impl Mapping {
    const CURVE_THICKNESS: f32 = 2.0; // This is how thick the curve will be drawn
    const HALF_CURVE_SEGMENTS: usize = 100;

    #[allow(dead_code)]
    pub fn new(mapping: impl Fn(Complex, f64) -> Complex + 'static, animation: impl Fn(f32) -> f32 + 'static, color: Color) -> Self {
        Self {
            mapping: Box::new(mapping),
            animation: Box::new(animation),
            color: color,
        }
    }

    pub fn plot(&self, camera: &Camera, settings: &Settings) {
        let grid_distance = (*settings.distance_animation)(settings.distance_time) as f64;
        for x in -(settings.grid_size as isize)..=settings.grid_size as isize {
            self.map_line(settings.grid_center + Complex::new(x as f64 * grid_distance, 0f64),
            Complex::new(0f64, 1f64), camera, settings);
        }

        for y in -(settings.grid_size as isize)..=settings.grid_size as isize {
            self.map_line(settings.grid_center + Complex::new(0f64, y as f64 * grid_distance),
            Complex::new(1f64, 0f64), camera, settings);
        }
    }

    fn map_line(&self, center: Complex, direction: Complex, camera: &Camera, settings: &Settings) {
        let map = &self.mapping;
        let grid_distance = (*settings.distance_animation)(settings.distance_time) as f64;
        let scale_factor = grid_distance * settings.grid_size as f64 / Self::HALF_CURVE_SEGMENTS as f64;
        let mut complex_next_point = map(center - direction * Self::HALF_CURVE_SEGMENTS as f64 * scale_factor, (*self.animation)(settings.t) as f64).conjugate();
        let mut next_point = camera_coords(camera, &complex_next_point);
        let mut next_slope = f64::NAN;
        for i in 1-(Self::HALF_CURVE_SEGMENTS as isize)..=Self::HALF_CURVE_SEGMENTS as isize { // The additional 1 is because we already accounted for the first point
            let complex_current_point = complex_next_point;
            let current_point = next_point;
            let current_slope = next_slope;
            complex_next_point = map(center + direction * i as f64 * scale_factor, (*self.animation)(settings.t) as f64).conjugate(); // The conjugate is because of how the camera coordinates work; in the rendering, y increases as you go down, which is not the behavior we want here
            next_point = camera_coords(camera, &complex_next_point);
            next_slope = (complex_next_point - complex_current_point).magnitude() / scale_factor;
            if Self::sufficiently_close(current_slope, next_slope) {
                draw_line(current_point.x, current_point.y, next_point.x, next_point.y, Self::CURVE_THICKNESS, self.color);
            }
        }
    }

    fn sufficiently_close(current_slope: f64, next_slope: f64) -> bool {
        if current_slope.is_nan() || next_slope.is_nan() { return true; }
        f64::abs(next_slope - current_slope) < Settings::EPSILON
    }
}

pub struct Settings {
    pub debug: bool,
    pub t: f32,
    pub distance_time: f32,
    pub grid_center: Complex,
    pub grid_size: usize, // If this is n, the number of lines vertically and horizontally is 2n+1
    pub distance_animation: Box<dyn Fn(f32) -> f32>,
}

impl Settings {
    const TRANSITION_TIME: f32 = 2.0; // Seconds
    const GRID_DISTANCE_TIME: f32 = 2.0; // Seconds
    const EPSILON: f64 = 10.0;
}

pub fn draw_frame(camera: &Camera, mappings: &[Mapping], settings: &Settings) {
    const AXIS_THICKNESS: f32 = 2.0;
    const AXIS_COLOR: Color = WHITE;
    const ENABLE_AXES: bool = false;
    const BACKGROUND_COLOR: Color = BLACK;

    clear_background(BACKGROUND_COLOR);

    if settings.debug {
        let camera_origin = camera_coords(camera, &Complex::new(0f64, 0f64));

        draw_line(camera_origin.x, 0f32, camera_origin.x, screen_height(), AXIS_THICKNESS, AXIS_COLOR);
        draw_line(0f32, camera_origin.y, screen_width(), camera_origin.y, AXIS_THICKNESS, AXIS_COLOR);

        draw_fps();
        draw_text("Mouse position: ".to_string() +
            &graph_coords(camera, &Vec2::from(mouse_position())).conjugate().to_string(),
            2.0, 40.0, 30.0, WHITE);
        draw_text(format!("t: {:.3}", settings.t), 2.0, 65.0, 30.0, WHITE);
        draw_text("Grid center: ".to_string() + &settings.grid_center.to_string(), 2.0, 90.0, 30.0, WHITE);
        draw_text("Grid size: ".to_string() + &settings.grid_size.to_string(), 2.0, 115.0, 30.0, WHITE);
        draw_text("Grid distance: ".to_string() + &format!("{:.3}", (*settings.distance_animation)(settings.distance_time)), 2.0, 140.0, 30.0, WHITE);
    }

    if ENABLE_AXES {
        let camera_origin = camera_coords(camera, &Complex::new(0f64, 0f64));
        draw_line(camera_origin.x, 0f32, camera_origin.x, screen_height(), AXIS_THICKNESS, AXIS_COLOR);
        draw_line(0f32, camera_origin.y, screen_width(), camera_origin.y, AXIS_THICKNESS, AXIS_COLOR);
    }

    for mapping in mappings {
        mapping.plot(camera, settings);
    }
}

pub fn process_input(camera: &mut Camera, settings: &mut Settings) {
    if is_key_pressed(KeyCode::F) { settings.debug = !settings.debug }

    if is_key_down(KeyCode::Left) {
        settings.t = clamp(settings.t - get_frame_time() / Settings::TRANSITION_TIME, 0f32, 1f32);
    }

    if is_key_down(KeyCode::Right) {
        settings.t = clamp(settings.t + get_frame_time() / Settings::TRANSITION_TIME, 0f32, 1f32);
    }

    if is_key_down(KeyCode::LeftBracket) {
        settings.distance_time = clamp(settings.distance_time - get_frame_time() / Settings::GRID_DISTANCE_TIME, 0f32, 1f32);
    }

    if is_key_down(KeyCode::RightBracket) {
        settings.distance_time = clamp(settings.distance_time + get_frame_time() / Settings::GRID_DISTANCE_TIME, 0f32, 1f32);
    }

    if is_key_down(KeyCode::Minus) {
        if settings.grid_size > 0 {
            settings.grid_size -= 1
        }
    }

    if is_key_down(KeyCode::Equal) {
        settings.grid_size += 1
    }

    if is_mouse_button_down(MouseButton::Left) {
        let complex_mouse_delta = Complex::from(mouse_delta_position());
        let complex_screen_dimensions = Complex::from(screen_dimensions());
        camera.center.x += complex_mouse_delta.x * complex_screen_dimensions.x * camera.zoom;
        camera.center.y += complex_mouse_delta.y * complex_screen_dimensions.y * camera.zoom;
    }

    if is_mouse_button_down(MouseButton::Right) {
        let complex_mouse_delta = Complex::from(mouse_delta_position());
        let complex_screen_dimensions = Complex::from(screen_dimensions());
        settings.grid_center.x -= complex_mouse_delta.x * complex_screen_dimensions.x * camera.zoom;
        settings.grid_center.y += complex_mouse_delta.y * complex_screen_dimensions.y * camera.zoom;
    }

    let mouse_scroll = mouse_wheel().1;
    if mouse_scroll != 0f32 {
        let mouse_pos = graph_coords(camera, &Vec2::from(mouse_position()));
        let diff = camera.center - mouse_pos;
        if mouse_scroll > 0f32 {
            camera.zoom /= Camera::ZOOM_FACTOR;
            camera.center = mouse_pos + diff / Camera::ZOOM_FACTOR;
        } else if mouse_scroll < 0f32 {
            camera.zoom *= Camera::ZOOM_FACTOR;
            camera.center = mouse_pos + diff * Camera::ZOOM_FACTOR;
        }
    }

    if is_key_down(KeyCode::H) {
        camera.center = Complex::default();
        camera.zoom = 2f64 * Camera::DEFAULT_ZOOM_LEVEL / screen_height() as f64;
    }
}
