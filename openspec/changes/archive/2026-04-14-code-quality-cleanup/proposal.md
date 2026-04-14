## Why

The codebase has accumulated dead code, duplication across frontends, an unnecessary unsafe block, and naming inconsistencies that hinder maintainability. Addressing these now—while the codebase is small—prevents them from compounding as features are added.

## What Changes

- **Remove dead code**: delete `DrawCommand::FillRect` (never emitted), remove unused `IndexedRandom` import
- **Extract floor generation into maze-core**: the algorithm-selection, room-carving, exit-placement, and time-budget logic is duplicated between maze-terminal and maze-web — move it to a shared `FloorGenerator` in maze-core
- **Remove unsafe block in maze-web**: replace raw-pointer borrow-checker workaround with struct destructuring
- **Add `Maze::neighbors()` iterator**: eliminate the 5× duplicated BFS neighbor-traversal pattern across production and test code
- **Derive `width`/`height` from grid**: remove redundant fields that can drift out of sync
- **Parameterize generator tests**: replace copy-pasted Kruskal/Prim tests with a single suite parameterized over `MazeGenerator`
- **Improve names**: replace `init_maze` 5-tuple return with a named struct, expand cryptic abbreviations (`rw`→`room_w`, `gax`→`grid_ax`, `g`/`f`→`game_ref`/`callback`)

## Capabilities

### New Capabilities
- `floor-generation`: Shared floor generation logic (algorithm selection, room carving, exit placement, time budget) extracted from frontends into maze-core
- `maze-neighbors`: `Maze::neighbors(x, y)` iterator yielding valid traversable neighbor coordinates

### Modified Capabilities
- `draw-command`: Remove unused `FillRect` variant
- `maze-generation`: Derive width/height from grid, improve naming, add neighbors iterator

## Impact

- **maze-core**: new `floor.rs` module, changes to `maze.rs` (fields, neighbors), `render.rs` (remove FillRect), `generator.rs` (naming)
- **maze-terminal**: simplify `main.rs` to use shared floor generation
- **maze-web**: remove unsafe block, simplify `lib.rs` to use shared floor generation
- **Tests**: rewrite generator tests as parameterized suite, add missing test coverage
