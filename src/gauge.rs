pub mod gauge {

    use embedded_graphics::{
        draw_target::DrawTarget,
        geometry::Size,
        mono_font::{ascii::FONT_10X20, ascii::FONT_4X6, MonoTextStyle},
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{
            Arc, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Styled,
        },
        text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
        Drawable,
    };

    pub struct Dial<'a> {
        pub title: &'a str,
        pub min_value: f32,
        pub max_value: f32,
        pub current_value: f32,
        pub digits: Digits,
        pub start_x: u8,

        bounding: Rectangle,
        arc_stroke: PrimitiveStyle<BinaryColor>,
        outline: PrimitiveStyle<BinaryColor>,
        character_style: MonoTextStyle<'a, BinaryColor>,
        text_style: TextStyle,
        outer_radius: u8,
        inner_radius: u8,
        drawables: Vec<DrawableWrapper<'a>>,
    }

    impl Dial<'_> {
        pub fn new<'a>(
            title: &'a str,
            min_value: f32,
            max_value: f32,
            current_value: f32,
            digits: Digits,
            start_x: u8,
            indicators: &[f32],
        ) -> Dial<'a> {
            let mut ret = Dial {
                title,
                min_value,
                max_value,
                current_value,
                digits,
                start_x,

                bounding: Rectangle::new(Point::new(start_x.into(), 0), Size::new(64, 64)),
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
                outer_radius: 30,
                inner_radius: 26,

                drawables: Vec::new(),
            };

            let center = ret.bounding.center();

            ret.drawables
                .push(DrawableWrapper::Text(Text::with_text_style(
                    title,
                    Point::new(center.x, 20),
                    MonoTextStyle::new(&FONT_4X6, BinaryColor::On),
                    ret.text_style,
                )));

            ret.drawables.push(DrawableWrapper::Arc(
                Arc::with_center(
                    center,
                    (ret.outer_radius * 2).into(),
                    270.0.deg(),
                    -270.0.deg(),
                )
                .into_styled(ret.outline),
            ));

            ret.drawables.push(DrawableWrapper::Arc(
                Arc::with_center(
                    center,
                    (ret.inner_radius * 2).into(),
                    270.0.deg(),
                    -270.0.deg(),
                )
                .into_styled(ret.outline),
            ));

            ret.create_indicator_line(min_value);
            ret.create_indicator_line(max_value);

            let iter = indicators.iter();
            for indicator in iter {
                ret.create_indicator_line(*indicator);
            }

            ret
        }

        fn create_indicator_line(&mut self, value: f32) {
            let percentage = ((value - self.min_value) / (self.max_value - self.min_value)) * 100.0;
            let sweep = 360.0 - ((100.0 - percentage) * 270.0 / 100.0);

            let center = self.bounding.center();
            let inner_point = Point::new(
                center.x + (self.inner_radius as f32 * sweep.to_radians().cos()).round() as i32,
                center.y + (self.inner_radius as f32 * sweep.to_radians().sin()).round() as i32,
            );

            let outer_point = Point::new(
                center.x
                    + ((self.outer_radius - 1) as f32 * sweep.to_radians().cos()).round() as i32,
                center.y
                    + ((self.outer_radius - 1) as f32 * sweep.to_radians().sin()).round() as i32,
            );

            self.drawables.push(DrawableWrapper::Line(
                Line::new(inner_point, outer_point).into_styled(self.outline),
            ));
        }
    }

    impl Drawable for Dial<'_> {
        type Color = BinaryColor;
        type Output = ();

        fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
        where
            D: DrawTarget<Color = BinaryColor>,
        {
            let percentage =
                ((self.current_value - self.min_value) / (self.max_value - self.min_value)) * 100.0;
            let sweep = percentage * 270.0 / 100.0;

            // Draw an arc with a 5px wide stroke.
            let arc = Arc::new(
                Point::new((self.start_x + 2).into(), 2),
                64 - 4,
                270.0.deg(),
                -sweep.deg(),
            );
            arc.into_styled(self.arc_stroke).draw(target)?;

            // Draw centered text.
            let text = match self.digits {
                Digits::None => format!("{:.0}", self.current_value),
                Digits::Single => format!("{:.1}", self.current_value),
                Digits::Two => format!("{:.2}", self.current_value),
            };
            Text::with_text_style(&text, arc.center(), self.character_style, self.text_style)
                .draw(target)?;

            let drawable_iter = self.drawables.iter();
            for drawable in drawable_iter {
                drawable.draw(target)?;
            }

            Ok(())
        }
    }

    pub enum Digits {
        None,
        Single,
        Two,
    }

    enum DrawableWrapper<'a> {
        Arc(Styled<Arc, PrimitiveStyle<BinaryColor>>),
        Line(Styled<Line, PrimitiveStyle<BinaryColor>>),
        Text(Text<'a, MonoTextStyle<'a, BinaryColor>>),
    }

    impl Drawable for DrawableWrapper<'_> {
        type Color = BinaryColor;
        type Output = ();

        fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
        where
            D: DrawTarget<Color = BinaryColor>,
        {
            match self {
                DrawableWrapper::Arc(arc) => Ok(arc.draw(target)?),
                DrawableWrapper::Line(line) => Ok(line.draw(target)?),
                DrawableWrapper::Text(text) => {
                    text.draw(target)?;
                    Ok(())
                }
            }
        }
    }
}
