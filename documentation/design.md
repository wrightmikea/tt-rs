# tt-rs Design Document

**Full Name**: Cartoon-oriented Talking Programming Application

## Overview

This document describes the key design decisions for tt-rs, a modern Rust/WebAssembly reimplementation of ToonTalk.

**Note**: This is a derived work based on ToonTalk. See the COPYRIGHT file for attribution.

## Design Principles

### 1. Faithful to the Original Vision

The genius of the original system lies in its metaphor-based approach to programming concepts. Every design decision should preserve or enhance these metaphors:

- Robots train by watching demonstrations
- Birds carry messages to nests
- Boxes hold things in compartments
- Scales compare by tipping
- Erasure creates generalized patterns

### 2. Modern Technology, Timeless Concepts

While the concepts remain unchanged, the implementation uses modern tools:

| Original | ToonTalk Reborn | tt-rs |
|----------|-----------------|-------|
| C++ | JavaScript/jQuery | Rust/WASM |
| Windows GDI | CSS/Canvas | Three.js/SVG/CSS |
| File system | localStorage/JSON | IndexedDB/JSON |

### 3. Type Safety Through Rust

Rust's type system helps prevent entire classes of bugs:

- No null pointer exceptions (use `Option<T>`)
- No data races (ownership system)
- Clear interfaces through traits
- Compile-time guarantees

### 4. Minimal JavaScript

JavaScript is used only where absolutely necessary:

- Browser API bindings (via wasm-bindgen)
- Three.js interop
- DOM manipulation where Yew is insufficient

All business logic, state management, and game logic is in Rust.

### 5. Speech-Ready Architecture

The "talking" aspect of the application name suggests potential for:

- Pre-generated TTS audio assets for common phrases
- Dynamic TTS integration during runtime
- Audio feedback for user interactions

## Core Data Model

### Widget Hierarchy

```
Widget (trait)
+-- Number
|   +-- { numerator: BigInt, denominator: BigInt, operator: ArithOp }
+-- Box
|   +-- { holes: Vec<Option<WidgetRef>>, orientation: Orientation }
+-- Robot
|   +-- { conditions: Conditions, body: Actions, state: RobotState }
+-- Bird
|   +-- { nest: NestRef, color: Color }
+-- Nest
|   +-- { contents: VecDeque<WidgetRef>, waiting: Vec<RobotRef> }
+-- Scale
|   +-- { left: Option<WidgetRef>, right: Option<WidgetRef> }
+-- Sensor
|   +-- { event_type: EventType, attribute: String }
+-- Element
|   +-- { content: DomContent, attributes: HashMap<String, String> }
+-- Tool
    +-- Wand
    +-- Vacuum
```

### Widget Identity

Each widget has a unique identity:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
```

### Widget References

Widgets are managed through reference-counted pointers:

```rust
pub type WidgetRef = Rc<RefCell<dyn Widget>>;

// For cross-thread scenarios (future)
pub type SharedWidgetRef = Arc<RwLock<dyn Widget + Send + Sync>>;
```

### Frontside and Backside

Every widget has two faces:

```rust
pub struct WidgetView {
    frontside: FrontsideView,  // Visual representation
    backside: BacksideView,    // Configuration interface
    is_flipped: bool,
}

pub struct FrontsideView {
    position: Point,
    dimensions: Size,
    visible: bool,
    // Widget-specific rendering data
}

pub struct BacksideView {
    widgets: Vec<WidgetRef>,  // Widgets on the backside
    configuration: WidgetConfig,
}
```

## Robot Programming Model

### Robot Structure

```rust
pub struct Robot {
    id: RobotId,
    name: Option<String>,

    // Conditions for when robot can run
    frontside_condition: Option<WidgetRef>,  // Pattern to match
    backside_conditions: Vec<WidgetRef>,     // Required backside widgets

    // What the robot does
    body: ActionSequence,

    // Behavior
    run_once: bool,           // Stop after one successful run
    watched_speed: f32,       // Animation speed when watched

    // Chaining
    next_robot: Option<Box<Robot>>,  // Run this robot next

    // Runtime state
    state: RobotState,
}

pub enum RobotState {
    Idle,
    Training,
    Waiting,        // Waiting for conditions to match
    Running(usize), // Currently at step N
    Finished,
}
```

### Pattern Matching

Robots use pattern matching to decide when to run:

```rust
pub enum MatchResult {
    Match(Bindings),           // Pattern matches, with bindings
    NoMatch,                   // Pattern doesn't match
    Partial(Vec<Requirement>), // Could match if requirements met
}

