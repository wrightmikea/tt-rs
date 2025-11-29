# Use Cases

This document describes concrete use cases for tt-rs, organized by user level and avatar. Each use case represents a realistic program that accomplishes something educational or practical.

See [avatars.md](avatars.md) for avatar definitions.

---

## tt1 - Basic Level Use Cases

### UC-1.1: Make 10 (Alex)

**Avatar:** Alex (Beginner/Child)

**Goal:** Combine numbers to reach exactly 10.

**Learning Objectives:**
- Understand that dragging numbers onto each other combines them
- Practice mental arithmetic
- Experience immediate visual feedback

**Steps:**
1. Click on +1 stack to create a "1"
2. Click on +5 stack to create a "5"
3. Drag the 5 onto the 1 - they combine to make 6
4. Create another 1, drag onto 6 - now you have 7
5. Continue until you reach 10

**Success Criteria:**
- A single number showing "10" exists on the workspace

**Why This Matters:**
Simple, achievable goal that teaches the core interaction pattern.

---

### UC-1.2: Countdown Timer (Alex)

**Avatar:** Alex (Beginner/Child)

**Goal:** Train a robot to count down from 5 to 0.

**Learning Objectives:**
- Understand robot training mode
- See that robots repeat what you show them
- Introduction to loops/repetition

**Steps:**
1. Create a number 5
2. Put it in a box hole
3. Click the Robot to start training
4. Drag -1 onto the number (it becomes 4)
5. Click Robot to stop training
6. Robot will repeat: 4 -> 3 -> 2 -> 1 -> 0

**Success Criteria:**
- Robot automatically decrements the number each cycle
- Stops when reaching 0 (or continues counting negative)

**Why This Matters:**
First introduction to "programming by demonstration" - the robot learned by watching.

---

### UC-1.3: Simple Calculator (Jordan)

**Avatar:** Jordan (Hobbyist/Enthusiast)

**Goal:** Build a workspace for doing arithmetic.

**Learning Objectives:**
- Understand different arithmetic operators
- Use boxes to organize work
- Copy numbers from stacks efficiently

**Steps:**
1. Create a 3-hole box for "operand 1", "operator", "result"
2. Put a number in hole 1 (e.g., 12)
3. From the *2 stack, create a multiplier
4. Drag the multiplier onto the number - result appears
5. Use Scales to compare results

**Success Criteria:**
- Can perform +, -, *, / operations
- Results appear correctly
- Understands the operator display (+, -, *, /)

**Why This Matters:**
Demonstrates practical arithmetic and introduces the concept of operators as first-class objects.

---

### UC-1.4: Number Comparison (Alex/Jordan)

**Avatar:** Alex or Jordan

**Goal:** Use Scales to compare two numbers visually.

**Learning Objectives:**
- Understand visual comparison
- See that Scales tip toward the heavier (larger) number
- Introduction to conditional logic concepts

**Steps:**
1. Create two numbers (e.g., 7 and 3)
2. Drag the Scales to a clear area
3. Put 7 on the left pan (drag onto left side)
4. Put 3 on the right pan
5. Watch the Scales tip left (7 > 3)

**Success Criteria:**
- Scales visually indicate which number is larger
- Child can predict which way Scales will tip

**Why This Matters:**
Visual representation of comparison - foundational for understanding conditionals.

---

### UC-1.5: Cleanup with Vacuum (Alex)

**Avatar:** Alex (Beginner/Child)

**Goal:** Learn to remove unwanted items from the workspace.

**Learning Objectives:**
- Understand that Vacuum removes items
- Practice targeting specific items
- Learn workspace management

**Steps:**
1. Create several numbers scattered around
2. Pick up the Vacuum
3. Touch a number with the Vacuum - it disappears
4. Clean up all but one number

**Success Criteria:**
- Workspace is cleaned up
- Target number remains

**Why This Matters:**
Essential workspace management skill. Also teaches that actions have consequences.

---

### UC-1.6: Duplicate with Wand (Alex)

**Avatar:** Alex (Beginner/Child)

**Goal:** Make copies of items using the Magic Wand.

**Learning Objectives:**
- Understand copying vs. moving
- Create multiple instances of a number
- See that copies are independent

**Steps:**
1. Create a number 5
2. Pick up the Wand
3. Touch the 5 with the Wand - a copy appears
4. Make several copies
5. Change one copy (drag +1 onto it) - others unchanged

**Success Criteria:**
- Multiple copies exist
- Changing one doesn't affect others

**Why This Matters:**
Copying is fundamental. Shows that copies are independent objects.

---

## tt2 - Messaging Level Use Cases

### UC-2.1: Send a Message (Jordan)

**Avatar:** Jordan (Hobbyist/Enthusiast)

