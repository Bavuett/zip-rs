use crate::traits::timeable::Timeable;

pub trait Second: Timeable {
    fn set(&mut self, value: u8);
}

impl Second for u8 {
    fn set(&mut self, value: u8) {
        if value > 59 {
            panic!("Second must be between 0 and 59");
        }

        Timeable::set(self, value);
    }
}
