# tt-rs Implementation Plan

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

This document outlines the phased implementation plan for tt-rs. The plan is structured into milestones with clear deliverables, allowing incremental progress while maintaining a working system at each stage.

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution.

## Phase 1: Foundation

**Goal**: Establish the project infrastructure and core abstractions.

### 1.1 Project Setup

- [ ] Configure Cargo.toml with dependencies
  - yew (UI framework)
  - wasm-bindgen (JS interop)
  - serde/serde_json (serialization)
  - gloo (browser APIs)
  - web-sys (DOM APIs)
  - js-sys (JS types)
  - num-rational (rational numbers)
  - thiserror (error handling)

- [ ] Set up build pipeline
  - trunk for development server
  - wasm-pack for production builds
  - wasm-opt for optimization

- [ ] Configure project structure
  - Create module hierarchy per architecture.md
  - Set up test infrastructure

### 1.2 Core Traits and Types

- [ ] Define `Widget` trait
  - `get_type_name()`
  - `id()`
  - `copy()`
  - `matches()`
  - `serialize()`/`deserialize()`
  - `render()`

- [ ] Define `WidgetId` and `WidgetRef`

- [ ] Define `MatchResult` and `Bindings`

- [ ] Define `WidgetPath` enum

- [ ] Define error types

### 1.3 Basic Yew Application

- [ ] Create main App component
- [ ] Create Workspace component
- [ ] Implement basic drag-and-drop
- [ ] Set up state management context

## Phase 2: Numbers and Boxes

**Goal**: Implement the first visual widgets with interactions.

### 2.1 Number Widget

- [x] Implement `Number` struct
  - [x] Rational number storage (num-rational)
  - [x] Arithmetic operator (+, -, *, /)
  - [ ] Erasure levels

- [x] Implement number rendering
  - [x] CSS-based display
  - [x] Operator indicator (for tools)
  - [x] Proper formatting

- [x] Implement number operations
  - [x] Drop number on number (apply operator)
  - [ ] Edit operator (backside)

- [ ] Implement number matching
  - [ ] Exact match
  - [ ] Sign match (erased)
  - [ ] Type match (fully erased)

### 2.2 Box Widget

- [x] Implement `BoxWidget` struct
  - [x] Configurable hole count
  - [x] Contents array
  - [ ] Horizontal/vertical orientation

- [x] Implement box rendering
  - [x] 3D-effect container
  - [x] Dividers between holes
  - [x] Contents rendering

- [x] Implement box interactions
  - [x] Drop widget into hole
  - [x] Remove widget from hole (via vacuum tool)
  - [x] Resize box (add/remove holes via keyboard 0-9)

- [ ] Implement box matching
  - [ ] Structure matching
  - [ ] Contents matching (recursive)
  - [ ] Partial matches

### 2.3 Box Operations (Original ToonTalk)

- [x] Keyboard-based hole control
  - [x] Type 0-9 while holding box to set hole count
  - [x] Excess contents drop when reducing holes
  - [x] Zero-hole box support

- [x] Box joining
  - [x] Drop box on edge of another box to join
  - [x] Combined box has sum of holes

- [x] Box splitting
  - [x] Drop box on number to split
  - [x] e.g., 5-hole box on 3 → 3-hole + 2-hole boxes
  - [ ] Array indexing via splitting

- [ ] Box labels
  - [ ] Add descriptive labels under holes
  - [ ] Labels are visual only (robots ignore them)

- [ ] Text explosion
  - [ ] Drop text on blank box → individual letter holes
  - [ ] e.g., "cat" on blank box → 3 holes with "c", "a", "t"

### 2.4 Scales in Box Holes (Comparison)

- [ ] Scales as comparison operator
  - [ ] Place scales in middle hole of box
  - [ ] Scales compare values in adjacent holes
  - [ ] Tilt toward larger number or later alphabetically

- [ ] Scales states
  - [ ] Balanced (values equal)
  - [ ] Left-heavy (left value larger)
  - [ ] Right-heavy (right value larger)
  - [ ] Tottering (adjacent to empty hole)

- [ ] Scales interaction with robots
  - [ ] Robot can check scales state for conditionals
  - [ ] Keyboard controls: +/- to cycle states, . to freeze, space to re-compare

### 2.5 Persistence (Basic)

- [ ] Implement JSON serialization
  - Number to/from JSON
  - Box to/from JSON
  - Workspace to/from JSON

- [ ] Local storage integration
  - Save workspace
  - Load workspace
  - Auto-save

## Phase 3: Robot Training

**Goal**: Implement the core programming-by-demonstration system.

### 3.1 Robot Widget

- [x] Implement `Robot` struct
  - [ ] Conditions (frontside, backside)
  - [x] Action body
  - [x] State machine (Idle/Training/Working)
  - [ ] Chaining (next robot)

- [x] Implement robot rendering
  - [x] Idle state
  - [x] Training state
  - [x] Working state
  - [x] Different visual states (CSS animations)

### 3.2 Action Recording

- [x] Implement `ActionSequence` (Vec<Action>)

- [x] Implement action types
  - [x] PickUp
  - [x] Drop
  - [x] Copy
  - [x] Remove
  - [x] ApplyArithmetic (number operations)

