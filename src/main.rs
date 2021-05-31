//use futures_util::stream::StreamExt;
//use tokio_socketcan::{CANSocket, Error};

//#[tokio::main]
//async fn main() -> Result<(), Error> {
//let mut socket_rx = CANSocket::open("vcan0")?;

//// Gauges:
//// - Oil pressure
//// - Oil temp
//// - MAP
//// - Coolant temp
//// - EGT?
//// - Ignition retard?
//// - Cruise control?
//// - Error? Limp mode?
//// - Fuel pressure?
//// - Fuel composition / ethanol content?
//// - Lambda?
//// - Knock warn light?

////socket_rx.set_filter(filters: &[socketcan::CANFilter]);
//while let Some(Ok(frame)) = socket_rx.next().await {
//println!("{}", frame.id());
//}
//Ok(())
//}

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use gauge::gauge::Dial;
use std::{thread, time::Duration};
use gauge::gauge::{Digits};

mod gauge;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Boost", &output_settings);

    let mut boost = Dial::new("Boost".to_string(), -1.0, 2.0, 1.2, Digits::Two, 0);
    let mut oiltemp = Dial::new("Oil temp".to_string(), 0.0, 150.0, 80.0, Digits::None, 64);
    let mut oilpres = Dial::new("Oil pres".to_string(), 0.0, 10.0, 4.0, Digits::Single, 128);

    'running: loop {
        display.clear(BinaryColor::Off)?;

        boost.draw(&mut display)?;
        oiltemp.draw(&mut display)?;
        oilpres.draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        boost.current_value += 0.05;
        if boost.current_value > boost.max_value {
            boost.current_value = boost.min_value;
        }
    }
}
