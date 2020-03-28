[![forthebadge](https://forthebadge.com/images/badges/oooo-kill-em.svg)](https://forthebadge.com)

# Chip-8!
Another Chip-8 implementation, woo! 🎉

## ToDo
- [ ] CI/CD
  - [ ] Builds on push
  - [ ] Serve build artifacts (github pages?)  
- [ ] Display
  - [ ] Support for multiple resolutions
- [ ] Sounds (w/buzzer)
- [ ] Fix bugs
  - [ ] Intense flickering on some roms
  - [ ] Drawing on the very edge of the screen causes panic
  - [ ] Some roms just don't do anything

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


## Built-In Roms
- [test_opcode.ch8: corax89](https://github.com/corax89/chip8-test-rom)
- [Chip-8 Program Pack](https://github.com/dmatlack/chip8/tree/master/roms)
  - Various authors, see `static/roms/chip8_program_pack/README.md`


# Building
## Tools Required
// TODO, sorry :/


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