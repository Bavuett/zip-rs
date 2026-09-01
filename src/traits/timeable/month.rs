use crate::traits::timeable::Timeable;

pub trait Month: Timeable {
    fn set(&mut self, value: u8);
}

impl Month for u8 {
    fn set(&mut self, value: u8) {
        if value < 1 || value > 12 {
            panic!("Month must be between 1 and 12");
        }

        Timeable::set(self, value);
    }
}
