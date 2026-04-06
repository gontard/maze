## MODIFIED Requirements

### Requirement: Max time derived from solution path length
The system SHALL compute the max allowed time as the current floor's maze shortest solution path length multiplied by a time factor.

#### Scenario: Max time calculation
- **WHEN** a maze with shortest solution path of N steps is generated for a new floor
- **THEN** max allowed time is recomputed based on that floor's maze

#### Scenario: Timer resets on new floor
- **WHEN** a new floor begins
- **THEN** the elapsed time resets to zero
- **AND** the max time is derived from the new maze's solution path
