# Physical Design: Component Architecture

This document defines the physical architecture for tt-rs, organized around strict modularity constraints and a phased feature reveal system inspired by [Racket's HtDP Teaching Languages](https://docs.racket-lang.org/htdp-langs/index.html).

## Strict Modularity Constraints

All code must adhere to these limits:

| Level | Max Items | Warning At |
|-------|-----------|------------|
| Functions per module | 4 | 3 |
| Modules per crate | 4 | 3 |
| Crates per component | 4 | 3 |
| Components per repo | unlimited | - |

These constraints enforce separation of concerns and make the codebase navigable.

---

## Architectural Layers

```
┌─────────────────────────────────────────────────────────────────────┐
│                        APPLICATION LAYER                            │
│  (User-facing applications built on platform)                       │
├─────────────────────────────────────────────────────────────────────┤
│  app-beginner    app-intermediate    app-advanced    app-full       │
│  (Phase 1)       (Phase 2-3)         (Phase 4-5)     (Phase 6+)     │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        FEATURE LAYER                                │
│  (Domain-specific features, each a component)                       │
├─────────────────────────────────────────────────────────────────────┤
│  values    containers    comparison    agents    messaging    ...   │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        PLATFORM LAYER                               │
│  (Reusable infrastructure, independent of ToonTalk concepts)        │
├─────────────────────────────────────────────────────────────────────┤
│  core    interaction    persistence    ui-kit    events             │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Platform Layer Components

These provide reusable infrastructure with no ToonTalk-specific knowledge.

### `core` - Fundamental Abstractions
- **Purpose**: Traits, IDs, result types
- **Crates**:
  - `tt-rs-traits` - Widget trait, Tool trait
  - `tt-rs-ids` - WidgetId, unique ID generation
  - `tt-rs-match` - MatchResult, pattern matching abstractions

### `interaction` - User Input Handling
- **Purpose**: Drag-and-drop, hit testing, gesture recognition
- **Crates**:
  - `tt-rs-dnd` - Drag-and-drop state machine
  - `tt-rs-hit` - Hit testing utilities
  - `tt-rs-gestures` - Touch/mouse gesture recognition

### `persistence` - Save/Load Infrastructure
- **Purpose**: Serialization, storage backends
- **Crates**:
  - `tt-rs-serial` - JSON serialization for widgets
  - `tt-rs-storage` - localStorage, file I/O
  - `tt-rs-programs` - Program listing, naming, metadata

### `ui-kit` - Reusable UI Components
- **Purpose**: Generic UI elements
- **Crates**:
  - `tt-rs-panels` - Slide-out panels, modals
  - `tt-rs-tooltips` - Hover tooltips
  - `tt-rs-palette` - Copy source palettes

### `events` - Event System
- **Purpose**: Application-wide event bus
- **Crates**:
  - `tt-rs-bus` - Event publishing/subscription
  - `tt-rs-sensors` - Keyboard, mouse, touch sensors

---

## Feature Layer Components

Domain-specific features organized by programming concept, not by "widget".

### `values` - Primitive Data Types
- **Purpose**: Numbers, text, and other atomic values
- **Crates**:
  - `tt-rs-number` - Rational arithmetic
  - `tt-rs-text` - Text strings
  - `tt-rs-element` - HTML/SVG elements (future)

### `containers` - Data Structures
- **Purpose**: Things that hold other things
- **Crates**:
  - `tt-rs-box` - Multi-hole containers
  - `tt-rs-nest` - Message queues (holds messages for birds)

### `comparison` - Value Comparison
- **Purpose**: Comparing and ordering values
- **Crates**:
  - `tt-rs-scales` - Visual balance scale

### `agents` - Autonomous Behavior
- **Purpose**: Things that act on their own
- **Crates**:
  - `tt-rs-robot` - Programmable agent
  - `tt-rs-team` - Multiple robots (future)

### `messaging` - Inter-Process Communication
- **Purpose**: Asynchronous message passing (actor model)
- **Crates**:
  - `tt-rs-bird` - Message carrier
  - `tt-rs-channel` - Bird-nest channel management

### `tools` - User Manipulation
- **Purpose**: Tools the user wields
- **Crates**:
  - `tt-rs-wand` - Copy tool
  - `tt-rs-vacuum` - Remove tool
  - `tt-rs-pump` - Resize tool (future)

### `processes` - Process Management (Future)
- **Purpose**: Concurrent execution
- **Crates**:
  - `tt-rs-house` - Process container
  - `tt-rs-truck` - Process spawner
  - `tt-rs-bomb` - Process terminator

### `patterns` - Pattern Matching
- **Purpose**: Erasure and matching logic
- **Crates**:
  - `tt-rs-erasure` - Erasure levels and erased values
  - `tt-rs-matcher` - Pattern matching engine

---

## Application Layer Components

Multiple applications exposing progressive feature sets.

### `app-beginner` - Phase 1 Features
- **Audience**: First-time users, young children
- **Features exposed**:
  - Numbers with arithmetic (+, -, *, /)
  - Boxes (containers)
  - Wand (copy)
  - Vacuum (remove)
- **Concepts taught**:
  - Data representation
  - Containers/organization
  - Basic manipulation

### `app-intermediate` - Phases 2-3 Features
- **Audience**: Users comfortable with basics
- **Features exposed**:
  - All beginner features
  - Scales (comparison)
  - Robot (training mode)
  - Bird/Nest (messaging)
- **Concepts taught**:
  - Comparison and conditions
  - Programming by demonstration
  - Message passing

### `app-advanced` - Phases 4-5 Features
- **Audience**: Confident users
- **Features exposed**:
  - All intermediate features
  - Pattern matching with erasure
  - Sensors (keyboard, mouse)
  - Recursive patterns
- **Concepts taught**:
  - Abstraction and generalization
  - Event-driven programming
  - Recursion

### `app-full` - Phase 6+ Features
- **Audience**: Power users, educators
- **Features exposed**:
  - All features
  - Houses (processes)
  - Trucks (spawning)
  - Bombs (termination)
  - City navigation
- **Concepts taught**:
  - Concurrency
  - Process management
  - Distributed computation

---

## Phased Feature Reveal System

Inspired by [Racket's How to Design Programs languages](https://docs.racket-lang.org/htdp-langs/index.html), features are revealed progressively.

### Phase 1: Beginning Student (Data & Manipulation)

| Feature | Widget | Concept |
|---------|--------|---------|
| Numbers | Number | Data representation |
| Text | Text | String data |
| Containers | Box | Data structures |
| Copy | Wand | Duplication |
| Remove | Vacuum | Deletion |

**Learning goals**: Understand that programs work with data, data can be organized, and the user can manipulate the workspace.

### Phase 2: Beginning with Communication

| Feature | Widget | Concept |
|---------|--------|---------|
| All Phase 1 | - | - |
| Comparison | Scales | Ordering/conditions |
| Message sending | Bird | Output |
| Message receiving | Nest | Input |

**Learning goals**: Data can be compared, and parts of a program can communicate.

### Phase 3: Intermediate (Automation)

| Feature | Widget | Concept |
|---------|--------|---------|
| All Phase 2 | - | - |
| Training | Robot | Recording actions |
| Playback | Robot | Repeating actions |

**Learning goals**: Actions can be recorded and replayed automatically.

### Phase 4: Intermediate with Abstraction

| Feature | Widget | Concept |
|---------|--------|---------|
| All Phase 3 | - | - |
| Erasure | Vacuum+ | Generalization |
| Pattern matching | Robot+ | Abstraction |

**Learning goals**: Programs can work with classes of data, not just specific values.

### Phase 5: Advanced (Events & Recursion)

| Feature | Widget | Concept |
|---------|--------|---------|
| All Phase 4 | - | - |
| Sensors | Sensor | External events |
| Self-messaging | Bird→own Nest | Recursion |

**Learning goals**: Programs can respond to external events and repeat operations.

### Phase 6: Full (Concurrency)

| Feature | Widget | Concept |
|---------|--------|---------|
| All Phase 5 | - | - |
| Processes | House | Isolation |
| Spawning | Truck | Parallelism |
| Termination | Bomb | Cleanup |

**Learning goals**: Multiple independent processes can run simultaneously.

---

## Persistence Architecture

Save/load is essential for real use. Architecture supports versioned file formats.

### File Format Versioning

File extensions indicate the minimum phase required to load:

| Extension | Phase | Features | App Compatibility |
|-----------|-------|----------|-------------------|
| `.tt1` | 1 | Numbers, Text, Box, Wand, Vacuum | All apps |
| `.tt2` | 2 | + Scales, Bird, Nest | tt2+ apps |
| `.tt3` | 3 | + Robot (training/playback) | tt3+ apps |
| `.tt4` | 4 | + Erasure, Pattern matching | tt4+ apps |
| `.tt5` | 5 | + Sensors, Recursion | tt5+ apps |
| `.tt6` | 6 | + House, Truck, Bomb | tt6 app only |

### Program Representation
```json
{
  "tt_version": 3,
  "id": "uuid-here",
  "name": "My Counter",
  "description": "Counts from 1 to 10",
  "created": "2025-11-29T10:30:00Z",
  "modified": "2025-11-29T11:45:00Z",
  "workspace": { /* serialized widget state */ }
}
```

The `tt_version` field is the authoritative version. The file extension should match but the runtime trusts the embedded metadata.

### Loading Behavior

When loading a file:

1. **Parse JSON** and extract `tt_version`
2. **Check compatibility**:
   - If `tt_version` ≤ current app phase → load normally
   - If `tt_version` > current app phase → offer choices:
     - "This program uses features from ToonTalk Phase X"
     - Option A: "Switch to ToonTalk Phase X" (auto-upgrade runtime)
     - Option B: "Cancel"
3. **Validate widgets** - ensure all widget types are supported
4. **Restore workspace** - deserialize and display

### Storage Operations
- **Save**: Serialize workspace to JSON with current app's `tt_version`
- **List**: Enumerate saved programs showing name, version badge, modified date
- **Load**: Deserialize with version checking (see above)
- **Export**: Download as `.ttN` file (N = version)
- **Import**: Upload `.tt*` file with version validation
- **Share**: Generate URL with embedded program (small programs, includes version)

### Version Badge Display

In the program list UI, show version badges:

```
┌──────────────────────────────────────┐
│ 📁 My Programs                       │
├──────────────────────────────────────┤
│ [tt1] Simple Addition     Nov 28     │
│ [tt2] Message Passer      Nov 29     │
│ [tt3] Doubling Robot      Nov 29     │
│ [tt4] Generic Sorter      Nov 29     │
└──────────────────────────────────────┘
```

Users can immediately see which programs are compatible with their current app level.

---

## Migration Plan from Current Structure

### Current State
```
components/
├── core/       (1 crate)  ✓ OK
├── widgets/    (7 crates) ✗ Too many
├── dnd/        (2 crates) ✓ OK
├── state/      (1 crate)  ✓ OK
├── handlers/   (3 crates) ✓ OK
├── commands/   (1 crate)  ✓ OK
└── app/        (1 crate)  ✓ OK
```

### Target State
```
components/
├── core/           (3 crates: traits, ids, match)
├── interaction/    (3 crates: dnd, hit, gestures) ← merge dnd+handlers
├── persistence/    (3 crates: serial, storage, programs) ← new
├── ui-kit/         (3 crates: panels, tooltips, palette) ← extract from dnd
├── values/         (2 crates: number, text) ← from widgets
├── containers/     (2 crates: box, nest) ← from widgets + new
├── comparison/     (1 crate: scales) ← from widgets
├── agents/         (1 crate: robot) ← from widgets
├── messaging/      (2 crates: bird, channel) ← new
├── tools/          (2 crates: wand, vacuum) ← from widgets
├── patterns/       (2 crates: erasure, matcher) ← new
├── state/          (1 crate) ✓ keep
├── commands/       (1 crate) ✓ keep
├── app-beginner/   (1 crate) ← new, replace app
├── app-intermediate/ (1 crate) ← new
└── app-advanced/   (1 crate) ← new
```

### Migration Steps

1. **Create `values/`**: Move number, text from widgets
2. **Create `containers/`**: Move box from widgets, add nest
3. **Create `comparison/`**: Move scales from widgets
4. **Create `agents/`**: Move robot from widgets
5. **Create `tools/`**: Move wand, vacuum from widgets
6. **Delete `widgets/`**: Now empty
7. **Create `messaging/`**: Add bird, channel
8. **Merge `dnd/` + `handlers/` → `interaction/`**
9. **Extract UI components → `ui-kit/`**
10. **Create `persistence/`**: New component
11. **Create phase-specific apps**

---

## Dependency Rules

### Platform Layer
- Core depends on nothing (leaf)
- Other platform components depend only on core

### Feature Layer
- Feature components depend on platform layer only
- Feature components do NOT depend on each other
- Exception: messaging depends on containers (nest)

### Application Layer
- Apps depend on feature components they expose
- Apps depend on platform layer
- Apps do NOT depend on each other

```
app-beginner → values, containers, tools, ui-kit, interaction, core
app-intermediate → app-beginner deps + comparison, agents, messaging
app-advanced → app-intermediate deps + patterns, events
app-full → all feature components
```

---

## Benefits of This Architecture

1. **Strict modularity**: No component grows unbounded
2. **Clear dependencies**: Easy to understand what depends on what
3. **Phased learning**: Users aren't overwhelmed
4. **Independent development**: Teams can work on components separately
5. **Future multi-repo**: Components can become separate repos
6. **Testing isolation**: Each component tested independently
7. **Feature flags**: Apps select which features to include

---

## References

- [Racket HtDP Teaching Languages](https://docs.racket-lang.org/htdp-langs/index.html)
- [How to Design Programs](https://htdp.org/)
- Original ToonTalk by Ken Kahn

## License

BSD 3-Clause License. See COPYRIGHT and LICENSE files.
