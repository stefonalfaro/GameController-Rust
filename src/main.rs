use gilrs::{Gilrs, Button, Event, EventType};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("Detected gamepad: {}", gamepad.name());
    }

    loop {
        // You would typically not want a busy loop like this in a real application
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    println!("Gamepad {}: Button {:?} is pressed", id, button);
                },
                EventType::ButtonReleased(button, _) => {
                    println!("Gamepad {}: Button {:?} is released", id, button);
                },
                EventType::AxisChanged(axis, value, _) => {
                    // Implementing a deadzone of 0.05
                    if value.abs() > 0.05 {
                        println!("Gamepad {}: Axis {:?} is at position {}", id, axis, value);
                    }
                },
                _ => {}
            }
        }
    }
}
