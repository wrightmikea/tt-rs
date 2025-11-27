# tt-rs Product Requirements Document

**Full Name**: Cartoon-oriented Talking Programming Application

## Executive Summary

tt-rs is a modern reimplementation of ToonTalk, an interactive visual programming environment originally created by Ken Kahn in the 1990s. This project aims to bring the innovative programming-by-demonstration approach to modern web browsers using Rust, WebAssembly, and contemporary rendering technologies.

**Note**: This is a derived work. See the COPYRIGHT file for full attribution.

## Background

### Original ToonTalk

ToonTalk was designed to make programming concepts accessible to children by mapping every abstract computational aspect to a concrete, animated metaphor:

- **Computation** = A city with houses
- **Active objects/agents** = Houses
- **Methods/clauses** = Robots trained by demonstration
- **Messages** = Items carried by birds to nests
- **Data structures** = Boxes with compartments
- **Comparison** = Scales that tip based on weight/value

The system was influenced by:
- The Janus programming language (concurrent constraint logic programming)
- The Actor model
- Video games like "The Legend of Zelda" and "Robot Odyssey"

ToonTalk was successfully used by children as young as 3 years old.

### ToonTalk Reborn (JavaScript)

In 2014, a JavaScript/HTML5 version was created, making ToonTalk accessible in web browsers. This version:
- Uses jQuery and jQuery UI
- Stores programs as JSON
- Supports Google Drive integration
- Includes primitive Lego-like graphics

### Why tt-rs?

The JavaScript implementation has accumulated technical debt and relies on aging technologies (jQuery, jQuery UI). This project aims to:

1. Modernize the technology stack
2. Improve visual appeal with modern graphics
3. Leverage Rust's type safety and performance
4. Create a maintainable, well-documented codebase
5. Add speech capabilities ("talking" in the name)

## Product Vision

**For** children, educators, and curious adults
**Who** want to learn programming concepts through visual, interactive exploration
**tt-rs is** a web-based visual programming environment
**That** teaches computational thinking through animated metaphors and programming by demonstration
**Unlike** traditional text-based programming or simplified block-based coding
**Our product** provides a deeply interactive, game-like experience where abstract concepts become tangible objects that can be manipulated, combined, and automated.

## Target Users

### Primary Users

1. **Children (ages 5-12)**
   - Little to no programming experience
   - Comfortable with touch/mouse interfaces
   - May not read fluently

2. **Educators**
   - Teachers introducing computational thinking
   - After-school program leaders
   - Parents doing homeschool education

3. **Curious Adults**
   - People interested in alternative programming paradigms
   - Those who find traditional programming intimidating
   - Concurrent programming enthusiasts

### Secondary Users

1. **Researchers**
   - Computer science education researchers
   - Programming language researchers
   - HCI researchers

2. **Developers**
   - Those interested in Rust/WASM game development
   - Contributors to educational software

## Requirements

### Functional Requirements

#### FR1: Core Widgets

| ID | Requirement | Priority |
|----|-------------|----------|
| FR1.1 | Numbers with rational arithmetic (arbitrary precision) | Must Have |
| FR1.2 | Boxes (containers with numbered compartments) | Must Have |
| FR1.3 | Birds and Nests (message passing) | Must Have |
| FR1.4 | Robots (programmable agents trained by demonstration) | Must Have |
| FR1.5 | Scales (comparison operations) | Must Have |
| FR1.6 | Sensors (keyboard, mouse, touch events) | Must Have |
| FR1.7 | Elements (HTML/SVG content) | Should Have |
| FR1.8 | Function birds (mathematical functions) | Should Have |

#### FR2: Tools

| ID | Requirement | Priority |
|----|-------------|----------|
| FR2.1 | Magic Wand for copying objects | Must Have |
| FR2.2 | Dusty the Vacuum for removing/erasing objects | Must Have |
| FR2.3 | Selection tool for multi-select operations | Should Have |

#### FR3: Robot Programming

