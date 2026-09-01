pub mod day;
pub mod hour;
pub mod minute;
pub mod month;
pub mod second;
pub mod year;

pub trait Timeable {
    fn set(&mut self, value: u8);
}

impl Timeable for u8 {
    fn set(&mut self, value: u8) {
        *self = value;
    }
}
