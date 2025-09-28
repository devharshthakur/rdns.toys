use crate::handlers::Service;
use anyhow::Result;
use async_trait::async_trait;
use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;

/// PiService provides access to the mathematical constant Pi (π) through DNS queries.
///
/// This service returns Pi in different formats depending on the query:
/// - As a text string with high precision
/// - As an IPv4 address (3.141.59.27)
/// - As an IPv6 address (3141:5926:5358:9793:2384:6264:3383:2795)
///
/// The service is designed to be educational and demonstrate DNS-based data retrieval
/// for mathematical constants.
pub struct PiService;

// Constants
const PI_TTL: u32 = 31536000;

impl PiService {
    /// Creates a new PiService instance.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Service for PiService {
    /// Handles Pi constant queries, returning Pi in different formats.
    ///
    /// This method provides access to the mathematical constant Pi (π) through
    /// DNS queries. It supports multiple record types, returning Pi as text,
    /// IPv4 address, or IPv6 address depending on the query type.
    ///
    /// This is a "mathematical constant" service - it provides easy access to Pi
    /// through DNS, which can be useful for educational purposes, mathematical
    /// calculations, or as a demonstration of DNS-based data retrieval.
    ///
    /// ## Arguments
    /// * `request` - The DNS request
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (TXT, A, AAAA)
    /// * `_cleaned_query` - The cleaned query string (unused for Pi service)
    ///
    /// ## Returns
    /// * `Some(Vec<Record>)` - Vector containing Pi in the requested format
    /// * `None` - If the query type is not supported
    async fn query(
        &self,
        _request: &Request,
        query_name: &Name,
        query_type: RecordType,
        _cleaned_query: &str,
    ) -> Option<Vec<Record>> {
        match query_type {
            // Return Pi as text
            RecordType::TXT => Some(vec![Record::from_rdata(
                query_name.clone(),
                PI_TTL,
                RData::TXT(rdata::TXT::new(vec![
                    "3.141592653589793238462643383279502884197169".to_string(),
                ])),
            )]),

            RecordType::A => {
                // Return Pi as IPv4: 3.141.59.27
                let pi_ip = std::net::Ipv4Addr::new(3, 141, 59, 27);
                Some(vec![Record::from_rdata(
                    query_name.clone(),
                    PI_TTL,
                    RData::A(pi_ip.into()),
                )])
            }

            RecordType::AAAA => {
                // Return Pi as IPv6: 3141:5926:5358:9793:2384:6264:3383:2795
                if let Ok(pi_ipv6) = "3141:5926:5358:9793:2384:6264:3383:2795".parse() {
                    Some(vec![Record::from_rdata(
                        query_name.clone(),
                        PI_TTL,
                        RData::AAAA(pi_ipv6),
                    )])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Exports raw service data for debugging or monitoring.
    ///
    /// This method provides access to the raw Pi constant data for debugging
    /// or monitoring purposes. It returns the Pi constant as a byte array.
    ///
    /// ## Returns
    /// * `Ok(Vec<u8>)` - Pi constant as bytes
    /// * `Err(anyhow::Error)` - If data export fails
    async fn dump(&self) -> Result<Vec<u8>> {
        let pi_text = "3.141592653589793238462643383279502884197169";
        Ok(pi_text.as_bytes().to_vec())
    }
}
