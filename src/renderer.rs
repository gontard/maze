use std::io::{self, Write};

use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::{cursor, execute};

use crate::maze::{Maze, Tile};

fn tile_char(tile: Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Path => ' ',
        Tile::Start => 'S',
        Tile::Exit => 'E',
    }
}

fn format_timer(elapsed: f64, max_time: f64) -> String {
    let elapsed_min = elapsed as u64 / 60;
    let elapsed_sec = elapsed as u64 % 60;
    let max_min = max_time as u64 / 60;
    let max_sec = max_time as u64 % 60;
    format!(
        "⏱ {:02}:{:02} / {:02}:{:02}",
        elapsed_min, elapsed_sec, max_min, max_sec
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerUrgency {
    Normal,
    Warning,
    Critical,
}

fn timer_urgency(elapsed: f64, max_time: f64) -> TimerUrgency {
    let remaining_pct = (max_time - elapsed) / max_time;
    if remaining_pct < 0.10 {
        TimerUrgency::Critical
    } else if remaining_pct < 0.25 {
        TimerUrgency::Warning
    } else {
        TimerUrgency::Normal
    }
}

fn urgency_color(urgency: TimerUrgency) -> Color {
    match urgency {
        TimerUrgency::Normal => Color::Cyan,
        TimerUrgency::Warning => Color::DarkYellow,
        TimerUrgency::Critical => Color::Red,
    }
}

#[cfg(test)]
fn build_status_bar(maze_width: usize, elapsed: f64, max_time: f64) -> String {
    let timer = format_timer(elapsed, max_time);
    let timer_display_len = timer.chars().count();
    if timer_display_len >= maze_width {
        timer[..maze_width].to_string()
    } else {
        let padding = maze_width - timer_display_len;
        format!("{}{}", " ".repeat(padding), timer)
    }
}

pub fn render(maze: &Maze, player: (usize, usize), elapsed: f64, max_time: f64) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // Render colored status bar
    let timer_text = format_timer(elapsed, max_time);
    let timer_display_len = timer_text.chars().count();
    let padding = if timer_display_len < maze.width {
        maze.width - timer_display_len
    } else {
        0
    };
    let color = urgency_color(timer_urgency(elapsed, max_time));
    execute!(
        stdout,
        Print(" ".repeat(padding)),
        SetForegroundColor(color),
        Print(&timer_text),
        ResetColor,
        Print("\r\n")
    )?;

    // Render maze grid
    let mut buf = String::with_capacity(maze.height * (maze.width + 2));
    for y in 0..maze.height {
        for x in 0..maze.width {
            if (x, y) == player {
                buf.push('@');
            } else {
                buf.push(tile_char(maze.grid[y][x]));
            }
        }
        buf.push_str("\r\n");
    }

    execute!(stdout, Print(&buf))?;
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
pub fn render_to_string(
    maze: &Maze,
    player: (usize, usize),
    elapsed: f64,
    max_time: f64,
) -> String {
    let mut buf = String::new();
    // Status bar
    buf.push_str(&build_status_bar(maze.width, elapsed, max_time));
    buf.push_str("\r\n");
    // Maze grid
    for y in 0..maze.height {
        for x in 0..maze.width {
            if (x, y) == player {
                buf.push('@');
            } else {
                buf.push(tile_char(maze.grid[y][x]));
            }
        }
        buf.push_str("\r\n");
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_maze() -> Maze {
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Start, Tile::Wall],
            vec![Tile::Wall, Tile::Exit, Tile::Wall],
        ];
        Maze {
            grid,
            width: 3,
            height: 3,
            start: (1, 1),
            exit: (1, 2),
        }
    }

    #[test]
    fn rendered_maze_line_width_matches_grid_width() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 0.0, 60.0);
        // Skip status bar (line 0), check maze lines
        for line in output.lines().skip(1) {
            assert_eq!(line.len(), maze.width, "line: {line:?}");
        }
    }

    #[test]
    fn rendered_line_count_includes_status_bar() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 0.0, 60.0);
        assert_eq!(output.lines().count(), maze.height + 1);
    }

    #[test]
    fn player_renders_at_position() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[2], "#@#"); // line 0 is status bar
    }

    #[test]
    fn wall_renders_as_hash() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "###"); // line 0 is status bar
    }

    #[test]
    fn exit_renders_correctly() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[3], "#E#"); // line 0 is status bar
    }

    #[test]
    fn tile_char_mapping() {
        assert_eq!(tile_char(Tile::Wall), '#');
        assert_eq!(tile_char(Tile::Path), ' ');
        assert_eq!(tile_char(Tile::Start), 'S');
        assert_eq!(tile_char(Tile::Exit), 'E');
    }

    #[test]
    fn format_timer_zero() {
        assert_eq!(format_timer(0.0, 60.0), "⏱ 00:00 / 01:00");
    }

    #[test]
    fn format_timer_partial() {
        assert_eq!(format_timer(73.0, 120.0), "⏱ 01:13 / 02:00");
    }

    #[test]
    fn format_timer_elapsed_exceeds_max() {
        // Should clamp display to max
        assert_eq!(format_timer(130.0, 120.0), "⏱ 02:10 / 02:00");
    }

    #[test]
    fn status_bar_is_first_line() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 30.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        // First line is the status bar, maze starts at line 1
        assert!(lines[0].contains("⏱"), "status bar should contain timer");
        assert_eq!(lines.len(), maze.height + 1); // status bar + maze rows
    }

    #[test]
    fn status_bar_right_aligned_to_maze_width() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 0.0, 60.0);
        let status_line = output.lines().next().unwrap();
        assert_eq!(
            status_line.len(),
            maze.width,
            "status bar width should match maze width"
        );
    }

    #[test]
    fn maze_rows_unchanged_after_status_bar() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        // Maze content starts at line 1
        assert_eq!(lines[1], "###");
        assert_eq!(lines[2], "#@#");
        assert_eq!(lines[3], "#E#");
    }

    #[test]
    fn timer_urgency_white() {
        // > 25% remaining
        assert_eq!(timer_urgency(10.0, 100.0), TimerUrgency::Normal);
    }

    #[test]
    fn timer_urgency_yellow() {
        // 10-25% remaining (80% elapsed of 100 = 20% remaining)
        assert_eq!(timer_urgency(80.0, 100.0), TimerUrgency::Warning);
    }

    #[test]
    fn timer_urgency_red() {
        // < 10% remaining (95% elapsed)
        assert_eq!(timer_urgency(95.0, 100.0), TimerUrgency::Critical);
    }
}
