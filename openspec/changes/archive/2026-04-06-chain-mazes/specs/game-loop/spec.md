## MODIFIED Requirements

### Requirement: Win condition
The system SHALL detect when the player reaches the `Exit` tile and transition to the next floor rather than ending the game.

#### Scenario: Player reaches exit
- **WHEN** the player moves onto the `Exit` tile
- **THEN** a new floor is generated and gameplay continues

#### Scenario: Player reaches exit on any floor
- **WHEN** the player reaches the exit on floor N
- **THEN** floor N+1 begins with a new maze, fresh timer, and the player at the same position
