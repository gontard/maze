# Tasks

## 1. Add `name()` to `MazeGenerator` trait
- [x] Add `fn name(&self) -> &'static str` to the trait
- [x] Implement for `RecursiveBacktracker` → `"Recursive Backtracker"`
- [x] Update existing tests if needed

## 2. Implement Kruskal's algorithm
- [x] Add union-find helper (private to `generator.rs`)
- [x] Add `Kruskal` struct implementing `MazeGenerator`
- [x] Tests: dimensions, solvable, all cells reachable, borders, deterministic, custom start

## 3. Implement Prim's algorithm
- [x] Add `Prim` struct implementing `MazeGenerator`
- [x] Tests: dimensions, solvable, all cells reachable, borders, deterministic, custom start

## 4. Random selection per floor
- [x] In `main.rs`, create rng for algorithm selection
- [x] Match on random index to pick algorithm and get name
- [x] Pass algorithm name to renderer

## 5. Display algorithm name on HUD
- [x] Add `algo_name: &str` parameter to `renderer::render()`
- [x] Display it alongside floor number and timer
