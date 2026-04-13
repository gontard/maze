mod renderer;

use std::cell::RefCell;
use std::rc::Rc;

use maze_core::game::{Direction, GameState, GameStatus};
use maze_core::generator::{Kruskal, MazeGenerator, Prim, RecursiveBacktracker};
use maze_core::maze::Maze;
use maze_core::render;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use wasm_bindgen::prelude::*;

struct Game {
    maze: Maze,
    state: GameState,
    level: usize,
    max_time_secs: f64,
    start_time: f64,
    start_pos: Option<(usize, usize)>,
    rng: StdRng,
}

impl Game {
    fn new() -> Self {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let start_pos = None;
        let level = 1;
        let (maze, max_time_secs) = Self::generate_floor(&mut rng, start_pos);
        let state = GameState::new_with_max_time(maze.start, max_time_secs);
        let start_time = performance_now();

        Self {
            maze,
            state,
            level,
            max_time_secs,
            start_time,
            start_pos,
            rng,
        }
    }

    fn generate_floor(rng: &mut StdRng, start_pos: Option<(usize, usize)>) -> (Maze, f64) {
        let algo_index = rng.random_range(0..3u32);
        let mut maze = match algo_index {
            0 => RecursiveBacktracker.generate(41, 21, None, start_pos),
            1 => Kruskal.generate(41, 21, None, start_pos),
            _ => Prim.generate(41, 21, None, start_pos),
        };
        maze.carve_rooms(3, 3, 5, rng);
        maze.place_exit();

        let path_length = maze.solve().expect("generated maze must be solvable");
        let max_time_secs = path_length as f64 * 0.375;
        (maze, max_time_secs)
    }

    fn advance_floor(&mut self) {
        self.start_pos = Some(self.maze.exit);
        self.level += 1;
        let (maze, max_time_secs) = Self::generate_floor(&mut self.rng, self.start_pos);
        self.maze = maze;
        self.max_time_secs = max_time_secs;
        self.state = GameState::new_with_max_time(self.maze.start, max_time_secs);
        self.start_time = performance_now();
    }

    fn elapsed(&self) -> f64 {
        (performance_now() - self.start_time) / 1000.0
    }
}

fn performance_now() -> f64 {
    web_sys::window()
        .expect("no window")
        .performance()
        .expect("no performance")
        .now()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");

    let canvas = document
        .get_element_by_id("maze-canvas")
        .expect("no #maze-canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .expect("no 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let game = Rc::new(RefCell::new(Game::new()));

    // Set canvas size
    {
        let g = game.borrow();
        let tile_size = 16;
        canvas.set_width((g.maze.width * tile_size) as u32);
        canvas.set_height(((g.maze.height + 1) * tile_size) as u32); // +1 for status bar
    }

    // Keyboard handler
    {
        let game = Rc::clone(&game);
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut g = game.borrow_mut();
            if g.state.status != GameStatus::Playing {
                return;
            }

            let direction = match event.key().as_str() {
                "ArrowUp" | "w" | "W" => Some(Direction::Up),
                "ArrowDown" | "s" | "S" => Some(Direction::Down),
                "ArrowLeft" | "a" | "A" => Some(Direction::Left),
                "ArrowRight" | "d" | "D" => Some(Direction::Right),
                "Escape" => {
                    g.state.quit();
                    None
                }
                _ => None,
            };

            if let Some(dir) = direction {
                event.prevent_default();
                let maze = &g.maze as *const Maze;
                // SAFETY: maze is not mutated during move_player
                unsafe { g.state.move_player(dir, &*maze) };
            }
        });

        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Animation loop
    {
        let game = Rc::clone(&game);
        let ctx = ctx.clone();
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut gm = game.borrow_mut();
            let elapsed = gm.elapsed();

            gm.state.check_timeout(elapsed);

            if gm.state.status == GameStatus::Won {
                gm.advance_floor();
                // Continue loop with new floor
            } else if gm.state.status != GameStatus::Playing {
                // Game over — render final frame and stop
                let cmds = render::render_frame(
                    &gm.maze,
                    gm.state.player,
                    gm.level,
                    elapsed,
                    gm.max_time_secs,
                );
                renderer::paint(&ctx, &cmds, 16);
                return;
            }

            let elapsed = gm.elapsed();
            let cmds = render::render_frame(
                &gm.maze,
                gm.state.player,
                gm.level,
                elapsed,
                gm.max_time_secs,
            );
            renderer::paint(&ctx, &cmds, 16);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}
