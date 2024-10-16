# RS Power Controller

Allows connecting a gamepad to [this ESP32 servo arm controller](https://github.com/QuantGeekDev/rs-power-trigger).

Has Force Feedback (vibration) for supported controllers on servo arm angle position
- Connects to rs-power-trigger servo arm over local network
- Press Left-Trigger to increment the servo angle from 0 to 180 degrees based on pressure
- Press controlPad up or down to change the indicator LED color

# Build
1. Run `cargo build`
2. Run `cargo run`