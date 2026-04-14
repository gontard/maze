mod renderer;

use std::cell::RefCell;
use std::rc::Rc;

use maze_core::floor::generate_floor;
use maze_core::game::{Direction, GameState, GameStatus};
use maze_core::maze::Maze;
use maze_core::render;
use rand::SeedableRng;
use rand::rngs::StdRng;
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
        let (maze, max_time_secs) = generate_floor(&mut rng, start_pos);
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

    fn advance_floor(&mut self) {
        self.start_pos = Some(self.maze.exit);
        self.level += 1;
        let (maze, max_time_secs) = generate_floor(&mut self.rng, self.start_pos);
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

    // Set canvas size using character cell dimensions
    {
        let g = game.borrow();
        canvas.set_width((g.maze.width as f64 * renderer::CELL_WIDTH) as u32);
        canvas.set_height(((g.maze.height + 1) as f64 * renderer::CELL_HEIGHT) as u32);
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
                let Game {
                    ref maze,
                    ref mut state,
                    ..
                } = *g;
                state.move_player(dir, maze);
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
                renderer::paint(&ctx, &cmds);
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
            renderer::paint(&ctx, &cmds);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}
