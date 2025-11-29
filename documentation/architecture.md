# tt-rs Architecture

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

tt-rs is a modern Rust/WebAssembly reimplementation of ToonTalk, an interactive visual programming environment. This document describes the actual implemented architecture as of November 2025.

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution details.

## Design Philosophy

### Core Principles

1. **Rust for All Logic**: All presentation and domain logic in Rust, compiled to WebAssembly. Minimal JavaScript (only browser API bindings).

2. **Multi-Component Architecture**: Independent Cargo workspaces for separation of concerns and sw-checklist compliance.

3. **Modern Rendering**: SVG and CSS for 2D graphics, with Three.js integration planned for 3D.

4. **Type Safety**: Leverage Rust's type system for compile-time guarantees.

5. **Modularity**: Follow sw-checklist limits (7 functions/module, 7 modules/crate, 7 crates/project).

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                       Browser Environment                            │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    App Component (WASM Entry)                │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │   │
│  │  │   Yew App   │ │  Callbacks  │ │  Robot Execution    │   │   │
│  │  │  Component  │ │  Handlers   │ │  Engine             │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    DnD Component                             │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │   │
│  │  │  Draggable  │ │  Copy       │ │  UI Components      │   │   │
│  │  │  System     │ │  Source     │ │  (Help, Tooltip)    │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Widgets Component                         │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────────┐   │   │
│  │  │ Number  │ │   Box   │ │  Text   │ │ Robot, Scales   │   │   │
│  │  │         │ │         │ │         │ │ Wand, Vacuum    │   │   │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Core Component                            │   │
│  │  ┌─────────────────────────────────────────────────────┐   │   │
│  │  │  Widget Trait  │  WidgetId  │  MatchResult          │   │   │
│  │  └─────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
tt-rs/
├── CLAUDE.md                    # AI assistant guidance
├── COPYRIGHT                    # Attribution
├── LICENSE                      # BSD 3-Clause
├── README.md                    # Project overview
│
├── components/                  # Multi-component architecture
│   ├── core/                    # Core abstractions (no dependencies)
│   │   └── crates/
│   │       └── tt-rs-core/
│   │           └── src/
│   │               ├── lib.rs
│   │               ├── widget_id.rs
│   │               └── widget_trait.rs
│   │
│   ├── widgets/                 # Widget implementations
│   │   └── crates/
│   │       ├── tt-rs-number/    # Number widget
│   │       ├── tt-rs-text/      # Text widget
│   │       ├── tt-rs-box/       # Box widget
│   │       ├── tt-rs-robot/     # Robot widget
│   │       ├── tt-rs-scales/    # Scales widget
│   │       ├── tt-rs-wand/      # Wand tool
│   │       └── tt-rs-vacuum/    # Vacuum tool
│   │
│   ├── dnd/                     # Drag-and-drop and UI
│   │   └── crates/
│   │       ├── tt-rs-drag/      # Draggable, CopySource
│   │       └── tt-rs-ui/        # Help panel, tooltips, accordion
│   │
│   ├── state/                   # State management
│   │   └── crates/
│   │       └── tt-rs-state/     # Position, BoxContents, TrainingState
│   │
│   ├── handlers/                # Event handlers
│   │   └── crates/
│   │       └── tt-rs-hit-test/  # Hit testing utilities
│   │
│   ├── commands/                # Command pattern
│   │   └── crates/
│   │       └── tt-rs-commands/  # Move, Remove commands
│   │
│   └── app/                     # WASM entry point
│       └── crates/
│           └── tt-rs-app/
│               ├── index.html   # Trunk entry point
│               ├── favicon.ico
│               └── src/
│                   ├── lib.rs
│                   ├── demo.rs
│                   ├── state.rs
│                   ├── app/         # App component
│                   ├── ops/         # Operations
│                   ├── robot_exec/  # Robot execution
│                   ├── widget_item/ # Widget rendering
│                   └── box_state/   # Box state management
│
├── scripts/                     # Build and deploy scripts
│   ├── serve.sh                 # Development server (port 1140)
│   ├── build-all.sh            # Build + test + clippy + fmt
│   ├── build-release.sh        # Production build
│   └── check-all.sh            # Run all quality checks
│
├── docs/                        # GitHub Pages deployment (built output)
│
└── documentation/               # Project documentation
    ├── architecture.md          # This file
    ├── prd.md                   # Product requirements
    ├── design.md                # Design decisions
    ├── plan.md                  # Implementation roadmap
    └── learnings.md             # Solutions to issues encountered
