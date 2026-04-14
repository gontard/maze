## REMOVED Requirements

### Requirement: DrawCommand::FillRect variant
**Reason**: The `FillRect` variant is never emitted by `render_frame`. No production code produces it. Both frontends handle it defensively but it serves no purpose.
**Migration**: Remove all `FillRect` match arms from maze-terminal and maze-web renderers. If rectangular fills are needed in the future, re-add the variant.

## MODIFIED Requirements

### Requirement: DrawCommand enum
The system SHALL define a `DrawCommand` enum in `maze-core` representing low-level rendering primitives. All rendering decisions SHALL be expressed as `Vec<DrawCommand>`. The variants are: `Clear`, `DrawChar`, and `DrawText`.

#### Scenario: Draw a character
- **WHEN** the renderer needs to draw a single character at a grid position
- **THEN** it emits `DrawCommand::DrawChar { x, y, ch, color }`

#### Scenario: Draw text
- **WHEN** the renderer needs to display text
- **THEN** it emits `DrawCommand::DrawText { x, y, text, color }`

#### Scenario: Clear the screen
- **WHEN** a new frame begins
- **THEN** the first command emitted SHALL be `DrawCommand::Clear`
