# tt-rs Implementation Plan

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

This document outlines the implementation plan for tt-rs, tracking both current progress and future work. The plan supports two implementation strategies:
1. **MVP Path**: Focus on ToonTalk Reborn (2017 jQuery) feature parity first
2. **Full Vision**: Eventually implement all original ToonTalk (1995 C++) features plus new innovations

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution.

---

## Current Implementation Status (as of November 2025)

### ✅ Completed Features

| Feature | Status | Notes |
|---------|--------|-------|
| **Project Infrastructure** | ✅ Complete | Multi-component architecture, build scripts, GitHub Pages deployment |
| **Widget Trait System** | ✅ Complete | Core abstraction with id, copy, matches, render, description |
| **Number Widget** | ✅ Complete | Rational arithmetic, operators (+,-,*,/), drop-to-apply |
| **Box Widget** | ✅ Complete | Configurable holes (0-9 via keyboard), contents management |
| **Text Widget** | ✅ Complete | Basic text display |
| **Scales Widget** | ✅ Complete | Numeric comparison, tipping animation |
| **Robot Widget** | ✅ Complete | Training mode, action recording, basic execution |
| **Bird Widget** | ✅ Complete | SVG rendering, copy source, basic structure |
| **Nest Widget** | ✅ Complete | SVG rendering, message queue structure |
| **Wand Tool** | ✅ Complete | Copy on touch |
| **Vacuum Tool** | ✅ Complete | Remove widgets |
| **Drag & Drop** | ✅ Complete | Full DnD system with visual feedback |
| **Box Joining** | ✅ Complete | Drop box on edge of another to combine |
| **Box Splitting** | ✅ Complete | Drop box on number to split |
| **Help Panel** | ✅ Complete | Slide-out help with tutorials, user-level aware |
| **User Levels** | ✅ Complete | tt1 (basics), tt2 (messaging) with level-specific help |
| **Tooltips** | ✅ Complete | Contextual tooltips on all widgets |
| **Compact Footer** | ✅ Complete | Links to License, GitHub, Changelog with build info |
| **Workspace Menu** | ✅ Complete | Tabbed browser for Tutorials, Examples, Challenges |
| **Workspace Notes** | ✅ Complete | Editable TextPane with mode-specific content |

### 🚧 Partially Implemented

| Feature | Status | What's Missing |
|---------|--------|----------------|
| Bird/Nest Messaging | ✅ Complete | Hatching, pairing, message delivery all working |
| Robot Execution | 🚧 Partial | Pattern matching, bindings, watched execution |
| Scales in Boxes | 🚧 Partial | Scales work standalone, not yet in box holes |
| Erasure Levels | 🚧 Partial | Vacuum removes but doesn't create erased patterns |

### ❌ Not Yet Started

| Feature | Priority | Notes |
|---------|----------|-------|
| Bird/Nest Pairing | High | Drop bird on nest to pair them |
| Message Delivery | High | Drop item on bird → delivers to paired nest |
| Pattern Matching | High | Required for robot generalization |
| Workspace Serialization | High | JSON format for save/load (UI done) |
| Bundled Tutorials/Examples | High | See tutorials.md, examples.md, challenges.md |
| Text Explosion | Medium | Drop text on box → individual letters |
| Sensors | Medium | Keyboard/mouse/touch event handling |
| Backside Views | Medium | Widget configuration interface |
| Element Widget | Low | HTML/SVG content |
| Houses/City | Low | Process containers (original ToonTalk) |
| Trucks | Low | Process spawning (original ToonTalk) |
| Bombs | Low | Process termination (original ToonTalk) |
| Helicopter | Low | City navigation (original ToonTalk) |
| 3D Graphics | Future | Three.js integration |
| Sound/Speech | Future | Audio feedback, TTS |

---

## Short-Term Roadmap (Next 3 Iterations)

### Iteration 1: Bird/Nest Messaging (MVP Critical) - IN PROGRESS

**Goal**: Enable asynchronous communication - the heart of ToonTalk's concurrency model.

