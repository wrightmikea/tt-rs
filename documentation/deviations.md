# Design Deviations from Original ToonTalk

This document captures conscious decisions where tt-rs behavior differs from the original ToonTalk (1995 C++) or ToonTalk Reborn (2014-2017 JavaScript). Each deviation includes the rationale for why the new approach is preferred.

## Deviations

### DEV-001: Box Creation from Stacks

**Original ToonTalk:** Boxes were created by picking up a box and the number of holes was determined by other interactions.

**tt-rs Behavior:** Drag a box stack and press 0-9 while dragging to create a copy with that many holes. The original box remains in place as a prototype.

**Rationale:**
- More discoverable through keyboard exploration
- Keeps the original as a prototype (consistent with other copy sources)
- Immediate feedback on box size

**Status:** Implemented

---

### DEV-002: Widget Z-Ordering

**Original ToonTalk:** Widget stacking was managed through the 3D environment and explicit "pick up" operations.

**tt-rs Behavior:** Uses CSS z-index layers:
- Copy sources (stacks): z-index 1
- Normal widgets: z-index 10
- Widgets being dragged: z-index 100

**Rationale:**
- Ensures newly created widgets always appear above their source stacks
- No explicit "bring to front" action needed
- Intuitive visual layering

**Status:** Implemented

---

### DEV-003: Bird/Nest Hatching

**Original ToonTalk:** Birds and nests had more complex creation and pairing mechanics involving the bird flying to find its nest.

**tt-rs Behavior:** Clicking a Nest stack "hatches" it, immediately creating a paired Bird+Nest. The bird appears next to the nest, already paired.

**Rationale:**
- Simpler mental model for beginners
- Immediate visual connection between bird and nest
- Reduces steps needed to start using messaging

**Status:** Implemented

---

### DEV-004: User Level System

**Original ToonTalk:** Had complex progression with different tools and features revealed over time through gameplay.

**tt-rs Behavior:** Simple dropdown selector (tt1/tt2) that shows/hides features by category:
- tt1: Basic operations (numbers, boxes, scales, tools)
- tt2: Messaging (bird/nest)

**Rationale:**
- More accessible for educational settings
- Teachers can control feature exposure
- Easier to add new levels without redesigning progression

**Status:** Implemented

---

### DEV-005: Column-Based Palette Layout

**Original ToonTalk:** Tools and widgets were scattered around a 3D environment, discovered through exploration.

**tt-rs Behavior:** Organized vertical columns by category (Numbers | Boxes | tt1 Tools | tt2 Tools).

**Rationale:**
- Faster access to common operations
- Clear visual grouping by purpose
- Leaves room for future levels (tt3, tt4 columns)

**Status:** Implemented

---

### DEV-006: Negative Index Box Splitting

**ToonTalk Reborn:** Dropping a box on a negative number displays an error message: "This would make a box with N holes for some negative N." The operation is rejected.

**tt-rs Behavior:** Negative numbers split from the right instead of the left:
- 8-hole box on -2 → 6-hole + 2-hole (2 holes from the right)
- 6-hole box on -1 → 5-hole + 1-hole (1 hole from the right)

**Rationale:**
- Provides useful functionality rather than rejecting valid inputs
- Symmetrical with positive splitting (positive splits from left, negative from right)
- Intuitive mental model: negative indices in many programming languages mean "from the end"
- No information is lost - users who want ToonTalk behavior can simply avoid negative numbers

**Status:** Implemented

---

### DEV-007: Box Copy on Zero

**ToonTalk Reborn:** Dropping a box on 0 creates a 2-hole box containing a 0-hole empty box and the original box's contents (wrapping behavior).

**tt-rs Behavior:** Dropping a box on 0 creates a deep copy of the original box, including all its contents.

**Rationale:**
- More intuitive: zero means "don't split, just copy"
- Useful for duplicating populated boxes without using the wand
- Simpler mental model: 0 acts as the identity operation for boxes
- The ToonTalk wrapping behavior can be achieved through other operations if needed

**Status:** Implemented

---

## Pending Decisions

### DEV-P01: Stack Dragging

**Question:** Should copy source stacks be draggable to rearrange the palette?

**Original ToonTalk:** Tools could be moved around the environment.

**Options:**
1. Keep stacks fixed (simpler, consistent layout)
2. Make stacks draggable (user customization)
3. Add "lock/unlock layout" mode

**Current Status:** Stacks are not draggable (regression from earlier behavior)

---

### DEV-P02: Workspace Persistence

**Question:** How should user work be saved and loaded?

**Original ToonTalk:** Save/load through operating system files.

**Options:**
1. Auto-save to localStorage
2. Explicit save/load buttons
3. URL-encoded sharing for small programs

**Current Status:** Not implemented

---

## Documentation Requirements

When adding a new deviation:

1. Assign next DEV-XXX number
2. Document original ToonTalk behavior (cite source if available)
3. Describe tt-rs behavior clearly
4. Explain rationale for the change
5. Note implementation status
6. Reference this deviation in code comments where relevant

## References

- [Original ToonTalk](http://toontalk.com/) - Ken Kahn's original work
- [ToonTalk Reborn](https://github.com/ToonTalk/ToonTalk) - JavaScript reimplementation
- [ToonTalk Papers](http://toontalk.com/English/papers.htm) - Academic documentation
