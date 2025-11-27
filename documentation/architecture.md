# tt-rs Architecture

## Overview

tt-rs is a modern Rust/WebAssembly reimplementation of ToonTalk, an interactive visual programming environment designed to make programming concepts accessible to children (and adults). This document describes the target architecture for the Rust-based implementation.

**Full Name**: Cartoon-oriented Talking Programming Application

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution details.

## Historical Context

ToonTalk was originally created by Ken Kahn in 1992-1995, implemented in C++ and released commercially from 1996-2009. In 2014, a JavaScript/HTML5 version called "ToonTalk Reborn" was created. This project (tt-rs) is a modern reimagining using Rust, Yew, and WebAssembly.

## Design Philosophy

### Core Principles

1. **Rust for All Logic**: All presentation and domain logic implemented in Rust, compiled to WebAssembly. Minimal JavaScript (only where absolutely required for browser APIs).

2. **Modern Rendering**: Replace primitive Lego-like graphics with modern rendering using:
   - Three.js (via wasm-bindgen) for 3D graphics
   - SVG for scalable 2D graphics
   - CSS animations for smooth transitions
   - d3.js integration for data visualization

3. **Component-Based Architecture**: Using Yew framework for reactive UI components.

4. **Type Safety**: Leverage Rust's type system to make the programming model safer and more explicit.

5. **Speech Integration**: Potential for text-to-speech (TTS) integration, either pre-generated at development time or dynamically during execution, to enhance the "talking" aspect of the application.

## System Architecture

```
+------------------------------------------------------------------+
|                        Browser Environment                        |
+------------------------------------------------------------------+
|  +------------------------------------------------------------+  |
|  |                    Presentation Layer                       |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  |  |  Yew/WASM   |  |   Three.js  |  |   SVG/CSS       |      |  |
|  |  |  Components |  |   Renderer  |  |   Animations    |      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  +------------------------------------------------------------+  |
|                              |                                    |
|  +------------------------------------------------------------+  |
|  |                     Domain Layer (Rust)                     |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  |  |   Widgets   |  |   Robots    |  |   Bird/Nest     |      |  |
|  |  |   (Actors)  |  | (Programs)  |  |   (Messaging)   |      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  |  |   Numbers   |  |   Boxes     |  |   Scales        |      |  |
|  |  | (Arithmetic)|  | (Containers)|  |   (Comparison)  |      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  +------------------------------------------------------------+  |
|                              |                                    |
|  +------------------------------------------------------------+  |
|  |                   Infrastructure Layer                      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  |  |   Storage   |  |  Event Bus  |  |   Serialization |      |  |
|  |  |   (Local/   |  |  (Channels) |  |   (JSON/Binary) |      |  |
|  |  |   Cloud)    |  |             |  |                 |      |  |
|  |  +-------------+  +-------------+  +-----------------+      |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
```

## Module Structure

