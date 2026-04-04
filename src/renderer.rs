use std::io::{self, Write};

use crossterm::{cursor, execute, style::Print};

use crate::maze::{Maze, Tile};

fn tile_char(tile: Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Path => ' ',
        Tile::Start => 'S',
        Tile::Exit => 'E',
    }
}

pub fn render(maze: &Maze, player: (usize, usize)) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0))?;

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
pub fn render_to_string(maze: &Maze, player: (usize, usize)) -> String {
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
    fn rendered_line_width_matches_grid_width() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start);
        for line in output.lines() {
            assert_eq!(line.len(), maze.width, "line: {line:?}");
        }
    }

    #[test]
    fn rendered_line_count_matches_height() {
        let maze = small_maze();
        let output = render_to_string(&maze, maze.start);
        assert_eq!(output.lines().count(), maze.height);
    }

    #[test]
    fn player_renders_at_position() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "#@#");
    }

    #[test]
    fn wall_renders_as_hash() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "###");
    }

    #[test]
    fn exit_renders_correctly() {
        let maze = small_maze();
        let output = render_to_string(&maze, (1, 1));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[2], "#E#");
    }

    #[test]
    fn tile_char_mapping() {
        assert_eq!(tile_char(Tile::Wall), '#');
        assert_eq!(tile_char(Tile::Path), ' ');
        assert_eq!(tile_char(Tile::Start), 'S');
        assert_eq!(tile_char(Tile::Exit), 'E');
    }
}
