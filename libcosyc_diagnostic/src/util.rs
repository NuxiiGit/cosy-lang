/// Returns the number of digits of this natural number.
pub fn digit_count(n : usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().ceil() as usize
    }
}