```

## Dependency Flow

```
┌──────────────────────────────────────────────────────────────┐
│                         app                                   │
│  (WASM entry, Yew app, robot execution)                      │
└──────────────────────────────────────────────────────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
┌────────────────┐  ┌────────────────┐  ┌────────────────┐
│    handlers    │  │     state      │  │    commands    │
│   (hit-test)   │  │  (positions)   │  │  (move, etc)   │
└────────────────┘  └────────────────┘  └────────────────┘
         │                   │                   │
         └───────────────────┼───────────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │      dnd       │
                    │ (drag, UI)     │
                    └────────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │    widgets     │
                    │ (Number, Box,  │
                    │  Robot, etc)   │
                    └────────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │      core      │
                    │ (Widget trait, │
                    │  WidgetId)     │
                    └────────────────┘
```

## Core Abstractions

### Widget Trait

The fundamental abstraction all visual objects implement:

```rust
// components/core/crates/tt-rs-core/src/widget_trait.rs

pub trait Widget: std::fmt::Debug {
    /// Type name for identification ("number", "box", "robot", etc.)
    fn type_name(&self) -> &'static str;

    /// Unique identifier for this widget instance
    fn id(&self) -> WidgetId;

    /// Create a deep copy with a new ID
    fn copy(&self) -> Box<dyn Widget>;

    /// Pattern matching for robot conditions
    fn matches(&self, other: &dyn Widget) -> MatchResult;

    /// Render as Yew HTML
    fn render(&self) -> Html;

    /// Human-readable description
    fn description(&self) -> String;
}
```

### WidgetId

Unique identity for widget instances:

```rust
// components/core/crates/tt-rs-core/src/widget_id.rs

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
```

### MatchResult

Pattern matching results for robot conditions:

```rust
// components/core/crates/tt-rs-core/src/widget_trait.rs

pub enum MatchResult {
    Match,      // Pattern matches
    NoMatch,    // Pattern doesn't match
    // Future: Partial(Bindings) for extracted values
}
```

## Widget Implementations

### Number Widget

Rational arithmetic with operators:

```rust
// components/widgets/crates/tt-rs-number/

pub struct Number {
    numerator: i64,
    denominator: i64,
    operator: ArithOperator,  // Add, Subtract, Multiply, Divide
}

impl Number {
    pub fn apply_to(&self, target: &Number) -> Number {
        // Apply this number's operator to target
    }
}
```

### Box Widget

Container with configurable holes:

```rust
// components/widgets/crates/tt-rs-box/

pub struct ToonBox {
    id: WidgetId,
    holes: Vec<Hole>,
}

pub struct Hole {
    contents: Option<Box<dyn Widget>>,
}
```

### Robot Widget

Programmable agent trained by demonstration:

```rust
// components/widgets/crates/tt-rs-robot/

pub struct Robot {
    id: WidgetId,
    state: RobotState,
    actions: Vec<Action>,
}

pub enum RobotState {
    Idle,
    Training,
    Working,
}

pub enum Action {
    PickUp { path: String },
    Drop { target: String },
    Copy { source: String },
    Remove { path: String },
    ApplyArithmetic { source: String, target: String },
}
```

## Rendering

### Yew Components

All widgets render through Yew's `Html` type:

```rust
impl Widget for Number {
    fn render(&self) -> Html {
        html! {
            <div class="widget number">
                <span class="operator">{self.operator_symbol()}</span>
                <span class="value">{self.display_value()}</span>
            </div>
        }
    }
}
```

### CSS Styling

Widget styles in `assets/style.css`:

```css
.widget {
    position: absolute;
    cursor: grab;
    user-select: none;
}

.widget.number {
    background: linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%);
    border-radius: 8px;
    padding: 8px 16px;
}

.widget.dragging {
    transform: scale(1.05);
    box-shadow: 0 8px 16px rgba(0,0,0,0.2);
}
```

## State Management

### AppState

Central application state in the Yew app:

```rust
// components/app/crates/tt-rs-app/src/state.rs

pub struct AppState {
    pub widgets: Vec<WidgetItem>,
    pub boxes: Vec<BoxState>,
    pub dragging: Option<DragState>,
    pub active_tool: Option<Tool>,
    pub training_robot: Option<WidgetId>,
    pub positions: PositionStore,
}
```

### Position Management

Widgets track their positions separately:

```rust
// components/state/crates/tt-rs-state/src/position.rs

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct PositionStore {
    positions: HashMap<WidgetId, Position>,
}
```

## Robot Execution

### Training Mode

During training, user actions are recorded:

1. User clicks robot to start training
2. Robot enters `Training` state (yellow glow)
3. User performs actions (drag, drop, etc.)
4. Actions recorded with widget paths
5. User clicks robot to stop training
6. Robot stores action sequence

### Execution Engine

Robot execution replays recorded actions:

```rust
// components/app/crates/tt-rs-app/src/robot_exec/

