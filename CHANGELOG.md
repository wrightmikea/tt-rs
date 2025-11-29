# Changelog

All notable changes to tt-rs.

## 2025-11-29

- deploy: Update live demo with user level selector and tt2 help (`61abe9c`)
- fix: tt2 help shows only messaging and about sections (`839bd30`)
- feat: Add user level selector and Bird/Nest messaging (`a7cbcb5`)
- feat: Add Bird widget for Phase 2 messaging system (`4e3a36a`)
- fix: Tooltip z-index so it appears above other widgets (`6da105a`)
- fix: Use tt-nest.svg asset instead of inline SVG (`3c82e65`)
- fix: Add cache-busting and disable SRI for development (`91b9314`)
- feat: Add Nest widget for message receiving (Phase 2) (`d9cd097`)
- refactor: Reorganize components by programming concept (`8f19ce9`)
- docs: Comprehensive documentation update with accurate status and roadmap (`e41f2bf`)

## 2025-11-28

- deploy: Update live demo with arithmetic fix (`0cb20f3`)
- fix: Use effective numerator for target in arithmetic operations (`6bd94bc`)
- fix: Reset operator to Add after arithmetic operations (`aeb6316`)
- fix: UI bug fixes for number display and tooltip persistence (`011fe11`)
- refactor: Split modules to meet sw-checklist function limits (`7298c1a`)
- docs: Add Code Quality Refactoring Plan (`b8b985a`)
- docs: Update screenshot with help panel showing About section (`56ba7a4`)
- fix: Show + prefix on number stacks, add -1 tool and About section (`271cd7c`)
- feat: Add tooltips and help panel for new users (`460fcd0`)
- docs: Update README screenshot with current demo state (`015f31e`)
- feat: Move robot widget to right of /2 stack for better UX (`fbcb932`)
- feat: Add Robot and Wand widgets, improve scales and vacuum behavior (`b5ccfaf`)
- docs: Add original ToonTalk concurrency features to PRD and plan (`cee964c`)
- feat: Add boxes to demo and fix vacuum to erase box contents (`c32fbe4`)
- feat: Add Scales and Vacuum widgets with SVG graphics (`0497e77`)
- wip: Refactor widgets for sw-checklist modularity compliance (`0eab8d2`)

## 2025-11-27

- docs: Update README screenshot and add ToonTalk links (`8be5d80`)
- fix: Update paths for multi-component architecture and rebuild docs (`00dffd1`)
- refactor: Migrate to multi-component architecture (`bc29088`)
- feat: Add "Make 10" tutorial with copy sources and improved UX (`b156e8a`)
- feat: Implement drag-and-drop widgets into box holes (`d7cf990`)
- fix: Fix drag-and-drop so mouseup releases the widget (`ff1a1ac`)
- fix: Restore GitHub Pages deployment with correct asset paths (`e34613f`)
- feat: Add Text, Box widgets and drag-and-drop interaction (`efa441d`)
- refactor: Convert to Rust workspace with modular crates (`db3acb5`)
- docs: Add learnings.md with WASM development solutions (`6ecae16`)
- fix: Use correct public URL for GitHub Pages (`2aacde4`)
- feat: Add GitHub Pages live demo and screenshot (`6a52064`)
- feat: Implement Yew/WASM scaffolding and Number widget (`cc1a81d`)
- docs: Add project documentation and structure for tt-rs (`486bc28`)
- Initial commit (`d98f63d`)
