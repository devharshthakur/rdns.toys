use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing;

use hickory_proto::rr::{LowerName, Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;

// Constants
const IP_TTL: u32 = 60;
const PI_TTL: u32 = 31536000;
// --- Regex for cleaning Queries ---//
static RE_CLEAN: Lazy<Regex> =
    Lazy::new(|| Regex::new("[^a-zA-Z0-9/\\-\\.:,]").expect("Invalid regex pattern"));

#[async_trait]
pub trait Service: Send + Sync {
    async fn queuery(&self) -> Result<Vec<String>>;
    async fn dump(&self) -> Result<Vec<u8>>;
}

/// `DnsHandlers` is the central struct responsible for managing DNS service handlers,
/// the authoritative domain, and pre-generated help records for a DNS server.
///
/// # Fields
/// - `services`: A mapping from DNS query suffixes (e.g., "ip", "pi") to their corresponding
///   boxed [`Service`] trait objects. This allows dynamic dispatch of service logic based on
///   the query type or suffix.
/// - `domain`: The authoritative [`LowerName`] domain for which this handler is responsible.
///   All queries are expected to be subdomains of this domain.
/// - `help_records`: A vector of [`Record`] objects containing TXT records that describe
///   available DNS services and usage instructions. These are typically returned in response
///   to "help" queries.
///
/// # Example
/// ```
/// use hickory_proto::rr::Name;
/// use hickory_proto::rr::LowerName;
/// use crate::handlers::DnsHandlers;
///
/// let domain = LowerName::from(Name::from_ascii("example.com.").unwrap());
/// let mut handlers = DnsHandlers::new(domain).unwrap();
/// ```
pub struct DnsHandlers {
    /// Mapping from DNS query suffix (e.g., "ip", "pi") to the corresponding service handler.
    pub services: HashMap<String, Box<dyn Service>>,
    /// The authoritative domain for which this handler is responsible.
    pub domain: LowerName,
    /// Pre-generated TXT records describing available DNS services and usage.
    pub help_records: Vec<Record>,
}

impl DnsHandlers {
    pub fn new(domain: LowerName) -> Result<Self> {
        let help_records = Self::create_help_records(&domain.to_string())?;
        Ok(DnsHandlers {
            services: HashMap::new(),
            domain,
            help_records,
        })
    }

    /// Registers a new DNS service with the given suffix.
    ///
    /// ## Arguments
    ///
    /// * `suffix` - The DNS query suffix (e.g., "ip", "pi", etc.) to associate with the service.
    /// * `service` - A boxed implementation of the [`Service`] trait to handle queries for this suffix.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut handlers = DnsHandlers::new(domain)?;
    /// handlers.register_service("ip".to_string(), Box::new(IpService::new()));
    /// ```
    pub fn register(&mut self, suffix: String, service: Box<dyn Service>) {
        self.services.insert(suffix.clone(), service);
        tracing::info!("Registered service for suffix: {}", suffix);
    }

    /// Creates a set of help TXT records describing available DNS services for the given domain.
    ///
    /// ## Arguments
    ///
    /// * `domain` - The domain name for which to generate help records.
    ///
    /// ## Returns
    ///
    /// Returns a vector of [`Record`] objects containing help information as TXT records.
    ///
    /// ## Errors
    ///
    /// Returns an error if the help record name cannot be parsed.
    fn create_help_records(domain: &str) -> Result<Vec<Record>> {
        let help_texts = vec![
            "Welcome! Available DNS services:".to_string(),
            format!("dig TXT ip.{}", domain),
            format!("dig A pi.{}", domain),
            format!("dig TXT <location>.time.{}", domain),
            format!("dig TXT help.{}", domain),
        ];
        let mut records = Vec::new();
        for text in help_texts {
            let record = Record::from_rdata(
                Name::from_str("help.")?.clone(),
                IP_TTL,
                RData::TXT(rdata::TXT::new(vec![text])),
            );
            records.push(record);
        }
        Ok(records)
    }

    /// Handles DNS queries for the "ip" service, returning the client's IP address.
    ///
    /// This function responds to DNS queries for the "ip" suffix, supporting both TXT and A record types.
    /// - For TXT queries, it returns the client's IP address as a string in a TXT record.
    /// - For A queries, it returns the client's IPv4 address as an A record, or `None` if the client uses IPv6.
    ///
    /// ## Arguments
    ///
    /// * `request` - The DNS request, used to extract the client's source IP address.
    /// * `query_name` - The DNS name being queried.
    /// * `query_type` - The type of DNS record requested (TXT or A).
    ///
    /// ## Returns
    ///
    /// Returns `Some(Record)` containing the appropriate DNS response if the query type is supported,
    /// or `None` if the query type is not supported or if an IPv6 address is requested as an A record.
    /// ```
    async fn handle_ip_query(
        &self,
        request: &Request,
        query_name: &Name,
        query_type: RecordType,
    ) -> Option<Record> {
        if query_type != RecordType::TXT && query_type != RecordType::A {
            return None;
        }

        let client_ip = request.src().ip();

        match query_type {
            RecordType::TXT => Some(Record::from_rdata(
                query_name.clone(),
                IP_TTL,
                RData::TXT(rdata::TXT::new(vec![client_ip.to_string()])),
            )),
            RecordType::A => {
                // Reuturn as an A record if its ipv4
                if let IpAddr::V4(ipv4) = client_ip {
                    Some(Record::from_rdata(
                        query_name.clone(),
                        IP_TTL,
                        RData::A(ipv4.into()),
                    ))
                } else {
                    None // Can't return IPv6 as A record
                }
            }
            _ => None,
        }
    }

    /// Handle Pi constant queries - returns Pi in different formats
    async fn handle_pi_query(&self, query_name: &Name, query_type: RecordType) -> Option<Record> {
        match query_type {
            // Return Pi as text
            RecordType::TXT => Some(Record::from_rdata(
                query_name.clone(),
                PI_TTL,
                RData::TXT(rdata::TXT::new(vec![
                    "3.141592653589793238462643383279502884197169".to_string(),
                ])),
            )),

            RecordType::A => {
                // Return Pi as IPv4: 3.141.59.27
                let pi_ip = std::net::Ipv4Addr::new(3, 141, 59, 27);
                Some(Record::from_rdata(
                    query_name.clone(),
                    PI_TTL,
                    RData::A(pi_ip.into()),
                ))
            }

            RecordType::AAAA => {
                // Return Pi as IPv6: 3141:5926:5358:9793:2384:6264:3383:2795
                if let Ok(pi_ipv6) = "3141:5926:5358:9793:2384:6264:3383:2795".parse() {
                    Some(Record::from_rdata(
                        query_name.clone(),
                        PI_TTL,
                        RData::AAAA(pi_ipv6),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
