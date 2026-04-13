use crate::maze::{Maze, Tile};

#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Clear,
    FillRect {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    },
    DrawText {
        x: usize,
        y: usize,
        text: String,
        color: Color,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
    Green,
    Cyan,
    Yellow,
    Red,
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
        TimerUrgency::Warning => Color::Yellow,
        TimerUrgency::Critical => Color::Red,
    }
}

fn format_timer(elapsed: f64, max_time: f64) -> String {
    let elapsed_min = elapsed as u64 / 60;
    let elapsed_sec = elapsed as u64 % 60;
    let max_min = max_time as u64 / 60;
    let max_sec = max_time as u64 % 60;
    format!(
        "\u{23f1} {:02}:{:02} / {:02}:{:02}",
        elapsed_min, elapsed_sec, max_min, max_sec
    )
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Wall => Color::White,
        Tile::Path => Color::Black,
        Tile::Start => Color::Green,
        Tile::Exit => Color::Green,
    }
}

/// Produce draw commands for one complete frame.
///
/// Coordinates use the maze grid system: (x, y) = (column, row).
/// Row 0 is the status bar; the maze grid starts at row 1.
pub fn render_frame(
    maze: &Maze,
    player: (usize, usize),
    level: usize,
    elapsed: f64,
    max_time: f64,
) -> Vec<DrawCommand> {
    let mut cmds = Vec::new();

    cmds.push(DrawCommand::Clear);

    // Status bar at row 0
    let floor_label = format!("Floor {level}");
    cmds.push(DrawCommand::DrawText {
        x: 0,
        y: 0,
        text: floor_label,
        color: Color::White,
    });

    let timer_text = format_timer(elapsed, max_time);
    let timer_color = urgency_color(timer_urgency(elapsed, max_time));
    cmds.push(DrawCommand::DrawText {
        x: maze.width,
        y: 0,
        text: timer_text,
        color: timer_color,
    });

    // Maze grid starting at row 1
    for y in 0..maze.height {
        for x in 0..maze.width {
            let (color, is_player) = if (x, y) == player {
                (Color::Cyan, true)
            } else {
                (tile_color(maze.grid[y][x]), false)
            };
            let _ = is_player; // color already set
            cmds.push(DrawCommand::FillRect {
                x,
                y: y + 1, // offset by 1 for status bar
                width: 1,
                height: 1,
                color,
            });
        }
    }

    cmds
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::Maze;

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
    fn frame_starts_with_clear() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        assert_eq!(cmds[0], DrawCommand::Clear);
    }

    #[test]
    fn frame_contains_floor_text() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 3, 0.0, 60.0);
        let has_floor = cmds.iter().any(|c| {
            matches!(c,
                DrawCommand::DrawText { text, y: 0, .. } if text == "Floor 3"
            )
        });
        assert!(has_floor, "should contain Floor 3 text");
    }

    #[test]
    fn frame_contains_timer_text() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        let has_timer = cmds.iter().any(|c| {
            matches!(c,
                DrawCommand::DrawText { text, y: 0, .. } if text.contains('\u{23f1}')
            )
        });
        assert!(has_timer, "should contain timer text");
    }

    #[test]
    fn frame_contains_player_rect() {
        let maze = small_maze();
        let cmds = render_frame(&maze, (1, 1), 1, 0.0, 60.0);
        let has_player = cmds.iter().any(|c| {
            matches!(
                c,
                DrawCommand::FillRect {
                    x: 1,
                    y: 2,
                    color: Color::Cyan,
                    ..
                }
            )
        });
        assert!(has_player, "should contain player rect at (1,2) with Cyan");
    }

    #[test]
    fn frame_contains_wall_rects() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        // Top-left wall at grid (0,0) should be at draw y=1
        let has_wall = cmds.iter().any(|c| {
            matches!(
                c,
                DrawCommand::FillRect {
                    x: 0,
                    y: 1,
                    color: Color::White,
                    ..
                }
            )
        });
        assert!(has_wall, "should contain wall rect");
    }

    #[test]
    fn frame_contains_exit_rect() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        // Exit at grid (1,2) → draw y=3
        let has_exit = cmds.iter().any(|c| {
            matches!(
                c,
                DrawCommand::FillRect {
                    x: 1,
                    y: 3,
                    color: Color::Green,
                    ..
                }
            )
        });
        assert!(has_exit, "should contain exit rect at (1,3) with Green");
    }

    #[test]
    fn frame_rect_count_matches_grid() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        let rect_count = cmds
            .iter()
            .filter(|c| matches!(c, DrawCommand::FillRect { .. }))
            .count();
        assert_eq!(rect_count, maze.width * maze.height);
    }

    #[test]
    fn timer_urgency_normal() {
        assert_eq!(timer_urgency(10.0, 100.0), TimerUrgency::Normal);
    }

    #[test]
    fn timer_urgency_warning() {
        assert_eq!(timer_urgency(80.0, 100.0), TimerUrgency::Warning);
    }

    #[test]
    fn timer_urgency_critical() {
        assert_eq!(timer_urgency(95.0, 100.0), TimerUrgency::Critical);
    }

    #[test]
    fn timer_color_changes_with_urgency() {
        let maze = small_maze();
        let cmds_normal = render_frame(&maze, maze.start, 1, 10.0, 100.0);
        let cmds_critical = render_frame(&maze, maze.start, 1, 95.0, 100.0);

        let timer_color = |cmds: &[DrawCommand]| {
            cmds.iter().find_map(|c| match c {
                DrawCommand::DrawText { text, color, .. } if text.contains('\u{23f1}') => {
                    Some(*color)
                }
                _ => None,
            })
        };

        assert_eq!(timer_color(&cmds_normal), Some(Color::Cyan));
        assert_eq!(timer_color(&cmds_critical), Some(Color::Red));
    }

    #[test]
    fn format_timer_zero() {
        assert_eq!(format_timer(0.0, 60.0), "\u{23f1} 00:00 / 01:00");
    }

    #[test]
    fn format_timer_partial() {
        assert_eq!(format_timer(73.0, 120.0), "\u{23f1} 01:13 / 02:00");
    }

    #[test]
    fn format_timer_elapsed_exceeds_max() {
        assert_eq!(format_timer(130.0, 120.0), "\u{23f1} 02:10 / 02:00");
    }

    #[test]
    fn timer_text_is_right_aligned() {
        let maze = small_maze();
        let cmds = render_frame(&maze, maze.start, 1, 0.0, 60.0);
        let timer = cmds.iter().find(|c| {
            matches!(c,
                DrawCommand::DrawText { text, .. } if text.contains('\u{23f1}')
            )
        });
        match timer {
            Some(DrawCommand::DrawText { x, .. }) => {
                assert_eq!(
                    *x, maze.width,
                    "timer x should equal maze width (right-aligned marker)"
                );
            }
            _ => panic!("timer text not found"),
        }
    }
}
