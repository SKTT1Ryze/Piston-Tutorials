//! Piston-Tutorial: getting-start
//! 
//! 
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod config;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    back_color: [f32; 4],   // Color for the background
    cube_color: [f32; 4],   // Color for the cube
    current_color: usize,  // Current color index
    timer: usize,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let backcolor = self.back_color;
        let cubecolor = self.cube_color;

        self.gl.draw(args.viewport(), |c, gl| {
            
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            
            // Clear the screen.
            clear(backcolor, gl);

            // Draw a box rotating around the middle of the screen.
            rectangle(cubecolor, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        if self.timer >= config::MAX_TICK {
            match self.current_color {
                0 => {
                    self.cube_color = config::COLOR_LIST[config::COLOR_NUM - 1];
                },
                _ => {
                    self.cube_color = config::COLOR_LIST[self.current_color - 1];
                }
            }
            if self.current_color < config::COLOR_NUM - 1 {
                self.back_color = config::COLOR_LIST[self.current_color + 1];
                self.current_color += 1;
            }
            else {
                self.back_color = config::COLOR_LIST[0];
                self.current_color = 0;
            }
            self.timer = 0;
        }
        self.timer += 1;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        back_color: config::BLUE,
        cube_color: config::RED,
        current_color: 4,
        timer: 0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
