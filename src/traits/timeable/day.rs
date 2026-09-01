use crate::traits::timeable::Timeable;

pub trait Day: Timeable {
    fn set(&mut self, value: u8);
}

impl Day for u8 {
    fn set(&mut self, value: u8) {
        if value > 31 {
            panic!("Hour must be between 0 and 23");
        }

        Timeable::set(self, value);
    }
}
