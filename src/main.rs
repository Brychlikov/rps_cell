extern crate piston;
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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    board: Vec<Vec<u8>>,
    brush_down: bool,
    current_selection: u8,
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

        for (y, row) in self.board.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                let square = rectangle::square(x as f64 * sq_size as f64, y as f64 * sq_size as f64, sq_size as f64);

                let color = match el {
                    0 => BACKGROUND,
                    1 => RED,
                    2 => GREEN,
                    3 => BLUE,
                    _ => unreachable!()
                };
                to_draw.push((square, color))
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

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
            self.board[y][x] = self.current_selection;
        }
    }

    fn update_board(&mut self) {
        let mut new_board = vec![vec![0u8; self.board[0].len()]; self.board.len()];
        let mut rng = rand::thread_rng();
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                let el = self.board[y][x];
                let enemy = el % 3 + 1;

                if x >= 1 {
                    if self.board[y][x - 1] == enemy {
                        new_board[y][x] = enemy;
                        continue;
                    }
                }

                if y >= 1 {
                    if self.board[y - 1][x] == enemy {
                        new_board[y][x] = enemy;
                        continue;
                    }
                }

                if x + 1 < self.board[0].len() {
                    if self.board[y][x + 1] == enemy {
                        new_board[y][x] = enemy;
                        continue;
                    }
                }

                if y + 1 < self.board.len() {
                    if self.board[y + 1][x] == enemy {
                        new_board[y][x] = enemy;
                        continue;
                    }
                }

                new_board[y][x] = el;
            }
        }
        self.board = new_board;
        
    }


}

fn main() {
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
        board: vec![vec![3; 1000]; 1000],
        current_selection: 1,
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
            app.current_selection = 1;
        }
        if let Some(Button::Keyboard(Key::G)) = e.press_args() {
            app.current_selection = 2;
        }
        if let Some(Button::Keyboard(Key::B)) = e.press_args() {
            app.current_selection = 3;
        }
    }
}
