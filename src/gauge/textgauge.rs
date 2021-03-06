use super::{Digits, DrawableWrapper, SetValue};
use crate::Config;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    prelude::Point,
    primitives::Rectangle,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
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

pub struct TextGauge<'a> {
    pub title: &'a str,
    pub unit: &'a str,
    pub value: f32,
    pub digits: Digits,

    bounding_box: Rectangle,
    character_style: MonoTextStyle<'a, Colour>,
    drawables: Vec<DrawableWrapper<'a>>,
}

impl TextGauge<'_> {
    pub fn new<'a>(
        title: &'a str,
        unit: &'a str,
        value: f32,
        digits: Digits,
        bounding_box: Rectangle,
        config: &Config,
    ) -> TextGauge<'a> {
        #[cfg(feature = "colors")]
        let primary = Rgb888::new(config.colors.primary.r, config.colors.primary.g, config.colors.primary.b);
        #[cfg(not(feature = "colors"))]
        let primary = BinaryColor::On;

        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Left)
            .build();
        let character_style = MonoTextStyle::new(&FONT_6X9, primary);
        let mut drawables: Vec<DrawableWrapper<'a>> = Vec::new();
        let center = bounding_box.center();

        drawables.push(DrawableWrapper::Text(Text::with_text_style(
            title,
            Point::new(bounding_box.top_left.x + 2, center.y),
            character_style,
            text_style,
        )));

        TextGauge {
            title,
            unit,
            value,
            digits,

            bounding_box,
            character_style,
            drawables,
        }
    }
}

impl Drawable for TextGauge<'_> {
    type Color = Colour;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let text_style = TextStyleBuilder::new()
            .baseline(Baseline::Middle)
            .alignment(Alignment::Right)
            .build();
        let value = match self.digits {
            Digits::None => format!("{:.0}", self.value),
            Digits::Single => format!("{:.1}", self.value),
            Digits::Two => format!("{:.2}", self.value),
        };
        Text::with_text_style(
            &format!("{} {}", value, self.unit),
            Point::new(
                self.bounding_box
                    .anchor_point(embedded_graphics::geometry::AnchorPoint::TopRight)
                    .x
                    - 2,
                self.bounding_box.center().y,
            ),
            self.character_style,
            text_style,
        )
        .draw(target)?;

        let drawable_iter = self.drawables.iter();
        for drawable in drawable_iter {
            drawable.draw(target)?;
        }

        Ok(())
    }
}

impl SetValue for TextGauge<'_> {
    fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}
