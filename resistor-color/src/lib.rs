use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(r) => format!("{:?}", r),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // (0..9).map(|x: usize| value_to_color_string(x))
    let mut v = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
    v.sort_by(|a, b| a.int_value().cmp(&b.int_value()));
    v
}
