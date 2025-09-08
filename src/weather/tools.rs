use crate::models::Units;

/// Convert Celsius to Fahrenheit
pub fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/// Convert Fahrenheit to Celsius
pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Calculate dew point using the Magnus formula
pub fn dew_point(mut t: f64, h: f64, units: Units) -> f64 {
    const B: f64 = 17.625;
    const C: f64 = 243.04;
    if units == Units::Imperial {
        t = f_to_c(t);
    }
    let gamma = (B * t) / (C + t) + (h / 100.0).ln();
    let mut result = (C * gamma) / (B - gamma);
    if units == Units::Imperial {
        result = c_to_f(result);
    }

    (result * 10.0).round() / 10.0 // Round to one decimal place
}
