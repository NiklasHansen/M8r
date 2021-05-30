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

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Arc, Line, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 64x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 64));

    // Create styles used by the drawing operations.
    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let outline = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Middle)
        .alignment(Alignment::Center)
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Boost", &output_settings);

    // The current progress percentage
    let mut progress = 78;

    'running: loop {
        display.clear(BinaryColor::Off)?;

        let sweep = progress as f32 * 270.0 / 100.0;

        // Draw an arc with a 5px wide stroke.
        let arc = Arc::new(Point::new(2, 2), 64 - 4, 270.0.deg(), -sweep.deg());
        arc.into_styled(arc_stroke).draw(&mut display)?;

        Arc::with_center(arc.center(), 64 - 4, 270.0.deg(), -270.0.deg())
            .into_styled(outline)
            .draw(&mut display)?;

        Arc::with_center(arc.center(), 64 - 12, 270.0.deg(), -270.0.deg())
            .into_styled(outline)
            .draw(&mut display)?;

        Line::new(Point::new(57, 31), Point::new(61, 31))
            .into_styled(outline)
            .draw(&mut display)?;

        Line::new(Point::new(2, 31), Point::new(6, 31))
            .into_styled(outline)
            .draw(&mut display)?;

        // Draw centered text.
        let text = format!("{}%", progress);
        Text::with_text_style(
            &text,
            arc.center(),
            character_style,
            text_style,
        )
        .draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % 101;
    }
}
