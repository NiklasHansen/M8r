use super::DrawableWrapper;

pub struct TextGauge<'a> {
    pub title: &'a str,
    pub unit: &'a str,
    pub value: f32,

    drawables: Vec<DrawableWrapper<'a>>,
}

impl TextGauge<'_> {
    pub fn new<'a>(title: &'a str, unit: &'a str, value: f32) -> TextGauge<'a> {
        TextGauge {
            title,
            unit,
            value,

            drawables: Vec::new(),
        }
    }
}
