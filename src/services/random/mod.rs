use crate::handlers::Service;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;

/// RandomService provides random number generation through DNS queries.
///
/// This service generates random numbers within a specified range based on DNS queries.
/// The query format should be "min-max" (e.g., "1-100", "10-50").
///
/// Examples:
/// - `dig TXT 1-100.random.localhost`
/// - `dig TXT 10-50.random.localhost`
///
/// The service is designed to be educational and demonstrate DNS-based random number generation.
pub struct RandomService;

const RANDOM_TTL: u32 = 1;

/// Regex to match a numeric range in the format "min-max".
///
/// This regex captures two groups of digits separated by a hyphen, anchored to the start and end of the string.
/// For example, the query "1-100" will match and capture "1" as the minimum and "100" as the maximum.
/// Captured from `dns.toys` project.
static RANGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([0-9]+)-([0-9]+)$").expect("Invalid Regex pattern"));

impl RandomService {
    pub fn new() -> Self {
        Self
    }

    /// Generates a random integer within a specified range parsed from the query string.
    ///
    /// The query string must be in the format "min-max" (e.g., "1-100"), where `min` and `max`
    /// are positive integers. The function parses the minimum and maximum values, validates them,
    /// and returns a random integer within the inclusive range `[min, max]`.
    ///
    /// # Arguments
    /// * `query` - A string slice representing the range in the format "min-max".
    ///
    /// # Returns
    /// * `Ok(i32)` - A random integer within the specified range.
    /// * `Err(anyhow::Error)` - If the query format is invalid, parsing fails, or the range is invalid.
    fn generate_random_number(&self, query: &str) -> Result<i32> {
        let captures = RANGE_REGEX
            .captures(query)
            .ok_or_else(|| anyhow!("Invalid random query format"))?;

        let min_str = captures.get(1).unwrap().as_str();
        let max_str = captures.get(2).unwrap().as_str();

        let min: i32 = min_str
            .parse()
            .map_err(|_| anyhow!("Invalid minimum value {}", min_str))?;
        let max: i32 = max_str
            .parse()
            .map_err(|_| anyhow!("Invalid maximum value {}", max_str))?;

        if min > max {
            return Err(anyhow!("Minimum value must be less than maximum value"));
        } else if min < 0 || max < 0 {
            return Err(anyhow!("Minimum and maximum values must be positive"));
        }

        let random_value = rand::rng().random_range(min..=max);
        Ok(random_value)
    }
}

#[async_trait]
impl Service for RandomService {
    /// Handles random number generation queries.
    ///
    /// This method processes DNS queries for random number generation.
    /// It expects queries in the format "min-max" and returns a random number
    /// within that range as a TXT record.
    ///
    /// ## Arguments
    /// * `request` - The DNS request
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (`TXT`, `A`, `AAAA`)
    /// * `cleaned_query` - The cleaned query string (e.g., "1-100")
    ///
    /// ## Returns
    /// * `Some(Vec<Record>)` - Vector containing the random number
    /// * `None` - If the query type is not supported
    async fn query(
        &self,
        _request: &Request,
        query_name: &Name,
        query_type: RecordType,
        cleaned_query: &str,
    ) -> Option<Vec<Record>> {
        match query_type {
            RecordType::TXT => {
                let random_value = self.generate_random_number(cleaned_query).unwrap();
                Some(vec![Record::from_rdata(
                    query_name.clone(),
                    RANDOM_TTL,
                    RData::TXT(rdata::TXT::new(vec![random_value.to_string()])),
                )])
            }
            _ => None,
        }
    }

    /// Exports raw service data for debugging or monitoring.
    ///
    /// This method provides access to the raw random service data for debugging
    /// or monitoring purposes. It returns information about the service.
    ///
    /// ## Returns
    /// * `Ok(Vec<u8>)` - Service information as bytes
    /// * `Err(anyhow::Error)` - If data export fails
    async fn dump(&self) -> Result<Vec<u8>> {
        let service_info = "Random service - Returns a random number between a given range";
        Ok(service_info.as_bytes().to_vec())
    }
}
