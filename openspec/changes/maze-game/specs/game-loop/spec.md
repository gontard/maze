## ADDED Requirements

### Requirement: Player movement
The system SHALL move the player one cell in the requested direction (up, down, left, right) when an arrow key or WASD key is pressed.

#### Scenario: Move to open cell
- **WHEN** the player presses an arrow key or WASD key
- **AND** the adjacent cell in that direction is traversable
- **THEN** the player position updates to that cell

#### Scenario: Move blocked by wall
- **WHEN** the player presses an arrow key or WASD key
- **AND** the adjacent cell in that direction is a `Wall`
- **THEN** the player position does not change

#### Scenario: Move at maze boundary
- **WHEN** the player presses a key that would move outside the grid
- **THEN** the player position does not change

### Requirement: Player starts at Start tile
The system SHALL place the player at the `Start` tile position when the game begins.

#### Scenario: Initial player position
- **WHEN** the game starts
- **THEN** the player is positioned on the `Start` tile

### Requirement: Win condition
The system SHALL detect when the player reaches the `Exit` tile and end the game with a victory message.

#### Scenario: Player reaches exit
- **WHEN** the player moves onto the `Exit` tile
- **THEN** the game ends and displays a victory message

### Requirement: Timer
The system SHALL track elapsed time from game start and display the elapsed time upon victory.

#### Scenario: Timer starts on game start
- **WHEN** the game begins
- **THEN** the timer starts counting from zero

#### Scenario: Timer displayed on win
- **WHEN** the player reaches the exit
- **THEN** the victory message includes the elapsed time in seconds

### Requirement: Quit game
The system SHALL allow the player to quit by pressing `q` or `Esc`.

#### Scenario: Player quits
- **WHEN** the player presses `q` or `Esc`
- **THEN** the game ends, terminal state is restored, and no victory message is shown

### Requirement: Terminal state restoration
The system SHALL restore the terminal to its original state when the game ends, whether by win, quit, or panic.

#### Scenario: Normal exit restores terminal
- **WHEN** the game ends normally (win or quit)
- **THEN** raw mode is disabled and the cursor is visible

#### Scenario: Panic restores terminal
- **WHEN** the game panics
- **THEN** a panic hook restores raw mode and cursor visibility before unwinding