```
tt-rs/
+-- Cargo.toml
+-- src/
|   +-- lib.rs              # Library root, WASM entry point
|   +-- main.rs             # Native entry point (for testing)
|   |
|   +-- domain/             # Core business logic
|   |   +-- mod.rs
|   |   +-- widget.rs       # Base widget trait and implementations
|   |   +-- number.rs       # Numeric widgets (rational arithmetic)
|   |   +-- box_.rs         # Container widgets
|   |   +-- robot.rs        # Programmable robots
|   |   +-- bird.rs         # Message-carrying birds
|   |   +-- nest.rs         # Message-receiving nests
|   |   +-- scale.rs        # Comparison widgets
|   |   +-- sensor.rs       # Event sensors
|   |   +-- element.rs      # HTML/SVG elements
|   |   +-- tool.rs         # Wand, vacuum tools
|   |
|   +-- execution/          # Robot execution engine
|   |   +-- mod.rs
|   |   +-- action.rs       # Robot actions
|   |   +-- path.rs         # Widget path resolution
|   |   +-- matching.rs     # Pattern matching for conditions
|   |   +-- scheduler.rs    # Concurrent execution scheduler
|   |
|   +-- presentation/       # Yew components
|   |   +-- mod.rs
|   |   +-- workspace.rs    # Main workspace component
|   |   +-- widget_view.rs  # Widget rendering
|   |   +-- backside.rs     # Widget backside views
|   |   +-- toolbar.rs      # Tool palette
|   |   +-- animations.rs   # Animation controllers
|   |
|   +-- rendering/          # Graphics abstraction
|   |   +-- mod.rs
|   |   +-- three_js.rs     # Three.js bindings
|   |   +-- svg.rs          # SVG rendering
|   |   +-- css_anim.rs     # CSS animation helpers
|   |
|   +-- storage/            # Persistence
|   |   +-- mod.rs
|   |   +-- local.rs        # LocalStorage/IndexedDB
|   |   +-- cloud.rs        # Cloud storage adapters
|   |   +-- serialization.rs # JSON serialization
|   |
|   +-- audio/              # Speech and sound
|   |   +-- mod.rs
|   |   +-- tts.rs          # Text-to-speech integration
|   |   +-- sounds.rs       # Sound effect playback
|   |
|   +-- bindings/           # JavaScript interop
|       +-- mod.rs
|       +-- three_js.rs     # Three.js FFI
|       +-- d3.rs           # d3.js FFI
|       +-- browser.rs      # Browser API bindings
|
+-- documentation/          # Project documentation
|   +-- architecture.md     # This file
|   +-- prd.md              # Product requirements
|   +-- design.md           # Design decisions
|   +-- plan.md             # Implementation plan
|   +-- process.md          # Development process
|
+-- assets/                 # Static assets
|   +-- images/
|   +-- sounds/
|   +-- styles/
|
+-- tests/                  # Integration tests
    +-- widget_tests.rs
    +-- robot_tests.rs
    +-- e2e/
```

## Core Abstractions

### Widget Trait

The fundamental abstraction is the `Widget` trait, which all visual programming objects implement:

```rust
pub trait Widget: Send + Sync {
    fn get_type_name(&self) -> &'static str;
    fn copy(&self) -> Box<dyn Widget>;
    fn matches(&self, other: &dyn Widget) -> MatchResult;
    fn serialize(&self) -> serde_json::Value;
    fn render(&self, ctx: &RenderContext) -> Html;

    // Identity and hierarchy
    fn id(&self) -> WidgetId;
    fn parent(&self) -> Option<WidgetRef>;
    fn set_parent(&mut self, parent: Option<WidgetRef>);

    // Backside (configuration/programming interface)
    fn backside(&self) -> &Backside;
    fn backside_mut(&mut self) -> &mut Backside;
}
```

### Robot (Program) Model

Robots are the core programming metaphor - they are trained by demonstration:

```rust
pub struct Robot {
    id: RobotId,
    frontside_conditions: Option<Box<dyn Widget>>,
    backside_conditions: Vec<Box<dyn Widget>>,
    body: ActionSequence,
    run_once: bool,
    next_robot: Option<Box<Robot>>,
}

pub struct ActionSequence {
    steps: Vec<Action>,
}

pub enum Action {
    PickUp { path: WidgetPath },
    Drop { target_path: WidgetPath },
    Copy { source_path: WidgetPath },
    Remove { path: WidgetPath },
    Edit { path: WidgetPath, operation: EditOperation },
    // ... more actions
}
```

### Message Passing (Bird/Nest)

Birds and nests implement the Actor model for concurrent communication:

```rust
pub struct Bird {
    id: BirdId,
    nest: NestRef,
    color: BirdColor,
}

pub struct Nest {
    id: NestId,
    contents: VecDeque<Box<dyn Widget>>,
    waiting_robots: Vec<RobotRef>,
    bird: Option<BirdRef>,
}

impl Bird {
    pub async fn deliver(&self, message: Box<dyn Widget>) {
        // Animated delivery to nest
    }
}

impl Nest {
    pub fn receive(&mut self, message: Box<dyn Widget>) {
        self.contents.push_back(message);
        self.notify_waiting_robots();
    }
}
```

