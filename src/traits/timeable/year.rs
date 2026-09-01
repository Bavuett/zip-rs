use crate::traits::timeable::Timeable;

pub trait Year: Timeable {
    fn set(&mut self, value: u8);
}

impl Year for u8 {
    fn set(&mut self, value: u8) {
        if value < 1980 {
            panic!("Year must be minimum 1980");
        }

        Timeable::set(self, value);
    }
}
