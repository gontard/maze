## MODIFIED Requirements

### Requirement: Timer
The system SHALL track elapsed time from game start, enforce a max time limit, and display the elapsed time upon victory or loss.

#### Scenario: Timer starts on game start
- **WHEN** the game begins
- **THEN** the timer starts counting from zero

#### Scenario: Timer displayed on win
- **WHEN** the player reaches the exit
- **THEN** the victory message includes the elapsed time in seconds

#### Scenario: Timer displayed on loss
- **WHEN** the player runs out of time
- **THEN** a loss message is displayed indicating time has expired

## ADDED Requirements

### Requirement: Poll-based game loop
The system SHALL use a poll-based loop with a timeout so the game state updates continuously, not only on key presses.

#### Scenario: Loop ticks without input
- **WHEN** no key is pressed for 100ms
- **THEN** the loop still ticks, re-renders the timer, and checks for timeout

#### Scenario: Key input still processed
- **WHEN** a key is pressed within the poll window
- **THEN** the key event is processed normally

### Requirement: Timeout check each tick
The system SHALL check whether elapsed time exceeds max time on every loop iteration.

#### Scenario: Timeout detected mid-idle
- **WHEN** the player is idle and time expires
- **THEN** the game transitions to `Lost` on the next tick without requiring a key press