#### 1.1 Nest Widget ✅ DONE
- [x] Create `tt-rs-nest` crate in containers component
- [x] Implement `Nest` struct with contents queue
- [x] Implement nest rendering (tt-nest.svg)
- [x] Implement `take()` method for retrieving contents

#### 1.2 Bird Widget ✅ DONE
- [x] Create `tt-rs-bird` crate in containers component
- [x] Implement `Bird` struct with nest reference
- [x] Implement bird rendering (tt-bird.svg)
- [x] Bird as copy source in demo

#### 1.3 Bird/Nest Pairing - NEXT
- [ ] Implement pairing logic (drop bird on nest)
- [ ] Store paired nest ID in Bird
- [ ] Visual indicator when paired (bird near nest)
- [ ] Pairing persists until bird removed

#### 1.4 Message Delivery - NEXT
- [ ] Implement "give to bird" drag operation
- [ ] Bird delivers copy to paired nest
- [ ] Nest receives and queues message
- [ ] Click nest to retrieve oldest message

#### 1.5 Demo/Tutorial ✅ DONE
- [x] Add bird/nest to demo widgets
- [x] Add tutorial section explaining messaging (tt2 help)
- [ ] Interactive example: send number to nest, retrieve it

### Iteration 2: Pattern Matching & Erasure

**Goal**: Enable robots to work with generalized patterns, not just specific values.

#### 2.1 Erasure System
- [ ] Define `ErasureLevel` enum (Specific, Sign, Type)
- [ ] Extend Number with erasure level field
- [ ] Vacuum tool creates erased copies (multi-click levels)
- [ ] Visual indication of erased widgets (ghosted/transparent)

#### 2.2 Pattern Matching
- [ ] Implement `matches()` for erased Numbers
- [ ] Implement `matches()` for Boxes (recursive)
- [ ] Extract bindings during match
- [ ] Robot condition checking uses matches()

#### 2.3 Robot Generalization
- [ ] Robot thought bubble shows conditions
- [ ] Erase widget in conditions to generalize
- [ ] Bindings passed to action execution

#### 2.4 Demo/Tutorial
- [ ] Tutorial: "Train robot, then erase number to work with any number"
- [ ] Example: Doubling robot (works on any input)

### Iteration 3: Persistence & Sharing (IN PROGRESS)

**Goal**: Save and load workspaces so users can continue their work and share tutorials.

#### 3.1 Workspace Menu UI (Part 1) ✅ DONE
- [x] Create `WorkspaceButton` component in tt-rs-ui
- [x] Create `WorkspaceMenu` slide panel component with tabs
- [x] Add workspace button to header (next to user level selector)
- [x] Implement open/close state management in App
- [x] Style workspace menu CSS
- [x] Add Tutorials, Examples, Challenges tabs with "Coming soon..." placeholders
- [x] Create `TextPane` component for Workspace Notes
- [x] Mode-specific default content (tt1/tt2)
- [x] Tooltips for mode dropdown and workspace button

#### 3.2 Workspace Data Structures
- [ ] Define `Workspace` struct with metadata
- [ ] Define `WorkspaceMetadata` (id, name, description, user_level, timestamps)
- [ ] Define `WidgetData` enum for serializable widget state
- [ ] Define `BoxData` struct for serializable box state
- [ ] Define `PositionData` struct for x/y coordinates
- [ ] Add serde Serialize/Deserialize derives

#### 3.3 Widget Serialization
- [ ] Implement `to_widget_data()` for Number
- [ ] Implement `to_widget_data()` for Text
- [ ] Implement `to_widget_data()` for Scales
- [ ] Implement `to_widget_data()` for Robot (with actions)
- [ ] Implement `to_widget_data()` for Bird (with paired nest)
- [ ] Implement `to_widget_data()` for Nest
- [ ] Implement `to_widget_data()` for Vacuum, Wand
- [ ] Implement `to_box_data()` for BoxState
- [ ] Implement reverse: `from_widget_data()` for all types