pub struct Bindings {
    widget_bindings: HashMap<WidgetPath, WidgetRef>,
    value_bindings: HashMap<String, Value>,
}
```

### Erasure for Generalization

The vacuum tool creates "erased" patterns that match more broadly:

```rust
pub struct Number {
    value: Rational,
    erased: ErasureLevel,
}

pub enum ErasureLevel {
    Specific,      // Matches only this exact value
    Sign,          // Matches any number with same sign
    Type,          // Matches any number
}

impl Number {
    fn matches(&self, other: &Number) -> MatchResult {
        match self.erased {
            ErasureLevel::Specific => {
                if self.value == other.value {
                    MatchResult::Match(Bindings::empty())
                } else {
                    MatchResult::NoMatch
                }
            }
            ErasureLevel::Sign => {
                if self.value.signum() == other.value.signum() {
                    MatchResult::Match(Bindings::with("value", other.value))
                } else {
                    MatchResult::NoMatch
                }
            }
            ErasureLevel::Type => {
                MatchResult::Match(Bindings::with("value", other.value))
            }
        }
    }
}
```

### Action Recording and Playback

During training, actions are recorded as a sequence:

```rust
pub struct ActionSequence {
    steps: Vec<RecordedAction>,
}

pub struct RecordedAction {
    action: Action,
    path: WidgetPath,
    timing: Duration,  // Time since last action
}

pub enum Action {
    PickUp,
    Drop { target: WidgetPath },
    Copy,
    Remove,
    Edit(EditAction),
    GiveToBird { bird: WidgetPath },
    // ... more actions
}
```

### Widget Path Resolution

Paths describe how to find a widget relative to the context:

```rust
pub enum WidgetPath {
    // Direct references
    Context,                    // The widget being processed
    Backside(Box<WidgetPath>),  // The backside of a widget

    // Box navigation
    Hole(usize, Box<WidgetPath>),  // Hole N of a box

    // Pattern-matched references
    MatchedWidget(String),      // Widget matched by name

    // Type-based lookup
    WidgetOfType(String),       // First widget of type on backside

    // Nest contents
    NestTop(Box<WidgetPath>),   // Top item in a nest
}
```

## Message Passing (Bird/Nest)

### Actor-Like Communication

Birds and nests implement a simple actor model:

```rust
impl Bird {
    pub fn give(&self, message: WidgetRef) -> DeliveryFuture {
        if let Some(nest) = &self.nest {
            self.fly_to_nest(message, nest)
        } else {
            // Bird without nest - message is lost
            DeliveryFuture::immediate(DeliveryResult::Lost)
        }
    }

    async fn fly_to_nest(&self, message: WidgetRef, nest: &NestRef) {
        // Animate flight
        self.animate_flight(nest.position()).await;

        // Deliver message
        nest.borrow_mut().receive(message);

        // Return home
        self.animate_return().await;
    }
}

impl Nest {
    pub fn receive(&mut self, message: WidgetRef) {
        self.contents.push_back(message);

        // Wake up any waiting robots
        for robot in self.waiting.drain(..) {
            robot.borrow_mut().notify_nest_ready();
        }
    }

    pub fn take(&mut self) -> Option<WidgetRef> {
        self.contents.pop_front()
    }
}
```

### Concurrent Execution

Multiple birds can be in flight simultaneously:

```rust
pub struct DeliveryManager {
    active_deliveries: Vec<DeliveryFuture>,
}

