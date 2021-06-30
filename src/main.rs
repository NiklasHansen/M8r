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

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use gauge::{dial::Dial, textgauge::TextGauge, Digits, SetValue};
use half::f16;
use serde::Deserialize;
use socketcan::CANSocket;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

mod gauge;

#[derive(Deserialize)]
struct Config {
    interface: String,
    slot_size: u8,
    gauges: Vec<Gauge>,
}

#[derive(Deserialize)]
struct Gauge {
    frame_id: u32,
    slot_id: u8, // 1-indexed
    gauge: GaugeType,
    title: String,
    unit: String,
    min_value: Option<f32>,
    max_value: Option<f32>,
    indicators: Option<Vec<f32>>,
    digits: u8,
    point: StartPoint,
    size: AreaSize,
}

#[derive(Deserialize)]
enum GaugeType {
    Dial,
    TextGauge,
}

#[derive(Deserialize)]
struct StartPoint {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct AreaSize {
    width: u32,
    height: u32,
}

struct GaugeSetup<'a> {
    gauge: gauge::Gauge<'a>,
    config: &'a Gauge, 
}

impl GaugeSetup<'_> {
    fn new<'a>(gauge: gauge::Gauge<'a>, config: &'a Gauge) -> GaugeSetup<'a> {
        GaugeSetup {
            gauge,
            config,
        }
    }
}

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 64));

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("m8r", &output_settings);

    let input_file = "./config.toml";
    let mut file = File::open(input_file).unwrap();
    let mut file_content = String::new();
    let _bytes_read = file.read_to_string(&mut file_content).unwrap();
    let config: Config = toml::from_str(&&file_content).unwrap();

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
                );
                list.push(GaugeSetup::new(gauge::Gauge::TextGauge(textgauge), &gauge_config));
            }
        }
    }

    // TODO: Set up filter, to filter out frames not relevant.
    let socket = CANSocket::open(&config.interface).unwrap();
    let target_fps = 30;
    let time_per_frame = Duration::from_millis(1000 / target_fps);

    'running: loop {
        let frame_start = std::time::Instant::now();
        display.clear(BinaryColor::Off)?;

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
                                            let slot_start = (gauge_setup.config.slot_id - 1) * config.slot_size;
                                            let slot_end: usize =
                                                (slot_start + config.slot_size).into();
                                            let data: &[u8; 2] = &f.data()
                                                [slot_start.into()..slot_end]
                                                .try_into()
                                                .expect("Failure");
                                            let value = f16::from_be_bytes(*data);
                                            gauge_setup.gauge.set_value(value.into());
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
                    println!("Time for next frame");
                    break;
                }
            }
        }
    }
}
