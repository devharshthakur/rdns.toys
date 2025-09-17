//! # Geolocation Service
//!
//! This module parses and indexes a geonames.org data file to provide
//! fast lookups for geographic locations by name. It's a direct port
//! of the Go version's `geo` package.

use anyhow::{Context, Result};
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone_name: String,
    pub population: u64,
    pub timezone: Tz,
    pub country: String,
}

#[derive(Debug)]
pub struct Geo {
    tz_map: HashMap<String, Vec<Location>>,
    count: usize,
}

static RE_CLEAN: Lazy<Regex> = Lazy::new(|| Regex::new("[^a-z/]+").unwrap()); // Regex for cleaning queries: from the go project

impl Geo {
    pub fn new(file_path: &str) -> Result<Self> {
        let locations = Self::read_file(file_path)
            .with_context(|| format!("Failed to read the geonames.org file at '{}'", file_path))?;

        let mut geo = Self {
            tz_map: HashMap::new(),
            count: 0,
        };

        geo.load(locations);
        Ok(geo)
    }

    /// Reads and parses a geonames.org cities data file into a collection of Location structs.
    ///
    /// The function expects a tab-delimited file with 19 columns containing city information
    /// including geographic coordinates, population, and timezone data. It filters out invalid
    /// records and cleans up city names by removing parenthetical information.
    ///
    /// # Arguments
    /// * `file_path` - Path to the geonames cities data file (e.g., cities15000.txt)
    ///
    /// # Returns
    /// * `Result<Vec<Location>>` - A vector of parsed Location structs or an error
    pub fn read_file(file_path: &str) -> Result<Vec<Location>> {
        let file = File::open(file_path)?;
        // Configure CSV reader (rdr) for tab-delimited format without headers
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(file);

        let mut locations = Vec::new();

        for result in rdr.records() {
            let record = result?;

            // Skip records that don't have the expected 19 columns
            if record.len() != 19 {
                continue;
            }

            let location = (|| {
                // Parse city name: remove parentheses and extra info (e.g., "New York (NY)" -> "New York")
                let name = record.get(2)?.trim().split('(').next()?.trim().to_string();

                // Parse geographic coordinates
                let latitude = record.get(4)?.parse::<f64>().ok()?;
                let longitude = record.get(5)?.parse::<f64>().ok()?;

                // Parse country code and population
                let country = record.get(8)?.to_string();
                let population = record.get(14)?.parse::<u64>().ok()?;

                // Parse timezone information
                let timezone_name = record.get(17)?.to_string();
                let timezone = Tz::from_str(&timezone_name).ok()?;

                Some(Location {
                    id: record.get(0)?.to_string(),
                    name,
                    latitude,
                    longitude,
                    country,
                    population,
                    timezone_name,
                    timezone,
                })
            })();

            // Only add valid locations to the collection
            if let Some(loc) = location {
                locations.push(loc);
            }
        }
        Ok(locations)
    }

    /// Builds a searchable index `tz_map` from location data for fast geographic lookups.
    ///
    /// Creates two types of search keys for each location:
    /// 1. Cleaned city names (e.g., "New York" -> "newyork")
    /// 2. Timezone aliases (e.g., "America/New_York" -> "new_york")
    ///
    /// Locations are sorted by population within each key so larger cities appear first
    /// when multiple cities share the same name.
    ///
    /// # Arguments
    /// * `locations` - Vector of locations from `read_file()` to index
    pub fn load(&mut self, locations: Vec<Location>) {
        for loc in &locations {
            let name = RE_CLEAN
                .replace_all(&loc.name.to_lowercase(), "")
                .to_string();
            self.tz_map.entry(name).or_default().push(loc.clone());
            self.count += 1;
        }

        for loc in &locations {
            if let Some(city_alias) = loc.timezone_name.split('/').nth(1) {
                let cleaned_alias = RE_CLEAN
                    .replace_all(&city_alias.to_string(), "")
                    .to_string();
                self.tz_map
                    .entry(cleaned_alias)
                    .or_insert_with(|| vec![loc.clone()]);
            }
        }

        for locs in self.tz_map.values_mut() {
            locs.sort_unstable_by_key(|loc| Reverse(loc.population));
        }
    }

    /// Queries the geo database for locations matching the given search term.
    ///
    /// Supports optional country filtering using the format "city/country_code" where
    /// the country code is a 2-letter ISO code (e.g., "london/gb", "paris/fr").
    ///
    /// # Arguments
    /// * `q` - Search query, optionally with "/country_code" suffix
    ///
    /// # Returns
    /// * `Some(Vec<Location>)` - Matching locations sorted by population (descending)
    /// * `None` - No locations found for the query
    ///
    /// # Examples
    /// ```ignore
    /// // Search for all cities named "london"
    /// let results = geo.query("london");
    ///
    /// // Search specifically for London in Great Britain
    /// let results = geo.query("london/gb");
    /// ```
    pub fn query(&self, q: &str) -> Option<Vec<Location>> {
        // Parse query - check for city/country format
        let (query_str, country_filter) = if let Some((city, country)) = q.split_once('/') {
            // Only treat as country filter if it's exactly 2 characters
            if country.len() == 2 {
                (city, Some(country.to_uppercase()))
            } else {
                (q, None)
            }
        } else {
            (q, None)
        };

        // Clean the query string (remove non-alphabetic characters, convert to lowercase)
        let cleaned_query = RE_CLEAN
            .replace_all(&query_str.to_lowercase(), "")
            .to_string();

        // Look up locations by the cleaned query
        let locations = self.tz_map.get(&cleaned_query)?;

        // Filter by country if specified (Country filter)
        if let Some(country) = country_filter {
            let filtered: Vec<Location> = locations
                .iter()
                .filter(|loc| loc.country == country)
                .cloned()
                .collect();

            if filtered.is_empty() {
                None
            } else {
                Some(filtered)
            }
        } else {
            // Return all matching locations (already sorted by population)
            Some(locations.clone())
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
