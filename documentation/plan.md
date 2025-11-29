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
| Persistence (Save/Load) | High | Essential for user experience |
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

### Iteration 3: Persistence & Sharing

**Goal**: Save and load workspaces so users can continue their work.

#### 3.1 JSON Serialization
- [ ] Define JSON schema for all widgets
- [ ] Implement `serialize()` for each widget type
- [ ] Implement `deserialize()` for each widget type
- [ ] Handle widget references (birds→nests)

#### 3.2 Layout Persistence
- [ ] Save widget positions to localStorage
- [ ] Load positions on startup
- [ ] Reset layout to defaults button
- [ ] Export/import layout as JSON

#### 3.3 Workspace Local Storage
- [ ] Save full workspace to localStorage
- [ ] Load workspace from localStorage
- [ ] Auto-save on changes (debounced)
- [ ] Workspace naming/listing

#### 3.4 File Export/Import
- [ ] Export workspace to JSON file
- [ ] Import workspace from JSON file
- [ ] Drag-drop file to import

#### 3.4 URL Sharing (Optional)
- [ ] Encode small programs in URL
- [ ] Decode and load from URL parameter

---

## Medium-Term Roadmap (Iterations 4-8)

### Iteration 4: Sensors & Events
- Keyboard sensor (key press detection)
- Mouse sensor (click, position)
- Touch sensor (mobile support)
- Sensors trigger robot activation

### Iteration 5: Backside & Configuration
- Flip widget to see backside
- Robot backside shows conditions
- Number backside shows operator selection
- Box backside shows labels

### Iteration 6: Robot Execution Polish
- Watched execution (step-by-step with animation)
- Speed control slider
- Visual highlighting of current action
- Robot chaining (sequential robots)

### Iteration 7: Text Operations
- Text explosion (text on box → letters in holes)
- Text joining (box contents → concatenated text)
- Text comparison on scales (alphabetical)

### Iteration 8: Element Widget
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
    └───────┬───────┘ │   (Iter 2)    │ └───────────────┘
            │         └───────┬───────┘
            │                 │
            └────────┬────────┘
                     │
                     ▼
            ┌───────────────┐
            │   Sensors     │
            │   (Iter 4)    │
            └───────┬───────┘
                    │
                    ▼
            ┌───────────────┐
            │  Houses/City  │
            │  (Phase A)    │
            └───────┬───────┘
                    │
            ┌───────┴───────┐
            │               │
            ▼               ▼
    ┌───────────────┐ ┌───────────────┐
    │ Trucks/Bombs  │ │  Helicopter   │
    │  (Phase B)    │ │  (Phase C)    │
    └───────────────┘ └───────────────┘
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
