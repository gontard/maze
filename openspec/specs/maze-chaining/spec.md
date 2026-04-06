## ADDED Requirements

### Requirement: Floor progression on win
The system SHALL generate a new maze when the player reaches the exit, incrementing the floor counter, and continue gameplay on the new floor.

#### Scenario: Player completes a floor
- **WHEN** the player moves onto the `Exit` tile on floor N
- **THEN** a new maze is generated for floor N+1
- **AND** the floor counter increments to N+1

#### Scenario: New maze start matches old exit
- **WHEN** a new maze is generated for floor N+1
- **THEN** the new maze's start position SHALL equal the previous maze's exit position
- **AND** the player's screen position does not change

#### Scenario: Timer resets on new floor
- **WHEN** a new floor begins
- **THEN** the countdown timer resets to zero
- **AND** a new max time is computed from the new maze's solution path length

### Requirement: Infinite play until loss or quit
The system SHALL continue generating new floors indefinitely until the player runs out of time or quits.

#### Scenario: Game ends on timeout
- **WHEN** the player runs out of time on any floor
- **THEN** the game ends with a loss

#### Scenario: Game ends on quit
- **WHEN** the player presses quit on any floor
- **THEN** the game ends

#### Scenario: No upper floor limit
- **WHEN** the player keeps completing floors
- **THEN** new floors continue to be generated with no cap

### Requirement: Final message includes floor count
The system SHALL display the number of floors cleared when the game ends.

#### Scenario: Lost on floor N
- **WHEN** the game ends with a loss on floor N
- **THEN** the message indicates the player cleared N-1 floors

#### Scenario: Quit on floor N
- **WHEN** the player quits on floor N
- **THEN** the message indicates the player cleared N-1 floors
