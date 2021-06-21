pub mod dial;
pub mod textgauge;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    primitives::{
        Arc, Line, PrimitiveStyle, Styled,
    },
    text::Text,
    Drawable,
};

pub enum Digits {
    None,
    Single,
    Two,
}

pub trait SetValue {
    fn set_name(&mut self, value: f32);
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
