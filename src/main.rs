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
use std::borrow::Borrow;
use piston::input::keyboard::Key::Colon;

mod cell;
use cell::{Cell, Color};

mod array;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    board: cell::RpsAutomata,
    brush_down: bool,
    current_selection: cell::Color,
    cursor_position: (f64, f64)
}

pub struct CellView {
    gl: GlGraphics,
    current_cell_size: f64,
    current_offset: (f64, f64),
}

impl CellView{
    fn new(gl: GlGraphics) -> Self {
        Self {
            gl, current_cell_size: 1.0, current_offset: (0.0, 0.0)
        }
    }

    fn render(&mut self, board: &cell::RpsAutomata, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let (win_x, win_y) = (args.window_size[0], args.window_size[1]);
        let smaller = if win_x < win_y {win_x} else {win_y};
        let mut sq_size = (smaller / board.size.0 as f64) as usize;
        if sq_size == 0 {
            sq_size = 1;
        }
        self.current_cell_size = sq_size as f64;


        let starting_position = ((win_x - sq_size as f64 * board.size.0 as f64) / 2.0,
                                (win_y - sq_size as f64 * board.size.1 as f64) / 2.0);
        self.current_offset = starting_position;

        let mut to_draw = Vec::new();

        for y in 0..board.size.1 {
            for x in 0..board.size.0 {
                let c = (x, y);
                let el = board.board[c];
                let square = rectangle::square(x as f64 * sq_size as f64 + starting_position.0, y as f64 * sq_size as f64 + starting_position.1, sq_size as f64);

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

        });
    }
}

struct BoardConroller {
    board: cell::RpsAutomata,
    boardview: CellView,
    brush_down: bool,
    current_selection: cell::Color,
    cursor_position: (f64, f64)
}


impl BoardConroller {

    fn new(board: cell::RpsAutomata, gl: GlGraphics) -> Self {
        Self {
            board,
            boardview: CellView::new(gl),
            brush_down: false,
            current_selection: cell::Color::Red,
            cursor_position: (0.0, 0.0)
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let cursor_x = self.cursor_position.0 - self.boardview.current_offset.0;
        let cursor_y = self.cursor_position.1 - self.boardview.current_offset.1;
        if self.brush_down {
            let x = (cursor_x / self.boardview.current_cell_size) as usize;
            let y = (cursor_y / self.boardview.current_cell_size) as usize;
            let c = (x, y);
            self.board.board[c] = cell::Cell{strength: cell::Cell::max_strength, color: self.current_selection};
        }
    }

    fn parse_event(&mut self, e: &Event) {
        if let Some(r) = e.render_args() {
            self.boardview.render(&self.board, &r);
            self.board.update();
        }

        if let Some(u) = e.update_args() {
            self.update(&u);
        }

        if let Some([x, y]) = e.mouse_cursor_args() {
            self.cursor_position = (x, y);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.brush_down = true;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            self.brush_down = false;
        }

        if let Some(Button::Keyboard(Key::R)) = e.press_args() {
            self.current_selection = cell::Color::Red;
        }
        if let Some(Button::Keyboard(Key::G)) = e.press_args() {
            self.current_selection = cell::Color::Green;
        }
        if let Some(Button::Keyboard(Key::B)) = e.press_args() {
            self.current_selection = cell::Color::Blue;
        }
    }
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
        self.update_board();
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
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
    
    let mut r = cell::RpsAutomata::new(300, 300);
    r.board[(0, 0)] = Cell{strength: 100, color: Color::Red};
//    r.board[(1, 1)] = Cell{strength: 100, color: Color::Green};


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
    let mut bc = BoardConroller::new(r, GlGraphics::new(opengl));
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     rotation: 0.0,
    //     board: r,
    //     current_selection: cell::Color::Red,
    //     brush_down: false,
    //     cursor_position: (0.0, 0.0)
    // };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        bc.parse_event(&e);
    }
}
