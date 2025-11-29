# state

State management traits enabling dependency injection and loose coupling.

## Crates

- **tt-rs-state**: Composable substates for single-responsibility:
  - `Position`: 2D coordinates
  - `PositionStore`: Widget position management
  - `BoxContents`: Box hole contents management
  - `TrainingState`: Robot training mode management

## Dependency Level

Depends on: core

## Design

Uses composable substates to avoid function explosion and separate concerns.
