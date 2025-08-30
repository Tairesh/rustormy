use crate::errors::RustormyError;
use crate::models::{Language, Location};
#[cfg(not(test))]
use directories::ProjectDirs;
use std::fs::File;
use std::path::PathBuf;

#[cfg(not(test))]
fn get_geocoding_cache_dir() -> Result<PathBuf, RustormyError> {
    let proj_dirs = ProjectDirs::from("", "", "rustormy")
        .ok_or_else(|| RustormyError::CacheFindError("Could not determine cache directory"))?;

    Ok(proj_dirs.cache_dir().to_path_buf())
}

#[cfg(test)]
fn get_geocoding_cache_dir() -> Result<PathBuf, RustormyError> {
    Ok(std::env::temp_dir().join("rustormy_test_cache"))
}

#[cfg(not(test))]
/// Get the path to the geocoding cache file based on city and language
fn get_geocoding_cache_path(city: &str, language: Language) -> Result<PathBuf, RustormyError> {
    let cache_dir = get_geocoding_cache_dir()?;
    Ok(cache_dir.join(format!(
        "geocoding_{}_{}.json",
        city.replace(' ', "_"),
        language.code()
    )))
}

#[cfg(test)]
/// Get the path to the geocoding cache file based on city and language (for tests, use temp dir)
fn get_geocoding_cache_path(city: &str, language: Language) -> Result<PathBuf, RustormyError> {
    let cache_dir = get_geocoding_cache_dir()?;
    Ok(cache_dir.join(format!(
        "geocoding_{}_{}.json",
        city.replace(' ', "_"),
        language.code()
    )))
}

/// Retrieve a cached location if it exists
/// Returns Ok(Some(Location)) if found, Ok(None) if not found, or Err on error
pub fn get_cached_location(
    city: &str,
    language: Language,
) -> Result<Option<Location>, RustormyError> {
    let cache_path = get_geocoding_cache_path(city, language)?;

    if cache_path.exists() {
        let location: Location = serde_json::from_reader(File::open(cache_path)?)?;
        Ok(Some(location))
    } else {
        Ok(None)
    }
}

/// Cache a location to a file
pub fn cache_location(
    city: &str,
    language: Language,
    location: &Location,
) -> Result<(), RustormyError> {
    let cache_path = get_geocoding_cache_path(city, language)?;

    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(cache_path)?;
    serde_json::to_writer(file, location)?;
    Ok(())
}

pub fn clear_cache() -> Result<(), RustormyError> {
    let cache_dir = get_geocoding_cache_dir()?;

    if cache_dir.exists() {
        std::fs::remove_dir_all(cache_dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_location_and_retrieve() {
        let city = "Test City";
        let location = Location {
            name: city.to_string(),
            latitude: 12.34,
            longitude: 56.78,
        };

        // Cache the location
        cache_location(city, Language::English, &location).expect("Failed to cache location");
        // Retrieve the cached location
        let cached_location =
            get_cached_location(city, Language::English).expect("Failed to get cached location");
        assert!(cached_location.is_some());
        let cached_location = cached_location.unwrap();
        assert_eq!(cached_location.name, location.name);
        assert_eq!(cached_location.latitude, location.latitude);
        assert_eq!(cached_location.longitude, location.longitude);

        // Check for a non-cached city
        let non_cached = get_cached_location("Nonexistent City", Language::English)
            .expect("Failed to get cached location");
        assert!(non_cached.is_none());

        // Check for a different language cache miss
        let lang_miss =
            get_cached_location(city, Language::Spanish).expect("Failed to get cached location");
        assert!(lang_miss.is_none());

        // Clean up the test cache file
        clear_cache().expect("Failed to clear cached location");
    }
}
