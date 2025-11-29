# User Avatars

This document describes representative users (avatars) for tt-rs. These personas guide feature development, tutorial design, and documentation priorities.

## Avatar 1: Alex (Beginner/Child)

**Profile:**
- Age: 8-14 years old
- Programming experience: None or minimal (Scratch exposure)
- Goal: Learn programming concepts through play
- Context: School computer lab, home learning, after-school program

**Characteristics:**
- Learns by doing, not reading
- Short attention span - needs immediate feedback
- Motivated by visible results and "cool" visuals
- May not understand abstract concepts like "variables"
- Comfortable with drag-and-drop interfaces

**Needs:**
- Clear visual feedback for every action
- Undo capability (mistakes should be recoverable)
- Simple, concrete examples (counting, basic math)
- Encouraging messages, no intimidating error states
- Guided tutorials with specific goals

**Success Metrics:**
- Can complete "Make 10" tutorial without help
- Understands that dragging numbers combines them
- Can train a robot to repeat a simple action
- Feels accomplished, wants to continue exploring

**Frustration Points:**
- Unclear what to do next
- Actions that have no visible effect
- Technical jargon in help text
- Complex multi-step operations without guidance

---

## Avatar 2: Jordan (Hobbyist/Enthusiast)

**Profile:**
- Age: 16-45 years old
- Programming experience: Some (Python basics, spreadsheets, maybe JavaScript)
- Goal: Explore visual programming paradigms, learn concurrent programming
- Context: Personal projects, self-directed learning, curiosity-driven

**Characteristics:**
- Willing to experiment and figure things out
- Appreciates elegant abstractions
- Interested in "how it works" not just "what it does"
- May compare to other visual languages (Scratch, LabVIEW, Node-RED)
- Values documentation and examples

**Needs:**
- Clear mental model of how widgets interact
- Documentation of advanced features (Bird/Nest messaging)
- Non-trivial example programs that do something "real"
- Ability to save and share creations
- Understanding of concurrency model (robots, messaging)

**Success Metrics:**
- Can build a simple calculator with multiple operations
- Understands Bird/Nest as actor-model messaging
- Can train robots to work together on a task
- Sees potential for building interesting programs

**Frustration Points:**
- Unclear semantics (what happens when X meets Y?)
- Missing features that seem obvious
- No way to see "under the hood"
- Limited to toy examples

---

## Avatar 3: Casey (Developer/Contributor)

**Profile:**
- Age: 20-55 years old
- Programming experience: Professional (Rust, JavaScript, systems programming)
- Goal: Contribute features, fix bugs, understand architecture
- Context: Open source contribution, academic research, tooling development

**Characteristics:**
- Reads source code fluently
- Expects professional tooling (tests, CI, documentation)
- Values clean architecture and clear abstractions
- May be interested in ToonTalk history/research
- Evaluates code quality and maintainability

**Needs:**
- Clear architecture documentation
- Build instructions that work first time
- Test coverage and CI pipeline
- Contribution guidelines
- Understanding of ToonTalk semantics (original + Reborn)

**Success Metrics:**
- Can build project from source without issues
- Understands Widget trait and component structure
- Can add a new widget type following existing patterns
- Can run and write tests

**Frustration Points:**
- Incomplete documentation
- Unclear code organization
- Missing tests for edge cases
- Deviation from original ToonTalk semantics without explanation

---

## Avatar Usage in Development

### Feature Prioritization

When adding features, consider impact on each avatar:

| Feature | Alex Impact | Jordan Impact | Casey Impact |
|---------|-------------|---------------|--------------|
| Tooltips | HIGH - needs guidance | MEDIUM - helpful | LOW - reads code |
| Save/Load | MEDIUM - wants to keep work | HIGH - essential | MEDIUM - for testing |
| Bird/Nest | LOW - advanced topic | HIGH - key differentiator | HIGH - core semantics |
| Documentation | LOW - prefers doing | HIGH - studies docs | HIGH - needs arch docs |

### Tutorial Design

- **Alex tutorials**: Concrete goals, step-by-step, immediate rewards
- **Jordan tutorials**: Concept explanations, "why" not just "how"
- **Casey tutorials**: Architecture overview, contribution workflow

### User Level Mapping

| Avatar | Primary Level | Secondary Level |
|--------|---------------|-----------------|
| Alex | tt1 (Basic) | - |
| Jordan | tt1 (Basic) | tt2 (Messaging) |
| Casey | All levels | Development mode |

---

## References

- Original ToonTalk research: http://toontalk.com/English/papers.htm
- ToonTalk Reborn: https://github.com/ToonTalk/ToonTalk
- Visual programming pedagogy research (Papert, Kay, Resnick)
