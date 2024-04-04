# Game Controller Rust
![image](https://github.com/stefonalfaro/GameController-Rust/assets/45152948/5e0f5aaf-b6bb-4ed3-a06f-0b67179b7ff2)

## Deadzone for Joystick
The joystick might seem stationary, but it's very sensitive and can report small movements or drifts even when you're not touching it. This is why you see the position output continuously, even if there's no noticeable movement.

To address this, you can implement a deadzone—a small range around the joystick's neutral (0) position that's considered as no movement. This way, minor drifts or inaccuracies won't be registered as movement.
