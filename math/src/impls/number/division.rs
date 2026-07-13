use crate::number::Divisibility;

impl Divisibility for i8 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for i16 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for i32 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for i64 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for i128 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for isize {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for u8 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for u16 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for u32 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for u64 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for u128 {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}

impl Divisibility for usize {
    fn divisible_by(&self, other: &Self) -> bool {
        if *other == 0 { false }
        else { self % other == 0 }
    }
}
