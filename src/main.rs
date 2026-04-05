mod game;
mod generator;
mod maze;
mod renderer;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};

use game::{Direction, GameState, GameStatus};
use generator::{MazeGenerator, RecursiveBacktracker};

fn main() -> io::Result<()> {
    // Set panic hook to restore terminal before unwinding
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), cursor::Show);
        default_hook(info);
    }));

    // Generate maze
    let generator = RecursiveBacktracker;
    let maze = generator.generate(41, 21, None);

    // Compute max time from solution path length
    let path_length = maze.solve().expect("generated maze must be solvable");
    let max_time_secs = path_length as f64 * 0.375;

    // Init game state
    let mut state = GameState::new_with_max_time(maze.start, max_time_secs);

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Initial render
    renderer::render(&maze, state.player, state.elapsed_secs(), max_time_secs)?;

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

        state.check_timeout();

        if state.status != GameStatus::Playing {
            break;
        }

        renderer::render(&maze, state.player, state.elapsed_secs(), max_time_secs)?;
    }

    // Restore terminal
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    // Show result
    match state.status {
        GameStatus::Won => {
            println!(
                "You escaped the maze in {:.1} seconds!",
                state.elapsed_secs()
            );
        }
        GameStatus::Lost => {
            println!("Time's up! You ran out of time.");
        }
        _ => {
            println!("Quit. Better luck next time!");
        }
    }

    Ok(())
}
