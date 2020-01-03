[![forthebadge](https://forthebadge.com/images/badges/oooo-kill-em.svg)](https://forthebadge.com)

# Chip-8!
Another Chip-8 implementation, woo! 🎉


## ToDo
- [ ] CI/CD
  - [ ] Pick a CI
  - [ ] List tools needed to build
  - [ ] Reproducable builds on push
  - [ ] Serve build artifacts (github pages?)  
- [ ] Display
  - [x] Divide canvas into "pixels"
    - [ ] With support for multiple resolutions
  - [x] Draw to canvas from Rust
  - [x] Draw sprites to canvas
    - [x] With collision detection
- [x] Keyboard
- [ ] Timers
  - [ ] Sounds (w/buzzer)
  - [ ] Delay
- [ ] System
  - [ ] Timers/Delays
  - [ ] Emulation loop MVP
  - [ ] Macros for generic instruction implementation
  - [ ] Instruction set 25% defined
  - [ ] Instruction set 50% defined
  - [ ] Instruction set 90% defined


## Purpose
Two reasons I'm building this:
1. Getting familiar with WASM + Rust.
2. Jump off point for improving my 6502 emulator. Emulating the screen is a problem I haven't solved yet. The Chip-8 screen is pretty simple, so it should be an easy start.


## Chip-8 Documentation
- [CHIP-8 Technical Reference - Matt Mikolay](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference)
- [CHIP-8 Documentation - trapexit; "A collection of documentation on the CHIP-8 and related"](https://github.com/trapexit/chip-8_documentation)
- [CHIP8; A CHIP-8 / SCHIP emulator - David Winter](http://vanbeveren.byethost13.com/stuff/CHIP8.pdf?i=1)
- [HP48-Superchip - Chromatophore](https://github.com/Chromatophore/HP48-Superchip)
- [(SUPER)CHIP 8 Secrets - Petr Kratina](https://github.com/AfBu/haxe-chip-8-emulator/wiki/(Super)CHIP-8-Secrets)


## Quirks and Extensions
There are a lot of popular Chip-8 implementations and since there is no concrete spec for the Chip-8 system, each implementation has it's own quirks and extensions. I'll support what I can based on time/difficulty and guard these with feature flags.


# Building
## Tools Required
// TODO


## Getting Started

```sh
npm install
```

## NPM Commands

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start


# Builds the project and places it into the `dist` folder.
npm run build
```