impl DeliveryManager {
    pub async fn tick(&mut self) {
        // Progress all active deliveries
        let mut completed = vec![];
        for (i, delivery) in self.active_deliveries.iter_mut().enumerate() {
            if delivery.poll().is_ready() {
                completed.push(i);
            }
        }
        // Remove completed deliveries
        for i in completed.into_iter().rev() {
            self.active_deliveries.remove(i);
        }
    }
}
```

## Visual Design

### Modern Widget Appearance

#### Numbers

Replace the old Lego-like numbers with clean, modern typography:

```rust
pub fn render_number(num: &Number, ctx: &RenderContext) -> Html {
    let display = num.to_display_string();
    let class = format!("widget number {}", num.operator_class());

    html! {
        <div class={class}>
            <svg viewBox="0 0 120 80">
                // Rounded rectangle background with gradient
                <defs>
                    <linearGradient id="num-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" style="stop-color:#f8f8f8" />
                        <stop offset="100%" style="stop-color:#e8e8e8" />
                    </linearGradient>
                </defs>
                <rect rx="8" ry="8" fill="url(#num-gradient)"
                      stroke="#ccc" stroke-width="2" />

                // Operator indicator
                <text class="operator" x="10" y="20">{num.operator_symbol()}</text>

                // Value with nice font
                <text class="value" x="60" y="50" text-anchor="middle">
                    {display}
                </text>
            </svg>
        </div>
    }
}
```

#### Boxes

3D-looking containers with clear compartments:

```rust
pub fn render_box(box_widget: &BoxWidget, ctx: &RenderContext) -> Html {
    html! {
        <div class="widget box-widget">
            <svg viewBox="0 0 200 60">
                // 3D effect with shadow
                <filter id="box-shadow">
                    <feDropShadow dx="2" dy="2" stdDeviation="2" flood-opacity="0.3"/>
                </filter>

                // Box frame
                <rect class="box-frame" filter="url(#box-shadow)" />

                // Compartment dividers
                {for (0..box_widget.size()).map(|i| {
                    html! { <line class="divider" x1={hole_x(i)} y1="0" x2={hole_x(i)} y2="60" /> }
                })}

                // Hole contents
                {for box_widget.holes().iter().enumerate().map(|(i, hole)| {
                    render_hole(i, hole, ctx)
                })}
            </svg>
        </div>
    }
}
```

#### Birds

Animated birds with personality:

```rust
pub fn render_bird(bird: &Bird, ctx: &RenderContext) -> Html {
    let animation_class = match bird.state() {
        BirdState::Idle => "bird-idle",
        BirdState::Carrying(_) => "bird-carrying",
        BirdState::Flying => "bird-flying",
    };

    html! {
        <div class={format!("widget bird {}", animation_class)}>
            // Using CSS animation sprites or Three.js for 3D bird
            <div class="bird-sprite" style={format!("--bird-color: {}", bird.color.to_css())} />
        </div>
    }
}
```

### Animation System

#### CSS-Based Animations

For simple transitions:

```css
.widget {
    transition: transform 0.2s ease-out, opacity 0.2s;
}

.widget.dragging {
    transform: scale(1.05);
    opacity: 0.9;
    filter: drop-shadow(0 4px 8px rgba(0,0,0,0.3));
}

.bird-flying {
    animation: bird-flap 0.2s infinite;
}

@keyframes bird-flap {
    0%, 100% { transform: translateY(0) rotate(0deg); }
    50% { transform: translateY(-5px) rotate(-5deg); }
}
```

#### Programmatic Animations

For complex, data-driven animations:

```rust
pub struct AnimationController {
    active_animations: Vec<Box<dyn Animation>>,
}

pub trait Animation {
    fn update(&mut self, delta: Duration) -> AnimationState;
    fn apply(&self, target: &mut dyn Widget);
}

pub struct BirdFlightAnimation {
    bird: WidgetRef,
    from: Point,
    to: Point,
    progress: f32,
    duration: Duration,
}

impl Animation for BirdFlightAnimation {
    fn update(&mut self, delta: Duration) -> AnimationState {
        self.progress += delta.as_secs_f32() / self.duration.as_secs_f32();
        if self.progress >= 1.0 {
            AnimationState::Complete
        } else {
            AnimationState::Running
        }
    }

    fn apply(&self, target: &mut dyn Widget) {
        let t = ease_in_out(self.progress);
        let pos = self.from.lerp(self.to, t);

        // Add arc for natural flight path
        let arc_height = (self.from.distance(self.to) * 0.2) * (t * (1.0 - t) * 4.0);
        let pos = Point::new(pos.x, pos.y - arc_height);

        target.set_position(pos);
    }
}
```

### Three.js Integration

For 3D effects:

```rust
#[wasm_bindgen]
extern "C" {
    pub type ThreeScene;
    pub type ThreeMesh;

    #[wasm_bindgen(constructor, js_namespace = THREE)]
    pub fn new() -> ThreeScene;

    #[wasm_bindgen(method)]
    pub fn add(this: &ThreeScene, mesh: &ThreeMesh);
}

