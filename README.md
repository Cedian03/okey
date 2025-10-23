# okey

Keyboard firmware in Rust. 🦀

## Usage

### Logging

To enable logging via defmt enable the `defmt` feature of this crate.

```sh
DEFMT_LOG=info cargo run
```

If you want okey to use error-level logging but the rest of you program to use info-level logging you can do that like this.

```sh
DEFMT_LOG=info,okey=error cargo run
```

## TODO

### Core design

- Determine what assumptions `okey` will make about the design of a keyboard.
- Figure out how actions can be handled.

### Features

- Caps, scroll and numlock support.
- Support for split layouts.
- Implement scanners for non-mechanical switches.
  - Hall-effect
  - TMR
