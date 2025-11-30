# Challenges

Programming puzzles and problems to solve with tt-rs.

## Overview

Challenges are goal-oriented puzzles that test understanding of tt-rs concepts. Each challenge has a clear objective and success criteria. Users must figure out the solution themselves.

## Proposed Challenges

### Arithmetic Challenges

#### Make 10
**Difficulty**: Easy
**Goal**: Combine the given numbers to make exactly 10
**Starting Widgets**: Numbers 3, 4, 7 (only these)
**Success**: Create a number showing 10
**User Level**: tt1
**Hint**: Try different operations

#### Make 100
**Difficulty**: Medium
**Goal**: Create the number 100 using only the given numbers
**Starting Widgets**: Numbers 2, 5, 10
**Success**: Create a number showing 100
**User Level**: tt1
**Hint**: Multiplication helps

#### Fraction Challenge
**Difficulty**: Medium
**Goal**: Create exactly 1 from the given fractions
**Starting Widgets**: Numbers 1/2, 1/3, 1/6
**Success**: Create a number showing 1
**User Level**: tt1
**Hint**: Add them together

### Sorting Challenges

#### Sort Three
**Difficulty**: Easy
**Goal**: Arrange three numbers in order using the scale
**Starting Widgets**: Numbers 7, 3, 9 and a scale, three empty boxes labeled 1st, 2nd, 3rd
**Success**: Boxes contain 3, 7, 9 in order
**User Level**: tt1
**Hint**: Compare pairs to find the smallest

#### Sort Five
**Difficulty**: Medium
**Goal**: Sort five numbers in ascending order
**Starting Widgets**: Numbers 4, 8, 2, 6, 1 and a scale
**Success**: Numbers arranged 1, 2, 4, 6, 8
**User Level**: tt1
**Hint**: Find minimum repeatedly

### Logic Challenges

#### Balance the Scale
**Difficulty**: Easy
**Goal**: Make the scale balance perfectly
**Starting Widgets**: Scale with 5 on left, numbers 2, 3, 4, 6 available
**Success**: Scale is balanced (horizontal)
**User Level**: tt1
**Hint**: What equals 5?

#### Double Balance
**Difficulty**: Hard
**Goal**: Balance two scales simultaneously
**Starting Widgets**: Two scales, limited number supply
**Success**: Both scales balanced
**User Level**: tt1
**Hint**: Plan which numbers go where

### Robot Challenges

#### Teach Addition
**Difficulty**: Medium
**Goal**: Train a robot to add 5 to any number
**Starting Widgets**: Untrained robot, number +5, test numbers
**Success**: Robot correctly adds 5 to different inputs
**User Level**: tt1
**Hint**: Train with one number, test with others

#### Copy to Box
**Difficulty**: Medium
**Goal**: Train a robot to copy a number into a box
**Starting Widgets**: Robot, empty box, test numbers
**Success**: Robot puts copies of numbers in the box
**User Level**: tt1
**Hint**: Use the wand during training

### Messaging Challenges (tt2)

#### Delayed Delivery
**Difficulty**: Easy
**Goal**: Send a message that arrives in a specific box
**Starting Widgets**: Bird, nest (not in box), target box
**Success**: Message ends up in the target box
**User Level**: tt2
**Hint**: Where should the nest go?

#### Message Chain
**Difficulty**: Hard
**Goal**: Create a chain where a message passes through multiple nests
**Starting Widgets**: Multiple bird/nest pairs
**Success**: Original message reaches final destination
**User Level**: tt2
**Hint**: Nests can receive birds too

## Implementation Notes

Each challenge will be a workspace file containing:
- Starting widgets (and only those widgets)
- Clear goal statement in Workspace Notes
- Success criteria defined
- No solution provided (user must discover)
- Difficulty rating

Challenges should be satisfying to solve, with "aha!" moments.

## Difficulty Guidelines

- **Easy**: Solvable in 1-2 minutes, single concept
- **Medium**: Requires 3-5 minutes, combines concepts
- **Hard**: May take 10+ minutes, requires insight
