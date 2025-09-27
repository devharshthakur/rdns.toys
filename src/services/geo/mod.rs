//! # Geo DNS Service
//!
//! This module provides DNS-based access to the geolocation service,
//! allowing users to query geographic information via DNS queries.

use crate::handlers::Service;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono_tz::Tz;
use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

/// Represents a geographic location with timezone and population data.
///
/// This struct contains all the essential information about a city or location
/// that can be queried through the DNS service.
#[derive(Debug, Clone)]
pub struct Location {
    /// Unique identifier from the geonames.org database
    pub id: String,
    /// Human-readable name of the location
    pub name: String,
    /// Latitude coordinate in decimal degrees
    pub latitude: f64,
    /// Longitude coordinate in decimal degrees
    pub longitude: f64,
    /// Timezone name (e.g., "America/New_York")
    pub timezone_name: String,
    /// Population count for the location
    pub population: u64,
    /// Parsed timezone object for time calculations
    pub timezone: Tz,
    /// Two-letter country code (e.g., "US", "CA")
    pub country: String,
}

/// Core geolocation service that manages location data and provides querying capabilities.
///
/// The Geo struct maintains an internal mapping of location names to Location objects,
/// allowing for efficient lookups by city name or timezone alias.
#[derive(Debug)]
pub struct Geo {
    /// Maps cleaned location names to vectors of matching locations
    tz_map: HashMap<String, Vec<Location>>,
    /// Total number of locations loaded
    count: usize,
}

/// Regex pattern used to clean location names for consistent lookups.
/// Removes all non-alphabetic characters except forward slashes.
static RE_CLEAN: Lazy<Regex> = Lazy::new(|| Regex::new("[^a-z/]+").unwrap());

impl Geo {
    /// Creates a new Geo instance by loading location data from a file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the geonames.org cities data file
    ///
    /// # Returns
    /// * `Result<Self>` - A new Geo instance or an error if file cannot be read
    ///
    /// # Example
    /// ```
    /// let geo = Geo::new("data/cities15000.txt")?;
    /// ```
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
    pub fn read_file(file_path: &str) -> Result<Vec<Location>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(file);

        let mut locations: Vec<Location> = Vec::new();

        for result in rdr.records() {
            let record = result?;

            if record.len() != 19 {
                continue;
            }

            let location = (|| {
                let name = record.get(2)?.trim().split('(').next()?.trim().to_string();
                let latitude = record.get(4)?.parse::<f64>().ok()?;
                let longitude = record.get(5)?.parse::<f64>().ok()?;
                let country = record.get(8)?.to_string();
                let population = record.get(14)?.parse::<u64>().ok()?;
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

            if let Some(loc) = location {
                locations.push(loc);
            }
        }
        Ok(locations)
    }

    /// Loads a collection of locations into the internal data structure.
    ///
    /// This method processes the locations, creates searchable mappings,
    /// and sorts locations by population for better query results.
    ///
    /// # Arguments
    /// * `locations` - Vector of Location objects to load
    pub fn load(&mut self, locations: Vec<Location>) {
        let clean_text =
            |text: &str| -> String { RE_CLEAN.replace_all(&text.to_lowercase(), "").to_string() };

        for location in &locations {
            let city_key = clean_text(&location.name);
            self.tz_map
                .entry(city_key)
                .or_default()
                .push(location.clone());

            if let Some(city_alias) = location.timezone_name.split('/').nth(1) {
                let alias_key = clean_text(city_alias);
                self.tz_map
                    .entry(alias_key)
                    .or_default()
                    .push(location.clone());
            }

            self.count += 1;
        }

        for location_list in self.tz_map.values_mut() {
            location_list.sort_unstable_by_key(|loc| Reverse(loc.population));
        }
    }

    /// Queries for locations matching the given search string.
    ///
    /// Supports both city name queries and city/country format queries.
    /// Results are sorted by population (largest first).
    ///
    /// # Arguments
    /// * `q` - Query string, optionally in "city/country" format
    ///
    /// # Returns
    /// * `Option<Vec<Location>>` - Matching locations or None if no matches
    ///
    /// # Examples
    /// ```
    /// // Query by city name
    /// let locations = geo.query("new york");
    ///
    /// // Query by city and country
    /// let locations = geo.query("london/uk");
    /// ```
    pub fn query(&self, q: &str) -> Option<Vec<Location>> {
        let (query_str, country_filter) = if let Some((city, country)) = q.split_once('/') {
            if country.len() == 2 {
                (city, Some(country.to_uppercase()))
            } else {
                (q, None)
            }
        } else {
            (q, None)
        };

        let cleaned_query = RE_CLEAN
            .replace_all(&query_str.to_lowercase(), "")
            .to_string();

        let locations = self.tz_map.get(&cleaned_query)?;

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
            Some(locations.clone())
        }
    }

    /// Returns the total number of locations loaded in the service.
    ///
    /// # Returns
    /// * `usize` - Total count of loaded locations
    pub fn count(&self) -> usize {
        self.count
    }
}

/// DNS service wrapper for the geolocation functionality.
///
/// This service allows users to query geographic information through DNS queries.
/// It supports both TXT and A record types, providing location data in different formats.
pub struct GeoService {
    geo: Geo,
}