- [x] Implement path recording
  - [x] Widget paths (widget:id)
  - [x] Box hole paths (box:id:hole:index)
  - [ ] Handle nested structures

- [x] Training mode UI
  - [x] Click to start/stop training
  - [x] Visual feedback during training (yellow glow animation)

### 3.3 Pattern Matching

- [ ] Implement condition matching
  - Frontside pattern match
  - Backside requirements check
  - Binding extraction

- [ ] Implement erasure
  - Vacuum tool integration
  - Multi-level erasure
  - Visual feedback for erased widgets

### 3.4 Robot Execution

- [x] Implement execution engine
  - [x] Step through actions
  - [x] Path resolution (parse_widget_path, parse_box_hole_path)
  - [ ] Binding application

- [ ] Watched execution
  - Step-by-step with animation
  - Speed control
  - Visual highlighting

- [x] Unwatched execution
  - [x] Full speed execution
  - [x] Batch updates

## Phase 4: Bird/Nest Messaging

**Goal**: Implement asynchronous communication between widgets.

### 4.1 Nest Widget

- [ ] Implement `Nest` struct
  - Contents queue
  - Waiting robots list
  - Bird reference

- [ ] Implement nest rendering
  - Egg animation
  - Contents indicator
  - Hatching animation

### 4.2 Bird Widget

- [ ] Implement `Bird` struct
  - Nest reference
  - Color
  - State (idle, carrying, flying)

- [ ] Implement bird rendering
  - Animated sprite
  - Flight path visualization
  - Color customization

### 4.3 Message Delivery

- [ ] Implement delivery animation
  - Bird picks up item
  - Flight arc calculation
  - Nest receives item

- [ ] Integrate with robot execution
  - "Give to bird" action
  - Wait for nest action
  - Concurrent deliveries

## Phase 5: Concurrency and Processes

**Goal**: Implement houses, trucks, and bombs for multiple concurrent processes.

### 5.1 House Widget

- [ ] Implement `House` struct
  - Contains robots and boxes
  - Unique address in city
  - Visual appearance (picture)

- [ ] Implement house rendering
  - 3D/SVG house graphic
  - Interior workspace view
  - Status indicator (active/idle)

- [ ] House interactions
  - Enter house (view interior)
  - Exit house (return to city)
  - Give items to house

### 5.2 Truck Widget (Process Spawner)

- [ ] Implement `Truck` struct
  - Cargo hold (robot + box)
  - Ready/not-ready state
  - Optional notebook (module)
  - Optional picture (house appearance)

- [ ] Implement truck behavior
  - Drop robot into truck
  - Drop box into truck
  - Truck drives off when ready
  - Creates new house at empty location

- [ ] Implement truck rendering
  - SVG truck graphic
  - Cargo indicators
  - Driving animation

### 5.3 Bomb Widget (Process Terminator)

- [ ] Implement `Bomb` struct
  - Target house reference
  - Armed/disarmed state

- [ ] Implement bomb behavior
  - Drop on house to arm
  - Explode and destroy house
  - Clean up process resources

- [ ] Implement bomb rendering
  - SVG bomb graphic
  - Explosion animation

### 5.4 City View

- [ ] Implement city overview
  - Grid/map of houses
  - Street addresses
  - Bird flight paths visible

- [ ] Helicopter navigation (optional)
  - Fly over city
  - Land at house
  - Pick up and move items

## Phase 6: Additional Widgets

**Goal**: Complete the widget set.

### 6.1 Scale Widget

- [x] Implement `Scale` struct
  - [x] Left/right sides
  - [x] Comparison logic (numeric)

- [x] Implement scale rendering
  - [x] Balance beam (SVG image)
  - [x] Tipping animation (CSS classes)
  - [ ] Physics-based movement (future enhancement)

- [x] Implement scale behavior (basic)
  - [x] Compare dropped numbers
  - [x] Numeric comparison
  - [ ] Text comparison (alphabetical)
  - [ ] Scales in box holes (see 2.4)

### 6.2 Sensor Widget

- [ ] Implement `Sensor` struct
  - Event type selection
  - Attribute binding

- [ ] Implement sensor rendering
  - Visual indicator of event type
  - Active/inactive states

- [ ] Event integration
  - Keyboard events
  - Mouse events
  - Touch events

### 6.3 Element Widget

- [ ] Implement `Element` struct
  - HTML content
  - Attributes
  - Children

- [ ] Implement element rendering
  - Safe HTML rendering
  - SVG support
  - Attribute editing

- [ ] Drag from browser
  - Import images
  - Import text
  - Handle external content

### 6.4 Tools

- [x] Implement Wand (magic wand)
  - [x] Copy on touch
  - [x] Visual feedback

- [x] Implement Vacuum (Dusty)
  - [x] Remove mode (deletes widgets)
  - [ ] Erase mode (multi-level erasure)
  - [x] Visual feedback

## Phase 7: Modern Graphics

**Goal**: Upgrade visuals with Three.js and advanced animations.

### 7.1 Three.js Integration

