use crate::traits::timeable::Timeable;

pub trait Hour: Timeable {
    fn set(&mut self, value: u8);
}

impl Hour for u8 {
    fn set(&mut self, value: u8) {
        if value > 23 {
            panic!("Hour must be between 0 and 23");
        }

        Timeable::set(self, value);
    }
}
