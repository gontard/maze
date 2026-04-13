mod renderer;

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};

use maze_core::game::{Direction, GameState, GameStatus};
use maze_core::generator::{Kruskal, MazeGenerator, Prim, RecursiveBacktracker};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

fn main() -> io::Result<()> {
    // Set panic hook to restore terminal before unwinding
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), cursor::Show);
        default_hook(info);
    }));

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    let mut level: usize = 1;
    let mut start_pos: Option<(usize, usize)> = None;
    let mut final_status;
    let mut rng = StdRng::from_rng(&mut rand::rng());

    // Level loop: each iteration is one floor of the tower
    loop {
        // Pick a random algorithm for this floor
        let algo_index = rng.random_range(0..3u32);
        let mut maze = match algo_index {
            0 => RecursiveBacktracker.generate(41, 21, None, start_pos),
            1 => Kruskal.generate(41, 21, None, start_pos),
            _ => Prim.generate(41, 21, None, start_pos),
        };
        maze.carve_rooms(3, 3, 5, &mut rng);
        maze.place_exit();

        // Compute max time from solution path length
        let path_length = maze.solve().expect("generated maze must be solvable");
        let max_time_secs = path_length as f64 * 0.375;

        // Fresh game state for this floor
        let mut state = GameState::new_with_max_time(maze.start, max_time_secs);
        let start_time = Instant::now();

        // Initial render
        let elapsed = start_time.elapsed().as_secs_f64();
        renderer::render(&maze, state.player, level, elapsed, max_time_secs)?;

        // Game loop (poll-based for continuous timer updates)
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) = event::read()?
                {
                    let direction = match code {
                        KeyCode::Up | KeyCode::Char('w') => Some(Direction::Up),
                        KeyCode::Down | KeyCode::Char('s') => Some(Direction::Down),
                        KeyCode::Left | KeyCode::Char('a') => Some(Direction::Left),
                        KeyCode::Right | KeyCode::Char('d') => Some(Direction::Right),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            state.quit();
                            None
                        }
                        _ => None,
                    };

                    if let Some(dir) = direction {
                        state.move_player(dir, &maze);
                    }
                }
            }

            let elapsed = start_time.elapsed().as_secs_f64();
            state.check_timeout(elapsed);

            if state.status != GameStatus::Playing {
                break;
            }

            renderer::render(&maze, state.player, level, elapsed, max_time_secs)?;
        }

        final_status = state.status;

        match state.status {
            GameStatus::Won => {
                // Next floor: start where we are (the exit of this maze)
                start_pos = Some(maze.exit);
                level += 1;
            }
            _ => break, // Lost or Quit — end the game
        }
    }

    // Restore terminal
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    // Show result with floor count
    let floors_cleared = level - 1;
    match final_status {
        GameStatus::Lost => {
            println!(
                "Time's up on floor {level}! You cleared {floors_cleared} floor{}.",
                if floors_cleared == 1 { "" } else { "s" }
            );
        }
        _ => {
            println!(
                "Quit on floor {level}. You cleared {floors_cleared} floor{}.",
                if floors_cleared == 1 { "" } else { "s" }
            );
        }
    }

    Ok(())
}
