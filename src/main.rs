//// Gauges:
//// - Oil pressure
//// - Oil temp
//// - MAP
//// - IAT
//// - Coolant temp
//// - EGT?
//// - Ignition retard?
//// - Cruise control?
//// - Error? Limp mode?
//// - Fuel pressure?
//// - Lambda?
//// - Knock warn light?

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use gauge::gauge::Dial;
use gauge::gauge::Digits;
use socketcan::CANSocket;
use std::time::Duration;

mod gauge;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Boost", &output_settings);

    let mut boost = Dial::new("Boost", -1.0, 2.0, 1.2, Digits::Two, 0, &[0.0]);
    let /*mut*/ oiltemp = Dial::new("Oil temp", 0.0, 150.0, 70.0, Digits::None, 64, &[80.0]);
    let /*mut*/ oilpres = Dial::new("Oil pres", 0.0, 10.0, 0.0, Digits::Single, 128, &[2.0, 4.0, 6.0, 8.0]);

    let socket = CANSocket::open("vcan0").unwrap();
    let target_fps = 30;
    let time_per_frame = Duration::from_millis(1000 / target_fps);

    'running: loop {
        let frame_start = std::time::Instant::now();
        display.clear(BinaryColor::Off)?;

        boost.draw(&mut display)?;
        oiltemp.draw(&mut display)?;
        oilpres.draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        loop {
            let time_to_next_frame =
                time_per_frame.checked_sub(std::time::Instant::now().duration_since(frame_start));
            match time_to_next_frame {
                Some(time) => {
                    if time.as_millis() > 0 {
                        socket.set_read_timeout(time).unwrap();
                        let frame = socket.read_frame();
                        match frame {
                            Result::Ok(f) => println!("{}", f.id()),
                            Result::Err(_) => println!("No frame to read"),
                        };
                    } else {
                        break;
                    }
                }
                None => {
                    println!("Time for next frame");
                    break;
                }
            }
        }

        // This is just for testing purposes
        boost.current_value += 0.05;
        if boost.current_value > boost.max_value {
            boost.current_value = boost.min_value;
        }
    }
}