#### 3.4 AppState Serialization
- [ ] Implement `AppState::to_workspace()` method
- [ ] Filter out copy sources (palette items)
- [ ] Serialize widget positions
- [ ] Serialize box contents (widget_in_box mapping)
- [ ] Implement `AppState::from_workspace()` method
- [ ] ID remapping on load (fresh IDs to avoid conflicts)

#### 3.5 Browser LocalStorage Backend (Part 2a)
- [ ] Create `LocalStorageBackend` struct
- [ ] Implement workspace index management
- [ ] Implement `list()` - return all workspace metadata
- [ ] Implement `save()` - store workspace JSON
- [ ] Implement `load()` - retrieve and parse workspace
- [ ] Implement `delete()` - remove workspace

#### 3.6 Save Dialog UI
- [ ] Create save form with name input
- [ ] Create description textarea (multi-line)
- [ ] Auto-populate user level from current state
- [ ] Validate name is not empty
- [ ] Implement save button callback
- [ ] Implement cancel button callback

#### 3.7 Workspace List UI
- [ ] Display list of saved workspaces
- [ ] Show metadata: name, description, user level, date
- [ ] Group by: My Workspaces vs Examples/Tutorials
- [ ] Load button for each workspace
- [ ] Delete button for user workspaces (not bundled)
- [ ] Confirmation dialog for delete

#### 3.8 File Export/Import (Part 2b)
- [ ] Implement `export_to_file()` using web_sys
- [ ] Generate filename: `{name}-{date}.tt-rs.json`
- [ ] Trigger browser download
- [ ] Implement file input component
- [ ] Handle file selection event
- [ ] Parse and validate imported JSON
- [ ] Import button in workspace menu

#### 3.9 Bundled Workspace Content (Part 2c)

See detailed content designs in:
- [tutorials.md](tutorials.md) - 7 step-by-step guided lessons
- [examples.md](examples.md) - 10 pre-built demo workspaces
- [challenges.md](challenges.md) - 10 programming puzzles

**Tutorials (tt1 Basic - 5 lessons)**
- [ ] `tutorial-hello-numbers.json` - Learn number stacks and arithmetic
- [ ] `tutorial-boxing-things.json` - Using boxes as containers
- [ ] `tutorial-balancing-act.json` - Comparing values with scales
- [ ] `tutorial-copy-and-clean.json` - Wand and vacuum tools
- [ ] `tutorial-meet-robot.json` - Introduction to automation

**Tutorials (tt2 Messaging - 2 lessons)**
- [ ] `tutorial-birds-nests.json` - Asynchronous messaging
- [ ] `tutorial-remote-comm.json` - Inter-box messaging

**Examples (5 priority)**
- [ ] `example-calculator.json` - Organized calculation workspace
- [ ] `example-fraction-fun.json` - Rational number arithmetic
- [ ] `example-nested-boxes.json` - Hierarchical organization
- [ ] `example-copy-machine.json` - Trained robot copies numbers
- [ ] `example-mailbox.json` - Bird/nest message queuing (tt2)

**Challenges (5 priority)**
- [ ] `challenge-make-10.json` - Combine numbers to make 10
- [ ] `challenge-sort-three.json` - Sort numbers using scales
- [ ] `challenge-balance.json` - Make the scale balance
- [ ] `challenge-teach-addition.json` - Train robot to add 5
- [ ] `challenge-delayed-delivery.json` - Send message to box (tt2)

**Implementation**
- [ ] Create `assets/workspaces/` directory in tt-rs-app
- [ ] Use `include_str!()` to embed workspaces in WASM
- [ ] Implement `BundledWorkspacesBackend`
- [ ] Mark bundled content with `is_bundled: true`
- [ ] Hide delete button for bundled content

#### 3.10 Description Display (Part 3)
- [ ] Show description text when workspace is loaded
- [ ] Create dismissible info panel at top of workspace
- [ ] Include tutorial instructions in description
- [ ] Style info panel with clear visual distinction
- [ ] Close button to dismiss description

#### 3.11 Level Switching on Load
- [ ] Store user_level in workspace metadata
- [ ] When loading, switch to workspace's user level
- [ ] Show confirmation if level will change
- [ ] Ensure widgets visible at that level are loaded

