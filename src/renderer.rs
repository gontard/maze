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
fn build_status_bar(maze_width: usize, level: usize, elapsed: f64, max_time: f64) -> String {
    let floor_label = format!("Floor {level}");
    let timer = format_timer(elapsed, max_time);
    let floor_len = floor_label.len();
    let timer_len = timer.chars().count();
    let total = floor_len + timer_len;
    if total >= maze_width {
        format!("{floor_label}{timer}")
    } else {
        let padding = maze_width - total;
        format!("{floor_label}{}{timer}", " ".repeat(padding))
    }
}

pub fn render(
    maze: &Maze,
    player: (usize, usize),
    level: usize,
    elapsed: f64,
    max_time: f64,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // Render status bar: "Floor N" left-aligned, timer right-aligned
    let floor_label = format!("Floor {level}");
    let timer_text = format_timer(elapsed, max_time);
    let floor_len = floor_label.len();
    let timer_len = timer_text.chars().count();
    let total = floor_len + timer_len;
    let padding = if total < maze.width {
        maze.width - total
    } else {
        0
    };
    let color = urgency_color(timer_urgency(elapsed, max_time));
    execute!(
        stdout,
        Print(&floor_label),
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
    level: usize,
    elapsed: f64,
    max_time: f64,
) -> String {
    let mut buf = String::new();
    // Status bar
    buf.push_str(&build_status_bar(maze.width, level, elapsed, max_time));
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

    fn wide_maze() -> Maze {
        // 41-wide maze (like the real game) but only 3 rows tall
        let row_wall = vec![Tile::Wall; 41];
        let mut row_mid = vec![Tile::Wall; 41];
        row_mid[1] = Tile::Start;
        row_mid[39] = Tile::Exit;
        for i in 2..39 {
            row_mid[i] = Tile::Path;
        }
        let grid = vec![row_wall.clone(), row_mid, row_wall];
        Maze {
            grid,
            width: 41,
            height: 3,
            start: (1, 1),
            exit: (39, 1),
        }
    }

    #[test]
    fn rendered_maze_line_width_matches_grid_width() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        // Skip status bar (line 0), check maze lines
        for line in output.lines().skip(1) {
            assert_eq!(line.len(), maze.width, "line: {line:?}");
        }
    }

    #[test]
    fn rendered_line_count_includes_status_bar() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        assert_eq!(output.lines().count(), maze.height + 1);
    }

    #[test]
    fn player_renders_at_position() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 1, 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[2], "#@#"); // line 0 is status bar
    }

    #[test]
    fn wall_renders_as_hash() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 1, 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "###"); // line 0 is status bar
    }

    #[test]
    fn exit_renders_correctly() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1), 1, 0.0, 60.0);
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
        assert_eq!(format_timer(130.0, 120.0), "⏱ 02:10 / 02:00");
    }

    #[test]
    fn status_bar_contains_floor_and_timer() {
        let maze = wide_maze();
        let output = render_to_string(&maze, maze.start, 3, 30.0, 60.0);
        let status_line = output.lines().next().unwrap();
        assert!(
            status_line.starts_with("Floor 3"),
            "status bar should start with floor label"
        );
        assert!(status_line.contains("⏱"), "status bar should contain timer");
    }

    #[test]
    fn status_bar_floor_left_timer_right() {
        let maze = wide_maze();
        let output = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        let status_line = output.lines().next().unwrap();
        let floor_pos = status_line.find("Floor").unwrap();
        let timer_pos = status_line.find("⏱").unwrap();
        assert_eq!(floor_pos, 0, "floor label should be at the start");
        assert!(timer_pos > floor_pos, "timer should be after floor label");
    }

    #[test]
    fn status_bar_width_matches_maze_width() {
        let maze = wide_maze();
        let output = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        let status_line = output.lines().next().unwrap();
        assert_eq!(
            status_line.chars().count(),
            maze.width,
            "status bar char count should match maze width"
        );
    }

    #[test]
    fn status_bar_floor_number_updates() {
        let maze = wide_maze();
        let output1 = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        let output7 = render_to_string(&maze, maze.start, 7, 0.0, 60.0);
        let line1 = output1.lines().next().unwrap();
        let line7 = output7.lines().next().unwrap();
        assert!(line1.starts_with("Floor 1"));
        assert!(line7.starts_with("Floor 7"));
    }

    #[test]
    fn maze_rows_unchanged_after_status_bar() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start, 1, 0.0, 60.0);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "###");
        assert_eq!(lines[2], "#@#");
        assert_eq!(lines[3], "#E#");
    }

    #[test]
    fn timer_urgency_white() {
        assert_eq!(timer_urgency(10.0, 100.0), TimerUrgency::Normal);
    }

    #[test]
    fn timer_urgency_yellow() {
        assert_eq!(timer_urgency(80.0, 100.0), TimerUrgency::Warning);
    }

    #[test]
    fn timer_urgency_red() {
        assert_eq!(timer_urgency(95.0, 100.0), TimerUrgency::Critical);
    }
}
