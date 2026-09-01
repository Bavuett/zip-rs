use crate::traits::timeable::Timeable;

pub trait Minute: Timeable {
    fn set(&mut self, value: u8);
}

impl Minute for u8 {
    fn set(&mut self, value: u8) {
        if value > 59 {
            panic!("Minute must be between 0 and 59");
        }

        Timeable::set(self, value);
    }
}
