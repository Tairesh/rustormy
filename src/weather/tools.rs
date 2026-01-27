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

/// Calculate apparent temperature (feels like) using the formula
/// AT = T + 0.33e - 0.70v - 4.00
/// where e is the vapor pressure in hPa and v is the wind speed in m/s
/// T is the air temperature in °C
/// w is the wind speed in m/s
/// e = (h / 100) * 6.105 * exp(17.27 * T / (237.7 + T))
/// h is the relative humidity in %
/// This formula is valid for temperatures between 10°C and 40°C
/// and wind speeds up to 10 m/s
pub fn apparent_temperature(t: f64, w: f64, h: f64) -> f64 {
    let e = (h / 100.0) * 6.105 * (17.27 * t / (237.7 + t)).exp();
    let at = t + 0.33 * e - 0.70 * w - 4.00;
    (at * 10.0).round() / 10.0 // Round to one decimal place
}
