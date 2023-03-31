pub fn remap(low1: f64, high1: f64, low2: f64, high2: f64, value: f64) -> f64 {
    low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}