pub struct ThreeRenderer {
    scene: ThreeScene,
    camera: ThreeCamera,
    renderer: ThreeRenderer,
}

impl ThreeRenderer {
    pub fn render_3d_widget(&self, widget: &dyn Widget) -> ThreeMesh {
        match widget.get_type_name() {
            "number" => self.create_number_mesh(widget),
            "box" => self.create_box_mesh(widget),
            _ => self.create_default_mesh(widget),
        }
    }
}
```

## State Management

### Application State

```rust
pub struct AppState {
    workspace: Workspace,
    selection: Selection,
    tool: Option<Tool>,
    dragging: Option<DragState>,
    robot_executions: Vec<RobotExecution>,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

pub enum Selection {
    None,
    Single(WidgetId),
    Multiple(HashSet<WidgetId>),
}

pub struct DragState {
    widget: WidgetRef,
    offset: Point,
    original_position: Point,
}
```

### Command Pattern for Undo

```rust
pub trait Command {
    fn execute(&self, state: &mut AppState) -> Result<(), Error>;
    fn undo(&self, state: &mut AppState) -> Result<(), Error>;
    fn description(&self) -> &str;
}

pub struct MoveWidgetCommand {
    widget_id: WidgetId,
    from: Point,
    to: Point,
}

impl Command for MoveWidgetCommand {
    fn execute(&self, state: &mut AppState) -> Result<(), Error> {
        state.workspace.move_widget(self.widget_id, self.to)
    }

    fn undo(&self, state: &mut AppState) -> Result<(), Error> {
        state.workspace.move_widget(self.widget_id, self.from)
    }

    fn description(&self) -> &str {
        "Move widget"
    }
}
```

## Serialization

### JSON Format

Compatible with ToonTalk Reborn where possible:

```rust
#[derive(Serialize, Deserialize)]
pub struct WidgetJson {
    pub semantic: SemanticData,
    pub view: ViewData,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SemanticData {
    #[serde(rename = "number")]
    Number {
        numerator: String,
        denominator: String,
        operator: String,
    },
    #[serde(rename = "box")]
    Box {
        size: usize,
        contents: Vec<Option<Box<WidgetJson>>>,
        horizontal: bool,
    },
    #[serde(rename = "robot")]
    Robot {
        body: BodyJson,
        conditions: Option<Box<WidgetJson>>,
        run_once: bool,
    },
    // ... more types
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TtrsError {
    #[error("Widget not found: {0}")]
    WidgetNotFound(WidgetId),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Robot error: {0}")]
    RobotError(RobotError),
}

#[derive(Debug, thiserror::Error)]
pub enum RobotError {
    #[error("No match for conditions")]
    NoMatch,

    #[error("Path not found: {0:?}")]
    PathNotFound(WidgetPath),

    #[error("Action failed: {0}")]
    ActionFailed(String),
}
```

### User-Friendly Messages

```rust
impl TtrsError {
    pub fn user_message(&self) -> String {
        match self {
            Self::WidgetNotFound(_) => "Oops! I can't find that thing.".into(),
            Self::InvalidOperation(msg) => format!("Sorry, I can't do that: {}", msg),
            Self::RobotError(RobotError::NoMatch) =>
                "The robot doesn't know what to do with this.".into(),
            // ... more friendly messages
        }
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_addition() {
        let a = Number::new(1, 1);
        let b = Number::new(2, 1);
        let result = a.apply_to(&b).unwrap();
        assert_eq!(result.value(), Rational::new(3, 1));
    }

    #[test]
    fn pattern_matching() {
        let pattern = Number::erased(ErasureLevel::Type);
        let value = Number::new(42, 1);
        assert!(matches!(pattern.matches(&value), MatchResult::Match(_)));
    }
}
```

### Integration Tests

```rust
#[wasm_bindgen_test]
async fn robot_training_and_execution() {
    let workspace = Workspace::new();

    // Create widgets
    let num1 = workspace.add_number(1);
    let num2 = workspace.add_number(2);
    let robot = workspace.add_robot();

    // Train robot
    robot.start_training(&workspace);
    workspace.pick_up(num1);
    workspace.drop_on(num2);
    robot.stop_training();

    // Run robot with new data
    let num3 = workspace.add_number(10);
    let num4 = workspace.add_number(20);
    robot.run_on(&workspace, num3).await;

    // Verify result
    assert_eq!(num4.value(), Rational::new(30, 1));
}
```

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
