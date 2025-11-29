# tt-rs Roadmap

This document captures future plans and ideas not yet implemented in tt-rs. For current implementation status, see [plan.md](plan.md). For product requirements, see [prd.md](prd.md).

## Near-Term Ideas

### Layout Persistence

**Goal:** Save and restore widget positions so users can customize their workspace.

**Features:**
- Save current layout to localStorage
- Load layout on startup
- Reset layout to defaults
- Export/import layout as JSON

**Motivation:** Users should be able to arrange widgets to their preference and have it persist across sessions.

---

### Stack Dragging (Restore)

**Goal:** Make copy source stacks (prototypes) draggable so users can rearrange the palette.

**Current State:** Stacks only support click-to-copy, not drag-to-move.

**Implementation:** Add draggable behavior to CopySource component while preserving click-to-copy.

---

### Multi-Select Operations

**Goal:** Select multiple widgets and perform bulk operations.

**Features:**
- Shift+click to add to selection
- Rectangle select (drag empty space)
- Move/delete selected widgets together

---

## Medium-Term Ideas

### Undo/Redo System

**Goal:** Let users reverse mistakes without losing work.

**Features:**
- Command pattern for all state changes
- Undo stack with configurable depth
- Keyboard shortcuts (Ctrl+Z, Ctrl+Shift+Z)

---

### Touch/Mobile Support

**Goal:** Make tt-rs usable on tablets and touch devices.

**Features:**
- Touch events for drag-and-drop
- Pinch-to-zoom workspace
- Mobile-friendly layout option
- Responsive help panel

---

### Keyboard Shortcuts

**Goal:** Power-user efficiency through keyboard navigation.

**Features:**
- Delete key to remove selected widget
- Arrow keys to nudge position
- Tab to cycle through widgets
- Escape to deselect

---

## Long-Term Ideas

### Tutorial Mode

**Goal:** Guided walkthroughs that teach ToonTalk concepts step by step.

**Features:**
- Highlighted targets ("click here")
- Progress tracking
- Achievement badges
- Prerequisite checking

---

### Collaborative Workspace

**Goal:** Multiple users editing the same workspace in real-time.

**Features:**
- WebSocket synchronization
- Cursor presence indicators
- Conflict resolution
- Chat/comments

---

### Sound and Speech

**Goal:** Audio feedback and accessibility.

**Features:**
- Sound effects for actions (drop, copy, delete)
- Text-to-speech for widget descriptions
- Optional narration mode
- Audio cues for robots executing

---

### 3D Graphics Mode

**Goal:** Restore the original ToonTalk's 3D environment.

**Features:**
- Three.js rendering backend
- 3D widget models
- Camera navigation (helicopter view)
- Houses/city visualization

---

## Implementation Notes

When implementing any roadmap item:

1. Create a design doc if the feature is complex
2. Add to plan.md when work begins
3. Update prd.md if it affects requirements
4. Document any ToonTalk deviations in [deviations.md](deviations.md)
5. Add regression tests for new functionality
6. Deploy to live demo immediately after completion

---

## Contributing Ideas

To suggest new features or changes:

1. Open a GitHub issue with the "enhancement" label
2. Describe the use case and motivation
3. Reference similar features in ToonTalk if applicable
4. Discuss in the issue before starting implementation

---

## References

- [plan.md](plan.md) - Current implementation roadmap
- [prd.md](prd.md) - Product requirements
- [deviations.md](deviations.md) - Design decisions differing from ToonTalk
- [use-cases.md](use-cases.md) - Concrete examples driving priorities
