pub mod hour;
pub mod minute;
pub mod second;

pub trait Timeable {
    fn set(&mut self, value: u8);
}

impl Timeable for u8 {
    fn set(&mut self, value: u8) {
        *self = value;
    }
}
