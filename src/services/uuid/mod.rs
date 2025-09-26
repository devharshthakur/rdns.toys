use anyhow::{Ok, Result, anyhow};
use async_trait::async_trait;
use std::str::FromStr;
use uuid::Uuid;

use crate::handlers::Service;
use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;

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
    async fn generate_uuids(&self, query: &str) -> Result<Vec<String>> {
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
    /// Handle a DNS query and return DNS records directly.
    /// For UUID service, this only supports TXT records with the cleaned query parameter.
    async fn query(
        &self,
        _request: &Request,
        query_name: &Name,
        query_type: RecordType,
        cleaned_query: &str,
    ) -> Option<Vec<Record>> {
        // UUID service only supports TXT records
        if query_type != RecordType::TXT {
            return None;
        }

        // Generate UUIDs based on the cleaned query
        let uuids_result = self.generate_uuids(cleaned_query).await;
        if uuids_result.is_ok() {
            let uuids = uuids_result.unwrap();
            let mut records = Vec::new();
            for uuid in uuids {
                let record = Record::from_rdata(
                    query_name.clone(),
                    60, // TTL
                    RData::TXT(rdata::TXT::new(vec![uuid])),
                );
                records.push(record);
            }
            Some(records)
        } else {
            None
        }
    }

    /// Export raw service data for debugging or monitoring.
    async fn dump(&self) -> Result<Vec<u8>> {
        let info = format!("UUID Service - Max results: {}", self.max_results);
        Ok(info.into_bytes())
    }
}
