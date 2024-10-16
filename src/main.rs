use gilrs::{Event, Gilrs};
use gilrs::ff::{EffectBuilder, BaseEffect, BaseEffectType, Ticks, Replay, Effect};
use std::net::UdpSocket;
use std::time::Instant;
use dotenv::dotenv;
use std::env;

fn update_force_feedback_effect(gilrs: &mut Gilrs, effect: &mut Option<Effect>, angle: i32) -> Result<(), gilrs::ff::Error> {
    let intensity = ((angle as f32 / 180.0) * 32767.0).round() as i16;
    let duration = Ticks::from_ms(100);

    if let Some(existing_effect) = effect {
        existing_effect.stop()?;
    }

    let new_effect = EffectBuilder::new()
        .add_effect(BaseEffect {
            kind: BaseEffectType::Strong { magnitude: intensity as u16 },
            scheduling: Replay { play_for: duration, ..Default::default() },
            envelope: Default::default(),
        })
        .gamepads(&gilrs.gamepads().filter_map(|(id, gp)| {
            if gp.is_ff_supported() { Some(id) } else { None }
        }).collect::<Vec<_>>())
        .finish(gilrs)?;

    new_effect.play()?;
    *effect = Some(new_effect);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut gilrs = Gilrs::new()?;
    let esp32_addr = env::var("ESP32_ADDR").expect("ESP32_ADDR not found in .env");
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut last_angle: i32 = -1;
    let mut last_update = Instant::now();
    let mut ff_effect: Option<Effect> = None;

    println!("Gamepad control started. Press the D-pad button to toggle color.");

    loop {
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            let gamepad = gilrs.gamepad(id);

            match event {
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadUp, _) |
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadDown, _) => {
                    socket.send_to("TOGGLE".as_bytes(), &esp32_addr)?;
                    println!("Toggled color");
                }
                _ => {
                    let rt_value = gamepad.value(gilrs::Axis::LeftZ);
                    let angle = if rt_value > 0.05 {
                        (rt_value * 180.0).round() as i32
                    } else {
                        0
                    };

                    if angle != last_angle || last_update.elapsed().as_millis() >= 16 {
                        last_angle = angle;
                        last_update = Instant::now();
                        let message = angle.to_string();
                        socket.send_to(message.as_bytes(), &esp32_addr)?;
                        println!("Sent angle: {}", angle);

                        if let Err(e) = update_force_feedback_effect(&mut gilrs, &mut ff_effect, angle) {
                            eprintln!("Failed to update force feedback effect: {:?}", e);
                        }
                    }
                }
            }
        }
        gilrs.inc();
    }
}
