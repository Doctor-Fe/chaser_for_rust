use std::u8;

pub struct ActionResult {
    pub data: Vec<u8>,
    pub will_continue: bool,
}

impl ActionResult {
    pub fn new(dat: [u8; 12]) -> ActionResult {
        return ActionResult {
            data: dat[1..10].iter().map(|a| *a - 48).collect(),
            will_continue: dat[0] != 48,
        };
    }
}
