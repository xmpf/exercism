use enum_iterator::{all, last, Sequence};

#[derive(Debug, PartialEq, Eq, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match value > color_to_value(last::<ResistorColor>().unwrap()) {
        true => String::from("value out of range"),
        _ => format!("{:?}", colors()[value as usize])
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}