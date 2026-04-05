## ADDED Requirements

### Requirement: Countdown timer display
The system SHALL display a countdown timer on a status bar above the maze, showing elapsed time and max time in MM:SS / MM:SS format.

#### Scenario: Timer visible during play
- **WHEN** the game is in progress
- **THEN** a status bar above the maze displays the timer in the format `⏱ MM:SS / MM:SS`

#### Scenario: Timer right-aligned to maze width
- **WHEN** the status bar is rendered
- **THEN** the timer text is right-aligned to the maze's display width

#### Scenario: Timer updates without input
- **WHEN** the player is idle (no key pressed)
- **THEN** the timer display still updates at least every 250ms

### Requirement: Time-based lose condition
The system SHALL end the game with a loss when elapsed time exceeds the max allowed time.

#### Scenario: Time runs out
- **WHEN** elapsed time exceeds max allowed time
- **THEN** the game ends with status `Lost`
- **AND** a timeout message is displayed (e.g., "Time's up!")

#### Scenario: Player cannot move after losing
- **WHEN** the game status is `Lost`
- **THEN** movement inputs are ignored

### Requirement: Max time derived from solution path length
The system SHALL compute the max allowed time as the maze's shortest solution path length multiplied by 1.5 seconds.

#### Scenario: Max time calculation
- **WHEN** a maze with shortest solution path of N steps is generated
- **THEN** max allowed time is N × 1.5 seconds

### Requirement: Timer color urgency
The system SHALL change the timer color based on remaining time percentage.

#### Scenario: Plenty of time remaining
- **WHEN** more than 25% of max time remains
- **THEN** the timer is displayed in white

#### Scenario: Time getting low
- **WHEN** between 10% and 25% of max time remains
- **THEN** the timer is displayed in yellow

#### Scenario: Time almost up
- **WHEN** less than 10% of max time remains
- **THEN** the timer is displayed in red