#### 3.12 Polish & Testing
- [ ] Test save/load round-trip for all widget types
- [ ] Test robot with recorded actions
- [ ] Test bird/nest pairings persistence
- [ ] Test box contents persistence
- [ ] Handle storage quota errors gracefully
- [ ] Handle corrupted workspace JSON gracefully

#### 3.13 URL Sharing (Optional - Lower Priority)
- [ ] Encode small programs in URL
- [ ] Decode and load from URL parameter

---

## Medium-Term Roadmap (Iterations 4-9)

### Iteration 4: Puzzle Infrastructure ✅ PARTIALLY COMPLETE

**Goal**: Enable puzzle/challenge workspaces with verification drop zones.

Based on ToonTalk Classic Puzzles (https://toontalk.github.io/ToonTalk/puzzles/classic/p1.html):

#### 4.1 DropZone Widget ✅ DONE
- [x] Create `tt-rs-dropzone` crate in containers component
- [x] Implement `DropZone` struct with:
  - `label: String` - displayed text (e.g., "I need a box with 1 and 2")
  - `expected` - pattern to match against (stored separately in puzzle JSON)
  - `on_success_message: Option<String>` - message on success
  - `on_success_url: Option<String>` - URL for next puzzle
- [x] Implement drop zone rendering (styled text box with dashed border)
- [x] Visual feedback on hover (highlight when valid drop target)

#### 4.2 Drop Verification ✅ DONE
- [x] Implement hit test for detecting drops on DropZone
- [x] On drop: check if dropped widget matches expected pattern
- [x] Success: green background, checkmark ✓, success message logged
- [x] Failure: red background, X mark ✗, "Try again!" message, shake animation
- [x] Widget bounces back on failure (stays in workspace)
- [ ] Optional: success message or next puzzle URL navigation

#### 4.3 Copy Source for All Widgets (Refactor)

**Note**: We already have `is_copy_source` pattern on Number, Nest, Bird. Extend to Box:

- [ ] Add `is_copy_source: bool` field to `BoxState`
- [ ] Implement `as_copy_source()` builder method for BoxState
- [ ] Update `BoxState::copy()` to reset `is_copy_source` to false
- [ ] Render copy-source boxes with stacked visual (shadow effect)
- [ ] Ensure copy sources work in puzzle workspaces

This uses the existing `CopySource` component from tt-rs-drag, which already:
- Stays in place when dragged from
- Creates copies on click/drag
- Shows stacked visual appearance

#### 4.4 ToonTalk Classic Puzzles (3 workspaces) 🚧 PARTIAL
- [x] `puzzle-fill-box.json` - Puzzle 1: Put 1 and 2 in a box (working!)
- [ ] `puzzle-make-four.json` - Puzzle 2: Add 2+2 to make 4
- [ ] `puzzle-make-nine.json` - Puzzle 3: Add 3+3+3 to make 9

#### 4.5 Puzzle Navigation (Optional)
- [ ] Success triggers navigation to next puzzle URL
- [ ] Puzzle sequence configuration
- [ ] Progress tracking in localStorage

### Iteration 4B: Puzzle/Tutorial Usability

**Goal**: Make puzzles and tutorials recoverable and shareable.

**Problem**: Currently, if a user makes a mistake (e.g., putting numbers in wrong order), they cannot recover without reloading the page, which exits the puzzle.

#### 4B.1 Reset Button (HIGH PRIORITY)
- [ ] Add "Reset" button to workspace when puzzle is loaded
- [ ] Button placement: floating toolbar or in DropZone widget
- [ ] On click: reload puzzle from original JSON
- [ ] Clear error state on DropZone
- [ ] Keep same URL (stay on puzzle)
- [ ] Optional: confirmation toast "Puzzle reset!"

#### 4B.2 URL-Based Routing (HIGH PRIORITY)
- [ ] Create `Route` enum (Home, Tutorial, Example, Challenge, Workspace)
- [ ] Implement hash-based routing (`#/tutorial/fill-a-box`)
- [ ] Parse route on page load
- [ ] Update URL when loading puzzle/workspace
- [ ] Listen for browser back/forward (popstate)

#### 4B.3 Page Reload Persistence
- [ ] On mount: check URL hash
- [ ] If hash indicates puzzle, load it
- [ ] Browser reload stays on same puzzle
- [ ] Bookmarks work correctly

#### 4B.4 Undo Last Action (MEDIUM PRIORITY)
- [ ] Create command pattern for undoable actions
- [ ] Track: PlaceInHole, RemoveFromHole, MoveWidget, ApplyArithmetic
- [ ] Single undo (Ctrl+Z or Undo button)
- [ ] Undo restores previous state

#### 4B.5 "Show Me" Animated Demo (LOW PRIORITY)
- [ ] Define `DemoStep` enum (Wait, MoveTo, DragStart, DragMove, DragEnd, ShowTooltip)
- [ ] Add `demo` field to puzzle JSON format
- [ ] Implement animation playback system
- [ ] "Show Me" button plays solution animation
- [ ] Smooth interpolation for drag movements

#### 4B.6 Hint System (LOW PRIORITY)
- [ ] Add `hints` field to puzzle JSON format
- [ ] "Hint" button reveals progressive hints
- [ ] Track hints revealed per puzzle
- [ ] Visual indicator for available hints

### Iteration 5: Sensors & Events
- Keyboard sensor (key press detection)
- Mouse sensor (click, position)
- Touch sensor (mobile support)
- Sensors trigger robot activation

### Iteration 6: Backside & Configuration
- Flip widget to see backside
- Robot backside shows conditions
- Number backside shows operator selection
- Box backside shows labels

### Iteration 7: Robot Execution Polish
- Watched execution (step-by-step with animation)
- Speed control slider
- Visual highlighting of current action
- Robot chaining (sequential robots)

### Iteration 8: Text Operations
- Text explosion (text on box → letters in holes)
- Text joining (box contents → concatenated text)
- Text comparison on scales (alphabetical)

### Iteration 9: Element Widget
- HTML content widgets
- SVG graphics support
- Drag images from browser
- Attribute editing

---

## Long-Term Roadmap (Original ToonTalk Features)

These features restore the full experience of the original 1995 ToonTalk:

### Phase A: Houses & City (Concurrency)
- House widget (process container)
- City view (multiple houses)
- Enter/exit house navigation
- Houses receive input via birds

### Phase B: Trucks & Bombs (Process Management)
- Truck widget (process spawner)
- Drop robot+box in truck → new house
- Bomb widget (process terminator)
- Clean process shutdown

### Phase C: Helicopter & Navigation
- Helicopter for flying between houses
- City map view
- Street addresses
- Bird flight paths visible in city

### Phase D: Additional Tools
- Pumpy (bike pump) for resizing
- Notebook for program storage/modules
- Team of robots (alternative behaviors)

---

## Future Innovations (Beyond Original ToonTalk)

### 3D Graphics Mode
- Three.js integration
- 3D widget rendering
- Physics-based animations
- VR/AR exploration
- **3D Z-Plane Debugging**: Render z-planes as semi-transparent layers spread apart in 3D when viewed at angle, making stacking order visually clear for debugging and potentially useful for 3D "programming" by users

### Design Pattern Library
- Pre-built robot patterns (Observer, Iterator, etc.)
- Visual representation of patterns
- Drag-and-drop pattern application

### Cross-Language Concepts
- Inspired by Alice, Scratch, Blockly
- Visual data flow
- State machine visualization

### Collaboration Features
- Real-time multi-user editing
- Shared workspaces
- Classroom mode for educators

### AI-Assisted Programming
- Natural language robot training
- Suggested actions during training
- Error detection and hints

---

## Dependencies Between Features

```
                    ┌─────────────────┐
                    │  Current State  │
                    │  (Numbers, Box, │
                    │  Robot, Tools)  │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
            ▼                ▼                ▼
    ┌───────────────┐ ┌───────────────┐ ┌───────────────┐
    │  Bird/Nest    │ │   Pattern     │ │  Persistence  │
    │  (Iter 1)     │ │   Matching    │ │  (Iter 3)     │
    └───────┬───────┘ │   (Iter 2)    │ └───────┬───────┘
            │         └───────┬───────┘         │
            │                 │                 │
            └────────┬────────┘                 │
                     │                          │
                     ▼                          │
            ┌───────────────┐                   │
            │   Sensors     │                   │
            │   (Iter 4)    │                   │
            └───────┬───────┘                   │
                    │                           │
                    ▼                           │
            ┌───────────────┐                   │
            │  Houses/City  │◄──────────────────┘
            │  (Phase A)    │  (persistence enables
            └───────┬───────┘   saving complex programs)
                    │
            ┌───────┴───────┐
            │               │
            ▼               ▼
    ┌───────────────┐ ┌───────────────┐
    │ Trucks/Bombs  │ │  Helicopter   │
    │  (Phase B)    │ │  (Phase C)    │
    └───────────────┘ └───────────────┘
```

### Persistence Implementation Phases

```
┌─────────────────────────────────────────────────────────────────┐
│                    Part 1: UI Components                        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ WorkspaceButton │─▶│  WorkspaceMenu  │─▶│   SaveDialog    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Part 2: Storage Backends                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ LocalStorage    │  │ File Import/    │  │ Bundled         │ │
│  │ (browser)       │  │ Export (share)  │  │ Examples        │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Part 3: Documentation                        │
│  ┌─────────────────┐  ┌─────────────────┐                      │
│  │ Description in  │  │ Info Panel      │                      │
│  │ workspace meta  │─▶│ on load         │                      │
│  └─────────────────┘  └─────────────────┘                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Success Criteria

### MVP (ToonTalk Reborn Parity)
- [ ] All basic widgets: Number, Box, Text, Robot, Bird, Nest, Scales
- [ ] Tools: Wand, Vacuum
- [ ] Robot training and execution with pattern matching
- [ ] Save/load workspaces
- [ ] 5+ example programs demonstrating capabilities

### Extended MVP
- [ ] Sensors for keyboard/mouse input
- [ ] Backside configuration views
- [ ] Watched robot execution
- [ ] Import ToonTalk Reborn JSON format

### Full Release
- [ ] All original ToonTalk features (Houses, Trucks, Bombs, Helicopter)
- [ ] Modern 3D graphics option
- [ ] Sound effects and optional speech
- [ ] Comprehensive documentation and tutorials

---

## Code Quality Maintenance

### sw-checklist Standards
Run `sw-checklist .` before commits. Thresholds:
- Functions: max 50 LOC (warn >25)
- Modules: max 7 functions (warn >4)
- Crates: max 7 modules (warn >4)
- Projects: max 7 crates (warn >4)

### Refactoring Triggers
When approaching limits, split proactively:
- Large function → Extract helper functions
- Large module → Split by responsibility
- Large crate → Extract new crate

See the "Code Quality Refactoring Plan" section at the end for specific guidance.

---

## Appendix: Code Quality Refactoring Plan

This section tracks specific refactoring needs identified by sw-checklist.

### Priority Refactoring Tasks

| Priority | Issue | Action |
|----------|-------|--------|
| 1 | app.rs is large | Already split into app/, ops/, robot_exec/, widget_item/ modules |
| 2 | help_panel.rs has many functions | Split into basics.rs, advanced.rs |
| 3 | robot.rs has many functions | Split into robot/, types.rs, ops.rs, mutators.rs |
| 4 | scales.rs has many functions | Split into scales/, ops.rs, mutators.rs |

### Completed Refactoring
- ✅ App component modularized (callbacks, render, ops)
- ✅ Help panel split (basics, advanced, messaging)
- ✅ Robot module split (types, ops, mutators)
- ✅ Scales module split (ops, mutators)
- ✅ Box tests split (creation, resize, erased)
- ✅ Bird rendering split from widget_impl
- ✅ Nest rendering split from widget_impl

---

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
