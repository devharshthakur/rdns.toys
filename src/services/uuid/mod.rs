use anyhow::{Ok, Result, anyhow};
use async_trait::async_trait;
use std::str::FromStr;
use uuid::Uuid;

use crate::handlers::Service;

/// UUID service that generates random UUIDs based on DNS queries.
///
/// The service supports generating multiple UUIDs by specifying a number
/// in the query (e.g., "5.uuid" generates 5 UUIDs). If no number is specified,
/// it defaults to generating 1 UUID.
pub struct UUidService {
    max_results: usize,
}

impl UUidService {
    /// Creates a new UUID service with the specified maximum number of results.
    ///
    /// # Arguments
    /// * `max_results` - Maximum number of UUIDs that can be generated in a single query
    ///
    /// # Returns
    /// A new `UuidService` instance
    pub fn new(max_results: usize) -> Self {
        let max_results = if max_results < 1 { 1 } else { max_results };
        Self { max_results }
    }

    /// Generates the specified number of UUIDs and formats them as strings.
    ///
    /// # Arguments
    /// * `query` - The cleaned query string (just the number part, or empty for default)
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - Vector of UUID strings
    /// * `Err(anyhow::Error)` - If the query format is invalid or exceeds max_results
    fn generate_uuids(&self, query: &str) -> Result<Vec<String>> {
        // Parse the number from the query
        let num = if query.is_empty() {
            1
        } else {
            usize::from_str(query).map_err(|_| anyhow!("Invalid number format: '{}'", query))?
        };

        if num < 1 || num > self.max_results {
            return Err(anyhow!(
                "Number of UUIDs must be between 1 and {}. Got {}",
                self.max_results,
                num
            ));
        }

        let mut result = Vec::with_capacity(num);
        for _ in 0..num {
            let uuid = Uuid::new_v4();
            result.push(uuid.to_string());
        }
        Ok(result)
    }
}

#[async_trait]
impl Service for UUidService {
    /// Handle a cleaned query string and return response strings for DNS records.
    async fn query(&self, query: &str) -> Result<Vec<String>> {
        self.generate_uuids(query)
    }

    /// Export raw service data for debugging or monitoring.
    async fn dump(&self) -> Result<Vec<u8>> {
        let info = format!("UUID Service - Max results: {}", self.max_results);
        Ok(info.into_bytes())
    }
}
