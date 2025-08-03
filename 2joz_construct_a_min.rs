// 2joz_construct_a_min.rs

// Import necessary libraries
extern crate rand;
extern crate piston2d_opengl_graphics;
extern crate pistoncore_event;
extern crate graphics;

use rand::Rng;
use piston2d_opengl_graphics::{GlGraphics, OpenGL};
use pistoncore_event::{Event, PressEvent};
use graphics::{clear, rectangle, Glyphs, GlyphCount, text};

// Define the size of the window
const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

// Define a struct to hold the data points
struct DataPoint {
    x: f64,
    y: f64,
}

impl DataPoint {
    fn new() -> DataPoint {
        DataPoint {
            x: rand::thread_rng().gen_range(0.0..WIDTH),
            y: rand::thread_rng().gen_range(0.0..HEIGHT),
        }
    }
}

fn main() {
    // Create a window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Minimalist Data Visualization", [WIDTH as u32, HEIGHT as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut gl = GlGraphics::new(opengl);

    // Create a vector to hold the data points
    let mut data_points: Vec<DataPoint> = vec![];

    // Create a font for the labels
    let mut glyphs = Glyphs::new("arial.ttf", window.create_texture_context());

    event_loop(window, gl, data_points, glyphs);
}

fn event_loop(window: PistonWindow, mut gl: GlGraphics, mut data_points: Vec<DataPoint>, mut glyphs: Glyphs) {
    let mut rng = rand::thread_rng();

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear([0.0; 4], gl);

                // Draw the data points
                for point in &data_points {
                    rectangle([1.0; 4], [point.x, point.y, 5.0, 5.0], c.transform, gl);
                }

                // Draw the axis labels
                text::Text::new(16)
                    .draw(
                        "X Axis",
                        &glyphs,
                        &draw_state::DrawState::new(),
                        c.transform.trans(10.0, 10.0),
                        gl,
                    )
                    .unwrap();
                text::Text::new(16)
                    .draw(
                        "Y Axis",
                        &glyphs,
                        &draw_state::DrawState::new(),
                        c.transform.trans(10.0, HEIGHT - 30.0),
                        gl,
                    )
                    .unwrap();
            });
        }

        if let Some/Button::Keyboard(key) = e.press_args() {
            match key {
                Key::Space => {
                    // Generate 100 new data points on space press
                    for _ in 0..100 {
                        data_points.push(DataPoint::new());
                    }
                }
                _ => (),
            }
        }
    }
}