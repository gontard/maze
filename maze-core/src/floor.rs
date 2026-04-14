use rand::RngExt;
use rand::rngs::StdRng;

use crate::generator::{Kruskal, MazeGenerator, Prim, RecursiveBacktracker};
use crate::maze::Maze;

pub fn generate_floor(rng: &mut StdRng, start_pos: Option<(usize, usize)>) -> (Maze, f64) {
    let algo_index = rng.random_range(0..3u32);
    let seed = Some(rng.random::<u64>());
    let mut maze = match algo_index {
        0 => RecursiveBacktracker.generate(41, 21, seed, start_pos),
        1 => Kruskal.generate(41, 21, seed, start_pos),
        _ => Prim.generate(41, 21, seed, start_pos),
    };
    maze.carve_rooms(3, 3, 5, rng);
    maze.place_exit();

    let path_length = maze.solve().expect("generated maze must be solvable");
    let max_time_secs = path_length as f64 * 0.375;
    (maze, max_time_secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn returns_solvable_maze() {
        let mut rng = StdRng::seed_from_u64(42);
        let (maze, _) = generate_floor(&mut rng, None);
        assert!(maze.solve().is_some());
    }

    #[test]
    fn correct_time_budget() {
        let mut rng = StdRng::seed_from_u64(42);
        let (maze, max_time_secs) = generate_floor(&mut rng, None);
        let expected = maze.solve().unwrap() as f64 * 0.375;
        assert_eq!(max_time_secs, expected);
    }

    #[test]
    fn deterministic_with_same_seed() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let (maze1, time1) = generate_floor(&mut rng1, None);

        let mut rng2 = StdRng::seed_from_u64(42);
        let (maze2, time2) = generate_floor(&mut rng2, None);

        assert_eq!(maze1.grid, maze2.grid);
        assert_eq!(time1, time2);
    }

    #[test]
    fn start_pos_is_used() {
        let mut rng = StdRng::seed_from_u64(42);
        let (maze, _) = generate_floor(&mut rng, Some((5, 5)));
        assert_eq!(maze.start, (5, 5));
    }
}
