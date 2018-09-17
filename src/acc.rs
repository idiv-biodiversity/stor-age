use std::ops::AddAssign;

pub struct Acc {
    pub total: u64,
    pub access: u64,
    pub modify: u64,
}

impl Acc {
    pub fn new(total: u64, access: u64, modify: u64) -> Acc {
        Acc { total, access, modify }
    }

    pub fn empty() -> Acc {
        Acc::new(0, 0, 0)
    }
}

impl AddAssign for Acc {
    fn add_assign(&mut self, other: Acc) {
        *self = Acc {
            total: self.total + other.total,
            access: self.access + other.access,
            modify: self.modify + other.modify,
        }
    }
}