pub fn execute_robot(state: &mut AppState, robot_id: WidgetId) {
    let robot = find_robot(state, robot_id);
    for action in &robot.actions {
        match action {
            Action::PickUp { path } => execute_pickup(state, path),
            Action::Drop { target } => execute_drop(state, target),
            // ... other actions
        }
    }
}
```

### Path Resolution

Widget paths identify targets for robot actions:

```rust
// widget:123       -> Widget with ID 123
// box:456:hole:0   -> First hole of box with ID 456
// box:456:hole:2   -> Third hole of box with ID 456
```

## Drag and Drop

### Draggable Component

Wrapper that makes widgets draggable:

```rust
// components/dnd/crates/tt-rs-drag/src/draggable.rs

#[function_component(Draggable)]
pub fn draggable(props: &DraggableProps) -> Html {
    // Mouse event handlers for drag operations
    // Position tracking via state
    // Visual feedback (scale, shadow)
}
```

### Copy Source

Palette items that create new widgets on drag:

```rust
// components/dnd/crates/tt-rs-drag/src/copy_source.rs

#[function_component(CopySource)]
pub fn copy_source(props: &CopySourceProps) -> Html {
    // Creates a new widget copy when dragged
    // Original stays in place
}
```

## Hit Testing

Utilities for finding widgets at screen coordinates:

```rust
// components/handlers/crates/tt-rs-hit-test/

pub fn find_widget_at(x: i32, y: i32, state: &AppState) -> Option<WidgetId>;
pub fn find_box_hole_at(x: i32, y: i32, state: &AppState) -> Option<(WidgetId, usize)>;
pub fn find_number_at(x: i32, y: i32, state: &AppState) -> Option<WidgetId>;
pub fn find_scales_at(x: i32, y: i32, state: &AppState) -> Option<WidgetId>;
```

## Build System

### Development Server

```bash
./scripts/serve.sh
# Runs trunk serve on port 1140
# Access at http://127.0.0.1:1140
```

### Quality Checks

```bash
./scripts/build-all.sh
# 1. cargo build --target wasm32-unknown-unknown
# 2. cargo test
# 3. cargo clippy --target wasm32-unknown-unknown
# 4. cargo fmt --check
```

### Production Build

```bash
./scripts/build-release.sh
# 1. trunk build --release --public-url /tt-rs/
# 2. Copy dist/ to docs/
# 3. Create .nojekyll
# 4. Validate paths contain /tt-rs/
```

## Technology Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2024 edition |
| UI Framework | Yew |
| WASM Bundler | Trunk |
| 2D Graphics | SVG, CSS |
| 3D Graphics | Three.js (planned) |
| State | Yew hooks (use_state, use_reducer) |
| Testing | cargo test |
| Linting | clippy, sw-checklist |

## Performance Considerations

### WASM Binary Size

- Use `wasm-opt` for optimization
- Careful dependency management
- Feature flags for optional components

### Rendering

- CSS animations where possible
- Batch DOM updates via Yew
- Virtual DOM diffing

### Memory

- Widget cleanup on removal
- Weak references where appropriate
- No global mutable state

## Security

### WebAssembly Sandboxing

- WASM provides memory isolation
- No direct system access
- Browser security model applies

### Input Validation

- Validate loaded programs
- Sanitize any user content
- Safe HTML rendering

## Testing Strategy

1. **Unit Tests**: Pure Rust tests for domain logic
2. **Component Tests**: Yew component testing (planned)
3. **Integration Tests**: End-to-end scenarios (planned)

## Future Architecture Changes

### Three.js Integration

When adding 3D graphics:

```rust
#[wasm_bindgen]
extern "C" {
    type ThreeScene;
    type ThreeMesh;
    // Bindings for Three.js
}
```

### Bird/Nest Messaging

New components needed:

```
components/widgets/crates/
├── tt-rs-bird/     # Bird widget
└── tt-rs-nest/     # Nest widget
```

### Persistence

New component for save/load:

```
components/persistence/crates/
└── tt-rs-storage/  # JSON serialization, localStorage
```

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
