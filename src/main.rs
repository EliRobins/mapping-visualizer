mod mappings;
mod complex;
mod plotting;
mod animate;

use macroquad::prelude::*;
use plotting::*;

#[allow(unused_imports)]
use complex::Complex;

#[allow(unused_imports)]
use animate::*;

#[macroquad::main("Mapping Visualizer")]
async fn main() {
    let mut camera = plotting::Camera::new();
    
    let mappings = vec![
        //Mapping::new(|z, _| z, lerp, RED),
        //Mapping::new(|z, p| Complex::pow(z, Complex::from(1f64 + 1f64 * p)), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(|w| w * w * w - w * w + w * 4f64, z, p), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(|w| Complex::from(1f64) / (Complex::from(1f64) + Complex::exp(-w)), z, p), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(|w| w / (Complex::from(1f64) + w), z, p), ease_in_out, YELLOW),
        //Mapping::new(|z, p| z / (Complex::from(1.0) - z * p), ease_in_out, YELLOW),
        //Mapping::new(|z, p| transition(|w| Complex::pow(w, Complex::from(2f64)), z, p), ease_in_out, YELLOW),
        //Mapping::new(|z, p| transition(Complex::log, z, p), ease_in_out, GREEN),
        //Mapping::new(|z, p| transition(Complex::exp, z, p), ease_in_out, GREEN),
        Mapping::new(|z, p| Complex::exp(z * Complex::from(p)) * Complex::pow(z, Complex::from(1f64 - p)), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(Complex::sin, z, p), ease_in_out, GREEN),
        //Mapping::new(|z, p| transition(Complex::cos, z, p), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(Complex::tan, z, p), ease_in_out, GREEN),
        //Mapping::new(|z, p| transition(|w| Complex::exp(Complex::sin(w)), z, p), ease_in_out, BLUE),
        //Mapping::new(|z, p| transition(mappings::zeta, z, p), ease_in_out, BLUE),
    ];

    let mut settings = Settings {
        debug: false,
        t: 0f32,
        distance_time: 0.3f32,
        grid_center: Complex::new(0f64, 0f64),
        grid_size: 10,
        distance_animation: Box::new(ease_in_out)
    };

    draw_frame(&camera, &mappings, &settings);
    loop {
        if is_key_pressed(KeyCode::Q) { break; }
        process_input(&mut camera, &mut settings);
        draw_frame(&camera, &mappings, &settings);
        next_frame().await;
    }
}
