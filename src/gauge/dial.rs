use super::{Digits, DrawableWrapper, SetValue};
use crate::Config;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_10X20, ascii::FONT_6X9, MonoTextStyle},
    prelude::*,
    primitives::{Arc, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
    Drawable,
};

#[cfg(feature = "colors")]
use embedded_graphics::pixelcolor::Rgb888;
#[cfg(not(feature = "colors"))]
use embedded_graphics::pixelcolor::BinaryColor;

#[cfg(feature = "colors")]
type Colour = Rgb888;
#[cfg(not(feature = "colors"))]
type Colour = BinaryColor;

pub struct Dial<'a> {
    pub title: &'a str,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
    pub digits: Digits,
    pub bounding: Rectangle,

    arc_stroke: PrimitiveStyle<Colour>,
    outline: PrimitiveStyle<Colour>,
    character_style: MonoTextStyle<'a, Colour>,
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
        bounding: Rectangle,
        indicators: &[f32],
        config: &Config,
    ) -> Dial<'a> {
        #[cfg(feature = "colors")]
        let primary = Rgb888::new(config.colors.primary.r, config.colors.primary.g, config.colors.primary.b);
        #[cfg(not(feature = "colors"))]
        let primary = BinaryColor::On;

        let mut ret = Dial {
            title,
            min_value,
            max_value,
            current_value,
            digits,
            bounding,

            arc_stroke: PrimitiveStyleBuilder::new()
                .stroke_color(primary)
                .stroke_width(5)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
            outline: PrimitiveStyleBuilder::new()
                .stroke_color(primary)
                .stroke_width(2)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
            character_style: MonoTextStyle::new(&FONT_10X20, primary),
            text_style: TextStyleBuilder::new()
                .baseline(Baseline::Middle)
                .alignment(Alignment::Center)
                .build(),
            outer_radius: 56,
            inner_radius: 48,

            drawables: Vec::new(),
        };

        let center = ret.bounding.center();

        ret.drawables
            .push(DrawableWrapper::Text(Text::with_text_style(
                title,
                Point::new(center.x, 40),
                MonoTextStyle::new(&FONT_6X9, primary),
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
            center.x + ((self.outer_radius - 1) as f32 * sweep.to_radians().cos()).round() as i32,
            center.y + ((self.outer_radius - 1) as f32 * sweep.to_radians().sin()).round() as i32,
        );

        self.drawables.push(DrawableWrapper::Line(
            Line::new(inner_point, outer_point).into_styled(self.outline),
        ));
    }
}

impl Drawable for Dial<'_> {
    type Color = Colour;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let percentage =
            ((self.current_value - self.min_value) / (self.max_value - self.min_value)) * 100.0;
        let sweep = percentage * 270.0 / 100.0;

        // Draw an arc with a 5px wide stroke.
        let arc = Arc::new(
            Point::new((self.bounding.top_left.x + 2).into(), 2),
            112,
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

impl SetValue for Dial<'_> {
    fn set_value(&mut self, value: f32) {
        self.current_value = value;
    }
}
