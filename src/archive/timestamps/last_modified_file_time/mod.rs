mod implementation;

#[derive(Debug)]
pub struct LastModifiedFileTime {
    day: u8,
    month: u8,
    year: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
}