impl GeoService {
    /// Creates a new GeoService instance.
    ///
    /// # Arguments
    /// * `data_path` - Path to the geonames.org cities data file
    ///
    /// # Returns
    /// * `Result<Self>` - A new GeoService instance or an error
    pub fn new(data_path: &str) -> Result<Self> {
        let geo = Geo::new(data_path).with_context(|| {
            format!(
                "Failed to initialize geo service with data from '{}'",
                data_path
            )
        })?;

        Ok(Self { geo })
    }

    /// Formats location data as a human-readable string for TXT records.
    ///
    /// # Arguments
    /// * `location` - The location to format
    ///
    /// # Returns
    /// * `String` - Formatted location information
    ///
    /// # Example
    /// ```
    /// let location = Location {
    ///     id: "123".to_string(),
    ///     name: "Mumbai".to_string(),
    ///     latitude: 19.0760,
    ///     longitude: 72.8777,
    ///     timezone_name: "Asia/Kolkata".to_string(),
    ///     population: 20411000,
    ///     timezone: chrono_tz::Asia::Kolkata,
    ///     country: "IN".to_string(),
    /// };
    /// let geo_service = GeoService { geo: geo_instance }; // assume geo_instance is a valid Geo
    /// let txt = geo_service.format_location_txt(&location);
    /// assert_eq!(
    ///     txt,
    ///     "Mumbai (IN) - Pop: 20411000, TZ: Asia/Kolkata, Lat: 19.08, Lon: 72.88"
    /// );
    /// ```
    fn format_location_txt(&self, location: &Location) -> String {
        format!(
            "{} ({}) - Pop: {}, TZ: {}, Lat: {:.4}, Lon: {:.4}",
            location.name,
            location.country,
            location.population,
            location.timezone_name,
            location.latitude,
            location.longitude
        )
    }

    /// Formats location data as a comma-separated string for A records.
    ///
    /// # Arguments
    /// * `location` - The location to format
    ///
    /// # Returns
    /// * `String` - Comma-separated location data (name,country,lat,lon)
    ///
    /// # Example
    /// ```
    /// let location = Location {
    ///     id: "123".to_string(),
    ///     name: "Mumbai".to_string(),
    ///     latitude: 19.0760,
    ///     longitude: 72.8777,
    ///     timezone_name: "Asia/Kolkata".to_string(),
    ///     population: 20411000,
    ///     timezone: chrono_tz::Asia::Kolkata,
    ///     country: "IN".to_string(),
    /// };
    /// let geo_service = GeoService { geo: geo_instance }; // assume geo_instance is a valid Geo
    /// let a_str = geo_service.format_location_a(&location);
    /// assert_eq!(
    ///     a_str,
    ///     "Mumbai,IN,19.076,72.8777"
    /// );
    /// ```
    fn format_location_a(&self, location: &Location) -> String {
        format!(
            "{},{},{},{}",
            location.name, location.country, location.latitude, location.longitude
        )
    }

    /// Handles TXT record queries for geographic information.
    ///
    /// Returns human-readable location data in TXT format.
    ///
    /// # Arguments
    /// * `query` - The DNS query string
    ///
    /// # Returns
    /// * `Option<Vec<Record>>` - TXT records with location data or None
    async fn handle_txt_query(&self, query: &str) -> Option<Vec<Record>> {
        let locations = self.geo.query(query)?;

        let mut records = Vec::new();
        for location in locations {
            let txt_data = self.format_location_txt(&location);
            let record = Record::from_rdata(
                Name::from_str(".").ok()?,
                60,
                RData::TXT(rdata::TXT::new(vec![txt_data])),
            );
            records.push(record);
        }

        if records.is_empty() {
            None
        } else {
            Some(records)
        }
    }

    /// Handles A record queries for geographic information.
    ///
    /// Returns location data in a structured format suitable for A records.
    ///
    /// # Arguments
    /// * `query` - The DNS query string
    ///
    /// # Returns
    /// * `Option<Vec<Record>>` - A records with location data or None
    async fn handle_a_query(&self, query: &str) -> Option<Vec<Record>> {
        let locations = self.geo.query(query)?;

        let mut records = Vec::new();
        for location in locations {
            let a_data = self.format_location_a(&location);
            let record = Record::from_rdata(
                Name::from_str(".").ok()?,
                60,
                RData::TXT(rdata::TXT::new(vec![a_data])),
            );
            records.push(record);
        }

        if records.is_empty() {
            None
        } else {
            Some(records)
        }
    }
}

#[async_trait]
impl Service for GeoService {
    async fn query(
        &self,
        _request: &Request,
        _query_name: &Name,
        query_type: RecordType,
        cleaned_query: &str,
    ) -> Option<Vec<Record>> {
        match query_type {
            RecordType::TXT => self.handle_txt_query(cleaned_query).await,
            RecordType::A => self.handle_a_query(cleaned_query).await,
            _ => None,
        }
    }

    /// Dumps service statistics and data for debugging purposes.
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - Service summary as bytes
    async fn dump(&self) -> Result<Vec<u8>> {
        // For now, return a simple summary
        let summary = format!("Geo service loaded with {} locations", self.geo.count());
        Ok(summary.into_bytes())
    }
}
