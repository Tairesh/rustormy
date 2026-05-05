use crate::models::Location;
use chrono::{DateTime, Datelike, Timelike, Utc};

pub fn is_daytime(location: &Location, now: DateTime<Utc>) -> bool {
    solar_altitude_deg(location.latitude, location.longitude, now) > 0.0
}

/// Calculates the solar altitude (the angle of the sun above the horizon) in degrees for a given location and time.
fn solar_altitude_deg(lat_deg: f64, lon_deg: f64, now: DateTime<Utc>) -> f64 {
    let lat = lat_deg.to_radians();

    let day_of_year = f64::from(now.ordinal());
    let hours =
        f64::from(now.hour()) + f64::from(now.minute()) / 60.0 + f64::from(now.second()) / 3600.0;
    let year = now.year();
    let days_in_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        366.0
    } else {
        365.0
    };

    let gamma = std::f64::consts::TAU / days_in_year * (day_of_year - 1.0 + (hours - 12.0) / 24.0);

    let eqtime = 229.18
        * (0.000_075 + 0.001_868 * gamma.cos()
            - 0.032_077 * gamma.sin()
            - 0.014_615 * (2.0 * gamma).cos()
            - 0.040_849 * (2.0 * gamma).sin());

    let decl = 0.006_918 - 0.399_912 * gamma.cos() + 0.070_257 * gamma.sin()
        - 0.006_758 * (2.0 * gamma).cos()
        + 0.000_907 * (2.0 * gamma).sin()
        - 0.002_697 * (3.0 * gamma).cos()
        + 0.001_48 * (3.0 * gamma).sin();

    let time_offset = eqtime + 4.0 * lon_deg;
    let tst_minutes = hours * 60.0 + time_offset;

    let hour_angle_deg = tst_minutes / 4.0 - 180.0;
    let h = hour_angle_deg.to_radians();

    let sin_alt = lat.sin() * decl.sin() + lat.cos() * decl.cos() * h.cos();
    sin_alt.asin().to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(y: i32, m: u32, d: u32, h: u32, mi: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, mi, 0).unwrap()
    }

    fn loc(latitude: f64, longitude: f64) -> Location {
        Location {
            name: String::new(),
            latitude,
            longitude,
        }
    }

    #[test]
    fn tromso_polar_day_is_day_at_all_hours() {
        let lat = 69.65;
        let lon = 18.96;
        for hour in [0, 3, 6, 9, 12, 15, 18, 21] {
            assert!(
                is_daytime(&loc(lat, lon), dt(2024, 6, 15, hour, 0)),
                "Tromsø should be in polar day on 2024-06-15 at {hour}:00 UTC"
            );
        }
    }

    #[test]
    fn longyearbyen_polar_day() {
        assert!(is_daytime(&loc(78.22, 15.65), dt(2024, 7, 1, 0, 0)));
    }

    #[test]
    fn tromso_polar_night_is_night_at_all_hours() {
        let lat = 69.65;
        let lon = 18.96;
        for hour in [0, 3, 6, 9, 12, 15, 18, 21] {
            assert!(
                !is_daytime(&loc(lat, lon), dt(2024, 12, 15, hour, 0)),
                "Tromsø should be in polar night on 2024-12-15 at {hour}:00 UTC"
            );
        }
    }

    #[test]
    fn longyearbyen_polar_night() {
        assert!(!is_daytime(&loc(78.22, 15.65), dt(2024, 12, 15, 12, 0)));
    }

    #[test]
    fn mcmurdo_polar_day_in_december() {
        assert!(is_daytime(&loc(-77.85, 166.67), dt(2024, 12, 15, 12, 0)));
    }

    #[test]
    fn mcmurdo_polar_night_in_june() {
        assert!(!is_daytime(&loc(-77.85, 166.67), dt(2024, 6, 15, 12, 0)));
    }

    #[test]
    fn quito_day_at_local_morning() {
        assert!(is_daytime(&loc(-0.18, -78.47), dt(2024, 3, 20, 12, 0)));
    }

    #[test]
    fn quito_night_at_local_after_midnight() {
        assert!(!is_daytime(&loc(-0.18, -78.47), dt(2024, 3, 20, 6, 0)));
    }

    #[test]
    fn fiji_and_samoa_same_utc_both_daytime() {
        let utc = dt(2024, 6, 21, 0, 0);
        assert!(is_daytime(&loc(-18.14, 178.44), utc), "Suva should be day");
        assert!(is_daytime(&loc(-13.83, -171.77), utc), "Apia should be day");
    }

    #[test]
    fn greenwich_noon_equinox_is_day() {
        assert!(is_daytime(&loc(51.48, 0.0), dt(2024, 3, 20, 12, 0)));
    }

    #[test]
    fn greenwich_midnight_equinox_is_night() {
        assert!(!is_daytime(&loc(51.48, 0.0), dt(2024, 3, 20, 0, 0)));
    }

    #[test]
    fn sydney_summer_local_afternoon_is_day() {
        assert!(is_daytime(&loc(-33.87, 151.21), dt(2024, 12, 21, 2, 0)));
    }

    #[test]
    fn reykjavik_winter_noon_is_just_above_horizon() {
        let alt = solar_altitude_deg(64.13, -21.94, dt(2024, 12, 21, 12, 0));
        assert!(alt > 0.0, "Reykjavík noon altitude was {alt}");
        assert!(alt < 5.0, "Reykjavík noon altitude was {alt}, expected < 5");
    }

    #[test]
    fn greenwich_2025_noon_equinox_is_day() {
        // 2025 is a non-leap year — exercises the `else` branch of
        // `days_in_year`. March 20 noon UTC at Greenwich is well past sunrise.
        assert!(is_daytime(&loc(51.48, 0.0), dt(2025, 3, 20, 12, 0)));
    }
}