| ID | Requirement | Priority |
|----|-------------|----------|
| FR3.1 | Train robots by demonstrating actions | Must Have |
| FR3.2 | Set frontside conditions (pattern matching) | Must Have |
| FR3.3 | Set backside conditions (additional constraints) | Must Have |
| FR3.4 | Chain robots (sequential execution) | Must Have |
| FR3.5 | Run robots watched (step-by-step visualization) | Must Have |
| FR3.6 | Run robots unwatched (full speed) | Must Have |
| FR3.7 | Generalize patterns using erasure | Must Have |
| FR3.8 | Robot teams (alternative behaviors) | Should Have |

#### FR4: Workspace

| ID | Requirement | Priority |
|----|-------------|----------|
| FR4.1 | Drag and drop widgets | Must Have |
| FR4.2 | Resize widgets | Must Have |
| FR4.3 | Widget backside access (flip to configure) | Must Have |
| FR4.4 | Multiple workspaces | Should Have |
| FR4.5 | Zoom and pan | Should Have |
| FR4.6 | Undo/Redo | Should Have |

#### FR5: Persistence

| ID | Requirement | Priority |
|----|-------------|----------|
| FR5.1 | Save to browser local storage | Must Have |
| FR5.2 | Load from local storage | Must Have |
| FR5.3 | Export to file | Must Have |
| FR5.4 | Import from file | Must Have |
| FR5.5 | Cloud storage integration | Could Have |
| FR5.6 | Share via URL | Could Have |

#### FR6: Audio/Speech

| ID | Requirement | Priority |
|----|-------------|----------|
| FR6.1 | Sound effects for interactions | Should Have |
| FR6.2 | Text-to-speech for instructions/feedback | Could Have |
| FR6.3 | Pre-generated speech assets | Could Have |
| FR6.4 | Dynamic TTS during execution | Could Have |

#### FR7: Compatibility

| ID | Requirement | Priority |
|----|-------------|----------|
| FR7.1 | Import ToonTalk Reborn JSON programs | Should Have |
| FR7.2 | Export to ToonTalk Reborn format | Could Have |

### Non-Functional Requirements

#### NFR1: Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR1.1 | Initial load time | < 3 seconds on broadband |
| NFR1.2 | WASM binary size | < 2MB compressed |
| NFR1.3 | Frame rate during animations | 60 FPS |
| NFR1.4 | Robot execution speed (unwatched) | 10,000+ steps/second |

#### NFR2: Compatibility

| ID | Requirement | Target |
|----|-------------|--------|
| NFR2.1 | Browser support | Chrome 90+, Firefox 90+, Safari 15+, Edge 90+ |
| NFR2.2 | Device support | Desktop, tablet (touch support) |
| NFR2.3 | Screen sizes | 1024x768 minimum |

#### NFR3: Accessibility

| ID | Requirement | Priority |
|----|-------------|----------|
| NFR3.1 | Keyboard navigation | Should Have |
| NFR3.2 | Screen reader support | Could Have |
| NFR3.3 | High contrast mode | Could Have |
| NFR3.4 | Configurable animation speed | Should Have |

#### NFR4: Usability

| ID | Requirement | Target |
|----|-------------|--------|
| NFR4.1 | Time to first interaction | < 10 seconds |
| NFR4.2 | Discoverable features | 80% without documentation |
| NFR4.3 | Error recovery | Clear, non-technical messages |

#### NFR5: Quality

| ID | Requirement | Target |
|----|-------------|--------|
| NFR5.1 | Test coverage | > 80% for domain logic |
| NFR5.2 | Crash rate | < 0.1% of sessions |
| NFR5.3 | Memory leaks | None detectable |

### Visual Design Requirements

#### VR1: Modern Aesthetics

| ID | Requirement | Priority |
|----|-------------|----------|
| VR1.1 | Replace Lego-like graphics with modern 3D/SVG | Must Have |
| VR1.2 | Consistent visual language across widgets | Must Have |
| VR1.3 | Smooth animations for all state changes | Must Have |
| VR1.4 | Appealing color palette | Must Have |

#### VR2: Widget Visuals

