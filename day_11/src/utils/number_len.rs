pub trait NumberLen {
    fn len(&self) -> usize;
}

impl NumberLen for u64 {
    fn len(&self) -> usize {
        if *self == 0 {
            1
        } else {
            (*self as f64).log10().floor() as usize + 1
        }
    }
}