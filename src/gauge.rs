pub mod gauge {

    use embedded_graphics::{
        draw_target::DrawTarget,
        mono_font::{ascii::FONT_10X20, MonoTextStyle},
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{Arc, Line, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
        Drawable,
    };

    pub struct Dial<'a> {
        pub title: String,
        pub min_value: f32,
        pub max_value: f32,
        pub current_value: f32,

        arc_stroke: PrimitiveStyle<BinaryColor>,
        outline: PrimitiveStyle<BinaryColor>,
        character_style: MonoTextStyle<'a, BinaryColor>,
        text_style: TextStyle,
    }

    impl Dial<'_> {
        pub fn new<'a>(
            title: String,
            min_value: f32,
            max_value: f32,
            current_value: f32,
        ) -> Dial<'a> {
            Dial {
                title,
                min_value,
                max_value,
                current_value,

                arc_stroke: PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(5)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
                outline: PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(1)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
                character_style: MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
                text_style: TextStyleBuilder::new()
                    .baseline(Baseline::Middle)
                    .alignment(Alignment::Center)
                    .build(),
            }
        }
    }

    impl Drawable for Dial<'_> {
        type Color = BinaryColor;
        type Output = ();

        fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
        where
            D: DrawTarget<Color = BinaryColor>,
        {
            let percentage = ((self.current_value - self.min_value) / (self.max_value - self.min_value)) * 100.0;
            let sweep = percentage * 270.0 / 100.0;

            // Draw an arc with a 5px wide stroke.
            let arc = Arc::new(Point::new(2, 2), 64 - 4, 270.0.deg(), -sweep.deg());
            arc.into_styled(self.arc_stroke).draw(target)?;

            Arc::with_center(arc.center(), 64 - 4, 270.0.deg(), -270.0.deg())
                .into_styled(self.outline)
                .draw(target)?;

            Arc::with_center(arc.center(), 64 - 12, 270.0.deg(), -270.0.deg())
                .into_styled(self.outline)
                .draw(target)?;

            Line::new(Point::new(57, 31), Point::new(61, 31))
                .into_styled(self.outline)
                .draw(target)?;

            Line::new(Point::new(2, 31), Point::new(6, 31))
                .into_styled(self.outline)
                .draw(target)?;

            // Draw centered text.
            let text = format!("{:.2}", self.current_value);
            Text::with_text_style(&text, arc.center(), self.character_style, self.text_style)
                .draw(target)?;

            Ok(())
        }
    }
}
