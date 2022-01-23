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

#[cfg(not(feature = "colors"))]
use embedded_graphics::pixelcolor::BinaryColor;
#[cfg(feature = "colors")]
use embedded_graphics::pixelcolor::Rgb888;

use embedded_graphics::{prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
#[cfg(not(feature = "colors"))]
use embedded_graphics_simulator::BinaryColorTheme;
use gauge::{dial::Dial, textgauge::TextGauge, Digits, SetValue};
use socketcan::CANSocket;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use crate::config::{Config, Gauge, GaugeType};

mod gauge;
mod config;

struct GaugeSetup<'a> {
    gauge: gauge::Gauge<'a>,
    config: &'a Gauge,
}

impl GaugeSetup<'_> {
    fn new<'a>(gauge: gauge::Gauge<'a>, config: &'a Gauge) -> GaugeSetup<'a> {
        GaugeSetup { gauge, config }
    }
}

fn main() -> Result<(), std::convert::Infallible> {
    let input_file = "./config.toml";
    let mut file = File::open(input_file).unwrap();
    let mut file_content = String::new();
    let _bytes_read = file.read_to_string(&mut file_content).unwrap();
    let config: Config = toml::from_str(&&file_content).unwrap();

    #[cfg(not(feature = "colors"))]
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(config.width, config.height));
    #[cfg(feature = "colors")]
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(config.width, config.height));

    #[cfg(feature = "colors")]
    let output_settings = OutputSettingsBuilder::new()
        .scale(3)
        .build();
    #[cfg(not(feature = "colors"))]
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("m8r", &output_settings);

    let mut gauges: HashMap<u32, Vec<GaugeSetup>> = HashMap::new();
    for gauge_config in config.gauges.iter() {
        if !gauges.contains_key(&gauge_config.frame_id) {
            gauges.insert(gauge_config.frame_id, Vec::new());
        }

        let list = gauges.get_mut(&gauge_config.frame_id).unwrap();

        let digits = match gauge_config.digits {
            0 => Digits::None,
            1 => Digits::Single,
            _ => Digits::Two,
        };

        match gauge_config.gauge {
            GaugeType::Dial => {
                let dial = Dial::new(
                    &gauge_config.title,
                    gauge_config.min_value.unwrap(),
                    gauge_config.max_value.unwrap(),
                    gauge_config.min_value.unwrap(),
                    digits,
                    Rectangle::new(
                        Point::new(gauge_config.point.x, gauge_config.point.y),
                        Size::new(gauge_config.size.width, gauge_config.size.height),
                    ),
                    gauge_config.indicators.as_ref().unwrap().as_slice(),
                    &config,
                );
                list.push(GaugeSetup::new(gauge::Gauge::Dial(dial), &gauge_config));
            }
            GaugeType::TextGauge => {
                let textgauge = TextGauge::new(
                    &gauge_config.title,
                    &gauge_config.unit,
                    0.0,
                    digits,
                    Rectangle::new(
                        Point::new(gauge_config.point.x, gauge_config.point.y),
                        Size::new(gauge_config.size.width, gauge_config.size.height),
                    ),
                    &config,
                );
                list.push(GaugeSetup::new(
                    gauge::Gauge::TextGauge(textgauge),
                    &gauge_config,
                ));
            }
        }
    }

    // TODO: Set up filter, to filter out frames not relevant.
    let socket = CANSocket::open(&config.interface).unwrap();
    let target_fps = 30;
    let time_per_frame = Duration::from_millis(1000 / target_fps);

    #[cfg(feature = "colors")]
    let background = Rgb888::new(config.colors.background.r, config.colors.background.g, config.colors.background.b);
    #[cfg(not(feature = "colors"))]
    let background = BinaryColor::Off;

    'running: loop {
        let frame_start = std::time::Instant::now();

        display.clear(background)?;

        for (_, gauge_slots) in gauges.iter() {
            for gauge_setup in gauge_slots.iter() {
                gauge_setup.gauge.draw(&mut display)?;
            }
        }

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
                            Result::Ok(f) => {
                                let frame_gauges = gauges.get_mut(&f.id());
                                match frame_gauges {
                                    Some(fgauges) => {
                                        for gauge_setup in fgauges.iter_mut() {
                                            let slot_start =
                                                (gauge_setup.config.slot_id - 1) * config.slot_size;
                                            let value = gauge_setup.config.data_type.value(&f.data(), slot_start);
                                            gauge_setup.gauge.set_value(value);
                                        }
                                    }
                                    None => continue,
                                }
                            }
                            Result::Err(_) => continue,
                        };
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}