| ID | Requirement | Priority |
|----|-------------|----------|
| VR2.1 | Numbers: Clean, readable typography | Must Have |
| VR2.2 | Boxes: 3D appearance with clear compartments | Must Have |
| VR2.3 | Birds: Animated flight paths | Must Have |
| VR2.4 | Robots: Expressive animations (working, waiting, done) | Must Have |
| VR2.5 | Nests: Egg hatching animation when receiving messages | Must Have |
| VR2.6 | Scales: Physics-based tipping animation | Should Have |

#### VR3: Rendering Technologies

| ID | Requirement | Priority |
|----|-------------|----------|
| VR3.1 | Three.js for 3D graphics | Should Have |
| VR3.2 | SVG for scalable 2D graphics | Must Have |
| VR3.3 | CSS animations for UI transitions | Must Have |
| VR3.4 | d3.js for data visualization elements | Could Have |

## User Stories

### Essential User Stories

1. **As a child**, I want to drag a number onto another number to add them, so I can do arithmetic visually.

2. **As a child**, I want to train a robot by showing it what to do, so I can automate repetitive tasks.

3. **As a child**, I want to give something to a bird and watch it fly to its nest, so I can understand message passing.

4. **As a child**, I want to put things in boxes and take them out, so I can organize my work.

5. **As an educator**, I want to save and share programs, so students can continue their work.

6. **As a user**, I want to watch a robot work step-by-step, so I can understand what it's doing.

7. **As a user**, I want to erase parts of a pattern, so my robot can work with many different inputs.

### Advanced User Stories

8. **As an advanced user**, I want to create recursive programs using birds and nests, so I can compute things like factorials.

9. **As an advanced user**, I want multiple robots to run concurrently, so I can build complex parallel systems.

10. **As an educator**, I want to import programs from ToonTalk Reborn, so I can use existing curricula.

## Constraints

### Technical Constraints

1. Must run entirely in the browser (no server-side execution)
2. Must compile to WebAssembly from Rust
3. Maximum JavaScript usage limited to browser API bindings
4. Must work without plugins or extensions

### Legal Constraints

1. Must maintain BSD license compatibility
2. Must preserve original copyright notices
3. Must properly attribute derived work

### Resource Constraints

1. Single primary developer
2. No dedicated design resources initially
3. Open source, community-driven development

## Success Metrics

### Launch Metrics (MVP)

- Core widgets functional (number, box, robot, bird/nest)
- Save/load working
- 3 example programs demonstrating capabilities
- Basic documentation complete

### Growth Metrics (Post-Launch)

- Monthly active users
- Programs created and saved
- Community contributions (issues, PRs)
- Educational deployments

## Milestones

### Milestone 1: Foundation (MVP)
- Basic widget system
- Drag and drop
- Number arithmetic
- Box containers
- Persistence

### Milestone 2: Programming
- Robot training
- Pattern matching
- Robot execution
- Bird/nest messaging

### Milestone 3: Polish
- Modern graphics (Three.js/SVG)
- Animations
- Sound effects
- UI polish

### Milestone 4: Ecosystem
- Import/export
- Sharing
- Documentation
- Examples

## Appendix

### Glossary

- **Widget**: Any visual object in the application (number, box, robot, etc.)
- **Backside**: The configuration/programming interface of a widget
- **Robot**: An automated agent that performs recorded actions
- **Training**: The process of teaching a robot by demonstration
- **Bird**: A messenger that carries items to its associated nest
- **Nest**: A receiver of messages delivered by its bird
- **Box**: A container with numbered compartments (holes)
- **Scale**: A comparison widget that tips based on placed values
- **Sensor**: A widget that responds to browser events
- **Wand**: A tool for copying widgets
- **Vacuum (Dusty)**: A tool for removing/erasing widgets

### References

- [ToonTalk Wikipedia](https://en.wikipedia.org/wiki/ToonTalk)
- [ToonTalk Papers](http://toontalk.com/English/papers.htm)
- [ToonTalk Reborn Wiki](https://github.com/ToonTalk/ToonTalk/wiki)
- [Ken Kahn's Homepage](https://toontalk.com/English/kenkahn.htm)

## License

BSD 3-Clause License

See COPYRIGHT and LICENSE files for full attribution and terms.