- [ ] Set up Three.js bindings
  - Scene management
  - Camera control
  - Renderer integration

- [ ] 3D widget rendering
  - Numbers with depth
  - Boxes with volume
  - Shadows and lighting

### 7.2 Advanced Animations

- [ ] Bird flight paths
  - Bezier curve paths
  - Wing flapping
  - Carrying physics

- [ ] Robot animations
  - Working animation
  - Completion celebration
  - Error indication

- [ ] Scale physics
  - Smooth tipping
  - Bounce effect
  - Weight visualization

### 7.3 Visual Polish

- [ ] Particle effects
  - Copy sparkles
  - Erase dust
  - Nest hatching

- [ ] Sound effects
  - Widget interactions
  - Robot events
  - Ambient sounds (optional)

## Phase 8: User Experience

**Goal**: Polish the user interface and interactions.

### 8.1 Workspace Improvements

- [ ] Zoom and pan
- [ ] Grid snapping (optional)
- [ ] Multiple workspaces
- [ ] Workspace tabs

### 8.2 Editing Features

- [ ] Undo/redo system
- [ ] Cut/copy/paste
- [ ] Multi-select operations
- [ ] Context menus

### 8.3 Help System

- [ ] Tooltips
- [ ] Guided tours
- [ ] Example programs
- [ ] Interactive manual

## Phase 9: Audio and Speech

**Goal**: Add the "talking" capabilities.

### 9.1 Sound Effects

- [ ] Interaction sounds
  - Widget pickup/drop
  - Robot start/stop
  - Bird flight

- [ ] Feedback sounds
  - Success/error
  - Pattern match
  - Completion

### 9.2 Text-to-Speech

- [ ] Pre-generated assets
  - Common instructions
  - Widget descriptions
  - Error messages

- [ ] Dynamic TTS (optional)
  - Browser TTS API integration
  - Custom voice settings
  - Language support

## Phase 10: Ecosystem

**Goal**: Enable sharing and community features.

### 10.1 Import/Export

- [ ] ToonTalk Reborn format import
- [ ] Export to file
- [ ] Import from file

### 10.2 Sharing

- [ ] URL-based sharing
- [ ] Embed code generation
- [ ] Screenshot/GIF export

### 10.3 Cloud Storage

- [ ] Account system (optional)
- [ ] Cloud save/load
- [ ] Program gallery

## Phase 11: Documentation and Release

**Goal**: Prepare for public release.

### 11.1 Documentation

- [ ] User guide
- [ ] Educator guide
- [ ] API documentation (for contributors)
- [ ] Tutorial videos

### 11.2 Testing

- [ ] Complete unit test coverage
- [ ] Integration test suite
- [ ] Cross-browser testing
- [ ] Performance benchmarks

### 11.3 Release

- [ ] Production build optimization
- [ ] Deploy to web hosting
- [ ] Create landing page
- [ ] Announce release

## Dependencies Between Phases

```
Phase 1 ------+---------------+
              |               |
              v               |
Phase 2 ------+---------------+
              |               |
              v               |
Phase 3 ------+---------------+
              |               |
              v               |
Phase 4 ------+---------------+
              |               |
              v               |
Phase 5 ------+---------------+  (Concurrency)
                              |
Phase 6 ----------------------+  (Additional Widgets)
                              |
Phase 7 ----------------------+  (Modern Graphics)
                              |
Phase 8 ----------------------+  (User Experience)
                              |
Phase 9 ----------------------+  (Audio and Speech)
                              |
Phase 10 ---------------------+  (Ecosystem)
                              |
                              v
                        Phase 11
```

Notes:
- Phases 1-5 are sequential (each depends on previous)
- Phase 5 (Concurrency) can partially overlap with Phase 4 (Bird/Nest)
- Phases 6-10 can be worked on in parallel after Phase 5
- Phase 11 (Documentation and Release) requires all previous phases

## Success Criteria

### MVP (Phases 1-4)

- Numbers can be created, moved, and combined
- Boxes can hold and organize widgets
- Robots can be trained by demonstration
- Robots can execute recorded actions
- Birds can carry messages to nests
- Work can be saved and loaded

### Extended MVP (Phase 5)

- Houses contain working robots
- Multiple concurrent processes (houses) run simultaneously
- Trucks spawn new houses (processes)
- Bombs terminate houses (processes)
- City view shows all running processes

### Full Release (All Phases)

- All widgets from original system implemented
- Modern 3D/SVG graphics
- Smooth animations throughout
- Sound and optional speech
- Import from ToonTalk Reborn
- Shareable programs
- Comprehensive documentation

## Risk Management

### Technical Risks

| Risk | Mitigation |
|------|------------|
| Three.js integration complexity | Start with SVG-only, add Three.js incrementally |
| WASM performance issues | Profile early, optimize critical paths |
| Browser compatibility | Test early and often on target browsers |
| Complex animation system | Use CSS animations where possible, programmatic only when needed |

### Project Risks

| Risk | Mitigation |
|------|------------|
| Scope creep | Stick to MVP first, defer enhancements |
| Single developer bus factor | Comprehensive documentation, clean code |
| Motivation loss | Small incremental milestones, visible progress |

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