**Goal:** Use Bird/Nest to send a number from one place to another.

**Learning Objectives:**
- Understand Bird/Nest pairing (hatching)
- Send a message by dropping on a bird
- Receive the message at the nest

**Steps:**
1. Switch to tt2 mode (dropdown)
2. Click on Nest stack to "hatch" - creates Nest + paired Bird
3. Drag the Bird away from the Nest (to simulate distance)
4. Create a number 42
5. Drag the 42 onto the Bird
6. Watch: Bird "delivers" the number - it appears at the Nest

**Success Criteria:**
- Number 42 appears near the Nest
- Original 42 is consumed (given to the bird)
- Bird returns to ready state

**Why This Matters:**
First introduction to asynchronous messaging - a key programming concept.

---

### UC-2.2: Two-Way Communication (Jordan)

**Avatar:** Jordan (Hobbyist/Enthusiast)

**Goal:** Set up bidirectional messaging between two "processes."

**Learning Objectives:**
- Multiple Bird/Nest pairs
- Each direction needs its own pair
- Messages can cross

**Steps:**
1. Hatch Nest A - get Bird A
2. Hatch Nest B - get Bird B
3. Put Bird B near Nest A (B sends to A's area)
4. Put Bird A near Nest B (A sends to B's area)
5. Send a number via Bird A - arrives at Nest B
6. Send a response via Bird B - arrives at Nest A

**Success Criteria:**
- Messages arrive at correct destinations
- Bidirectional flow is established

**Why This Matters:**
Models real-world communication patterns. Foundation for understanding distributed systems.

---

### UC-2.3: Producer-Consumer (Jordan/Casey)

**Avatar:** Jordan or Casey

**Goal:** One robot produces numbers, another consumes them.

**Learning Objectives:**
- Robots can work independently
- Birds carry data between robots
- Asynchronous coordination

**Steps:**
1. Create Box A with a number and Robot A
2. Create Box B with a Nest and Robot B
3. Train Robot A: increment number, give copy to Bird
4. Train Robot B: when Nest has message, process it
5. Run both robots - they coordinate via messaging

**Success Criteria:**
- Robot A produces values
- Values appear at Robot B's Nest
- Robot B processes them

**Why This Matters:**
Classic concurrent programming pattern. Shows power of actor model.

---

### UC-2.4: Request-Response Pattern (Casey)

**Avatar:** Casey (Developer/Contributor)

**Goal:** Implement a service that responds to requests.

**Learning Objectives:**
- Birds as return addresses
- Nests as request queues
- Two-phase communication

**Steps:**
1. Service has a Nest (receives requests)
2. Each request includes a Bird (for the response)
3. Service processes request, sends result to included Bird
4. Client receives response at their Nest

**Implementation Notes:**
- Requires boxes to bundle request + return-bird
- Shows how to build protocols on Bird/Nest

**Success Criteria:**
- Client sends request with return bird
- Service processes and responds
- Client receives response

**Why This Matters:**
Foundation for remote procedure call (RPC) patterns. Demonstrates protocol design.

---

## Future Use Cases (tt3+)

These use cases require features not yet implemented:

### UC-3.1: Pattern Matching Calculator

**Requires:** Erasure, pattern matching

**Goal:** Robot that handles any arithmetic operation based on operator pattern.

---

### UC-3.2: Sorting Robot

**Requires:** Pattern matching, multiple boxes

**Goal:** Robot that sorts a list of numbers.

---

### UC-3.3: Factorial Calculator

**Requires:** Pattern matching, recursion via messaging

**Goal:** Compute factorial using Bird/Nest for recursive calls.

---

## Use Case Status Tracker

| ID | Name | Level | Avatar | Status |
|----|------|-------|--------|--------|
| UC-1.1 | Make 10 | tt1 | Alex | Ready |
| UC-1.2 | Countdown Timer | tt1 | Alex | Needs Robot fix |
| UC-1.3 | Simple Calculator | tt1 | Jordan | Ready |
| UC-1.4 | Number Comparison | tt1 | Alex/Jordan | Ready |
| UC-1.5 | Cleanup with Vacuum | tt1 | Alex | Ready |
| UC-1.6 | Duplicate with Wand | tt1 | Alex | Ready |
| UC-2.1 | Send a Message | tt2 | Jordan | Ready |
| UC-2.2 | Two-Way Communication | tt2 | Jordan | Ready |
| UC-2.3 | Producer-Consumer | tt2 | Jordan/Casey | Needs Robot fix |
| UC-2.4 | Request-Response | tt2 | Casey | Future |

---

## References

- [avatars.md](avatars.md) - User persona definitions
- [tutorials-roadmap.md](tutorials-roadmap.md) - Tutorial implementation plans
- [plan.md](plan.md) - Feature roadmap
