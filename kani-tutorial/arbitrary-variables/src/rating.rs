#[derive(Copy, Clone)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub enum Rating {
    One,
    Two,
    Three,
}

impl Rating {
    #[allow(dead_code)]
    fn as_int(&self) -> u8 {
        match self {
            Rating::One => 1,
            Rating::Two => 2,
            Rating::Three => 3,
        }
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    pub fn check_rating() {
        let rating: Rating = kani::any();
        assert!((1..=3).contains(&rating.as_int()));
    }
}

#[cfg(kani)]
mod expanded {
    use super::*;

    pub fn any_rating() -> Rating {
        match kani::any() {
            0 => Rating::One,
            1 => Rating::Two,
            _ => Rating::Three,
        }
    }
}
