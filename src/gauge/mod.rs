pub mod dial;
pub mod textgauge;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    primitives::{Arc, Line, PrimitiveStyle, Styled},
    text::Text,
    Drawable,
};

pub enum Digits {
    None,
    Single,
    Two,
}

pub trait SetValue {
    fn set_value(&mut self, value: f32);
}

pub enum Gauge<'a> {
    Dial(dial::Dial<'a>),
    TextGauge(textgauge::TextGauge<'a>),
}

impl SetValue for Gauge<'_> {
    fn set_value(&mut self, value: f32) {
        match self {
            Gauge::Dial(dial) => dial.set_value(value),
            Gauge::TextGauge(textgauge) => textgauge.set_value(value),
        }
    }
}

impl Drawable for Gauge<'_> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self {
            Gauge::Dial(dial) => Ok(dial.draw(target)?),
            Gauge::TextGauge(textgauge) => Ok(textgauge.draw(target)?),
        }
    }
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
