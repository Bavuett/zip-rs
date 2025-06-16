use std::io::BufReader;

use crate::utils::constants::ConstantValues;

pub trait Validatable {
    fn is_zip(&self) -> bool;
}

impl Validatable for Vec<u8> {
    fn is_zip(&self) -> bool {
        if self.len() < 4 { return false; }
        self[0..4] == ConstantValues::ZIP_SIGNATURE
    }
}

impl Validatable for BufReader<std::fs::File> {
    fn is_zip(&self) -> bool {
        false
    }
}