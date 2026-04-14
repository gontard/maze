use std::io::{self, Write};

use crossterm::style::{Color as CtColor, Print, ResetColor, SetForegroundColor};
use crossterm::{cursor, execute};

use maze_core::maze::Maze;
use maze_core::render::{self, Color, DrawCommand};

fn to_crossterm_color(color: Color) -> CtColor {
    match color {
        Color::White => CtColor::White,
        Color::Black => CtColor::Black,
        Color::Green => CtColor::Green,
        Color::Cyan => CtColor::Cyan,
        Color::Yellow => CtColor::DarkYellow,
        Color::Red => CtColor::Red,
    }
}

pub fn render(
    maze: &Maze,
    player: (usize, usize),
    level: usize,
    elapsed: f64,
    max_time: f64,
) -> io::Result<()> {
    let cmds = render::render_frame(maze, player, level, elapsed, max_time);
    let mut stdout = io::stdout();

    // Collect grid chars for the maze area (rows 1..=maze.height)
    let mut grid: Vec<Vec<char>> = vec![vec![' '; maze.width]; maze.height];

    // Status bar pieces
    let mut floor_label = String::new();
    let mut timer_text = String::new();
    let mut timer_color = CtColor::Cyan;

    for cmd in &cmds {
        match cmd {
            DrawCommand::Clear => {
                execute!(stdout, cursor::MoveTo(0, 0))?;
            }
            DrawCommand::DrawText { x, y, text, color } => {
                if *y == 0 {
                    if *x == 0 {
                        floor_label = text.clone();
                    } else {
                        timer_text = text.clone();
                        timer_color = to_crossterm_color(*color);
                    }
                }
            }
            DrawCommand::DrawChar { x, y, ch, .. } => {
                if *y >= 1 && *y <= maze.height {
                    let row = *y - 1;
                    if *x < maze.width {
                        grid[row][*x] = *ch;
                    }
                }
            }
        }
    }

    // Render status bar
    let floor_len = floor_label.len();
    let timer_len = timer_text.chars().count();
    let total = floor_len + timer_len;
    let padding = if total < maze.width {
        maze.width - total
    } else {
        0
    };
    execute!(
        stdout,
        Print(&floor_label),
        Print(" ".repeat(padding)),
        SetForegroundColor(timer_color),
        Print(&timer_text),
        ResetColor,
        Print("\r\n")
    )?;

    // Render maze grid
    let mut buf = String::with_capacity(maze.height * (maze.width + 2));
    for row in &grid {
        for &ch in row {
            buf.push(ch);
        }
        buf.push_str("\r\n");
    }
    execute!(stdout, Print(&buf))?;
    stdout.flush()?;

    Ok(())
}
