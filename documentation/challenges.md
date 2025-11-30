# Challenges

Programming puzzles and problems to solve with tt-rs.

## Overview

Challenges are goal-oriented puzzles that test understanding of tt-rs concepts. Each challenge has a clear objective and success criteria. Users must figure out the solution themselves.

## ToonTalk Classic Puzzles (from ToonTalk Reborn 2017)

These three puzzles are adapted from the original ToonTalk Reborn puzzle series:
https://toontalk.github.io/ToonTalk/puzzles/classic/p1.html

### Puzzle 1: Fill a Box with 1 and 2

**File**: `puzzle-fill-box.json`
**Difficulty**: Easy (Introductory)
**User Level**: tt1

**Starting Widgets**:
- Number 1
- Number 2
- Empty 2-hole box
- DropZone: "I need a box with a 1 and a 2. Please drop it here."

**Goal**: Put the numbers 1 and 2 into the box, then drop the completed box on the DropZone.

**Success Criteria**: Box with [1, 2] matches the expected pattern.

**Skills Taught**:
- Dragging widgets
- Dropping items into box holes
- Using the DropZone to verify answers

**Solution**: Drag 1 into first hole, drag 2 into second hole, drag box onto DropZone.

---

### Puzzle 2: Make a 4

**File**: `puzzle-make-four.json`
**Difficulty**: Easy
**User Level**: tt1

**Starting Widgets**:
- Number 2 (first copy)
- Number 2 (second copy)
- DropZone: "I need a 4. Please drop it here."

**Goal**: Create the number 4 and drop it on the DropZone.

**Success Criteria**: Number 4 matches the expected pattern.

**Skills Taught**:
- Number arithmetic (drop one number on another to add)
- Understanding that dropping adds numbers

**Solution**: Drag one 2 onto the other 2. Result is 4. Drop 4 on DropZone.

---

### Puzzle 3: Make a 9

**File**: `puzzle-make-nine.json`
**Difficulty**: Easy-Medium
**User Level**: tt1

**Starting Widgets**:
- Number 3 (as copy source - unlimited copies)
- DropZone: "I need a 9. Please drop it here."

**Goal**: Create the number 9 using only 3s and drop it on the DropZone.

**Success Criteria**: Number 9 matches the expected pattern.

**Skills Taught**:
- Using copy sources (drag to get copies, original stays)
- Multiple additions (3 + 3 + 3 = 9)
- Planning a multi-step solution

**Solution**:
1. Drag a 3 from the stack (creates a copy)
2. Drag another 3 from the stack
3. Drop one 3 on the other to make 6
4. Drag another 3 from the stack
5. Drop it on 6 to make 9
6. Drop 9 on DropZone

---

## New Widget Types Required

### DropZone Widget

A verification drop target used in puzzles:
- Displays a label/instruction (e.g., "I need a 4")
- Has an expected pattern that dropped widgets are checked against
- Visual feedback on success (green) or failure (red/rejected)
- Can optionally trigger navigation to next puzzle on success

### Copy Source Pattern (existing)

Any widget can be marked as a copy source using `is_copy_source`:
- Already implemented for Number, Nest, Bird
- Need to extend to Box for puzzles
- When dragged FROM, creates a copy while original stays
- Visual indicator showing it's a stack (shadow/depth effect via CSS)
- Used for puzzle starting materials and the palette

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