## Rendering Architecture

### Three.js Integration

For 3D graphics, we use wasm-bindgen to call Three.js:

```rust
#[wasm_bindgen]
extern "C" {
    type Scene;
    type Mesh;
    type Camera;

    #[wasm_bindgen(constructor)]
    fn new() -> Scene;

    #[wasm_bindgen(method)]
    fn add(this: &Scene, object: &Mesh);
}
```

### SVG Rendering

For 2D widgets, SVG provides scalable graphics:

```rust
pub fn render_number(value: &Rational, ctx: &RenderContext) -> Html {
    html! {
        <svg class="widget number" viewBox="0 0 100 60">
            <rect class="number-background" />
            <text class="number-value">{value.to_string()}</text>
        </svg>
    }
}
```

### CSS Animations

Smooth transitions using CSS:

```rust
pub fn animate_bird_flight(bird: &Bird, from: Point, to: Point) -> Animation {
    Animation::new()
        .keyframes(vec![
            Keyframe::at(0.0).transform(translate(from)),
            Keyframe::at(1.0).transform(translate(to)),
        ])
        .duration(Duration::from_millis(500))
        .easing(Easing::EaseInOut)
}
```

## Concurrency Model

The application is inherently concurrent - multiple robots can run simultaneously. We use Rust's async/await with a custom scheduler:

```rust
pub struct Scheduler {
    run_queue: VecDeque<RobotExecution>,
    channel_rx: mpsc::Receiver<SchedulerMessage>,
}

impl Scheduler {
    pub async fn run(&mut self) {
        loop {
            // Process pending messages
            while let Ok(msg) = self.channel_rx.try_recv() {
                self.handle_message(msg);
            }

            // Execute one step from each ready robot
            for execution in self.run_queue.iter_mut() {
                if execution.is_ready() {
                    execution.step().await;
                }
            }

            // Yield to browser event loop
            gloo::timers::future::TimeoutFuture::new(0).await;
        }
    }
}
```

## State Management

Using Yew's context and agents for global state:

```rust
pub struct AppState {
    workspace: Workspace,
    selected_widget: Option<WidgetId>,
    tool: Option<Tool>,
    robots_running: HashSet<RobotId>,
}

pub enum AppAction {
    SelectWidget(WidgetId),
    DeselectWidget,
    PickUpTool(Tool),
    DropTool,
    StartRobot(RobotId),
    StopRobot(RobotId),
    // ...
}
```

## Persistence

Programs are serialized to JSON for storage:

```rust
#[derive(Serialize, Deserialize)]
pub struct WorkspaceData {
    pub version: u32,
    pub widgets: Vec<WidgetData>,
    pub settings: Settings,
}

impl Workspace {
    pub fn save(&self) -> Result<String, Error> {
        let data = WorkspaceData::from(self);
        serde_json::to_string(&data)
    }

    pub fn load(json: &str) -> Result<Self, Error> {
        let data: WorkspaceData = serde_json::from_str(json)?;
        Self::from(data)
    }
}
```

## Browser Integration

Minimal JavaScript interop for browser APIs:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;
}
```

## Testing Strategy

1. **Unit Tests**: Pure Rust tests for domain logic
2. **Component Tests**: Yew component testing
3. **Integration Tests**: WASM-based browser tests
4. **Visual Regression**: Screenshot comparison tests

## Performance Considerations

1. **WASM Binary Size**: Use `wasm-opt` and careful dependency management
2. **Rendering**: Batch DOM updates, use virtual scrolling for large workspaces
3. **Memory**: Implement proper cleanup for dropped widgets
4. **Animation**: Use requestAnimationFrame, CSS transitions where possible

## Security

1. **Sandboxing**: WebAssembly provides memory isolation
2. **Input Validation**: Validate all user input and loaded programs
3. **Content Security**: Sanitize any HTML content from elements

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
