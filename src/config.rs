use serde::Deserialize;
use half::f16;
use std::convert::TryInto;

#[derive(Deserialize)]
pub struct Config {
    pub interface: String,
    pub slot_size: u8,
    pub width: u32,
    pub height: u32,
    pub gauges: Vec<Gauge>,

    #[cfg(feature = "colors")]
    pub colors: Colors,
}

#[derive(Deserialize)]
pub struct Gauge {
    pub frame_id: u32,
    pub slot_id: u8, // 1-indexed
    pub gauge: GaugeType,
    pub data_type: GaugeDataType,
    pub title: String,
    pub unit: String,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub indicators: Option<Vec<f32>>,
    pub digits: u8,
    pub point: StartPoint,
    pub size: AreaSize,
}

#[derive(Deserialize)]
pub enum GaugeType {
    Dial,
    TextGauge,
}

#[derive(Deserialize)]
pub enum GaugeDataType {
    F16,
    U16,
    I16,
    U8,
    I8,
    B8,
    B16,
}

#[cfg(feature = "colors")]
#[derive(Deserialize)]
pub struct Colors {
    pub primary: Rgb,
    pub background: Rgb,
}

#[cfg(feature = "colors")]
#[derive(Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl GaugeDataType {
    pub fn slot_size(&self) -> u8 {
        match self {
            GaugeDataType::F16 => 2,
            GaugeDataType::U16 => 2,
            GaugeDataType::I16 => 2,
            GaugeDataType::U8 => 1,
            GaugeDataType::I8 => 1,
            GaugeDataType::B8 => 1,
            GaugeDataType::B16 => 2,
        }
    }

    pub fn value(&self, data: &[u8], slot_start: u8) -> f32 {
        let slot_end: usize = (slot_start + self.slot_size()).into();
        let sliced_data = &data[slot_start.into()..slot_end];

        match self {
            &GaugeDataType::F16 => {
                let d: &[u8; 2] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return f16::from_be_bytes(*d).into();
            },
            &GaugeDataType::U16 => {
                let d: &[u8; 2] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return u16::from_be_bytes(*d).into();
            },
            &GaugeDataType::I16 => {
                let d: &[u8; 2] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return i16::from_be_bytes(*d).into();
            },
            &GaugeDataType::U8 => {
                let d: &[u8; 1] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return u8::from_be_bytes(*d).into();
            },
            &GaugeDataType::I8 => {
                let d: &[u8; 1] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return i8::from_be_bytes(*d).into();
            },
            &GaugeDataType::B8 => {
                let d: &[u8; 1] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return u8::from_be_bytes(*d).into();
            },
            &GaugeDataType::B16 => {
                let d: &[u8; 2] = &sliced_data[slot_start.into()..slot_end]
                    .try_into()
                    .expect("Failure");
                return u16::from_be_bytes(*d).into();
            },
        }
    }
}

#[derive(Deserialize)]
pub struct StartPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize)]
pub struct AreaSize {
    pub width: u32,
    pub height: u32,
}
