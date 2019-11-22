#![feature(generators, generator_trait)]
extern crate piston;
extern crate ndarray;
extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::Rng;

mod cell;
mod array;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    board: cell::RpsAutomata,
    brush_down: bool,
    current_selection: cell::Color,
    cursor_position: (f64, f64)
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let sq_size = 2;
        let starting_position = (0, 0);

        let mut to_draw = Vec::new();

        for y in 0..self.board.size.1 {
            for x in 0..self.board.size.0 {
                let c = (x, y);
                let el = self.board.board[c];
                let square = rectangle::square(x as f64 * sq_size as f64, y as f64 * sq_size as f64, sq_size as f64);

                let color = match el.color {
                    cell::Color::White => BACKGROUND,
                    cell::Color::Red => RED,
                    cell::Color::Green => GREEN,
                    cell::Color::Blue => BLUE,
                    _ => unreachable!()
                };
                to_draw.push((square, color))
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let transform = c.transform.trans(0.0, 0.0);
            for (square, color) in to_draw {
                rectangle(color, square, transform, gl);
            }

            // Draw a box rotating around the middle of the screen
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.update_board();
        if self.brush_down {
            let x = (self.cursor_position.0 / 2.0) as usize;
            let y = (self.cursor_position.1 / 2.0) as usize;
            let c = (x, y);
            self.board.board[c] = cell::Cell{strength: cell::Cell::max_strength, color: self.current_selection};
        }
    }

    fn update_board(&mut self) {
        self.board.update();
    }


}

fn main() {
    
    let r = cell::RpsAutomata::new(300, 300);


    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut rng = rand::thread_rng();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        board: r,
        current_selection: cell::Color::Red,
        brush_down: false,
        cursor_position: (0.0, 0.0)
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some([x, y]) = e.mouse_cursor_args() {
            app.cursor_position = (x, y);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            app.brush_down = true;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            app.brush_down = false;
        }

        if let Some(Button::Keyboard(Key::R)) = e.press_args() {
            app.current_selection = cell::Color::Red;
        }
        if let Some(Button::Keyboard(Key::G)) = e.press_args() {
            app.current_selection = cell::Color::Green;
        }
        if let Some(Button::Keyboard(Key::B)) = e.press_args() {
            app.current_selection = cell::Color::Blue;
        }
    }
}
