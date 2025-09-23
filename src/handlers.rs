use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing;

use hickory_proto::{
    op::OpCode,
    rr::{LowerName, Name, RData, Record, RecordType, rdata},
};
use hickory_server::server::Request;

// Constants
const IP_TTL: u32 = 60;
const PI_TTL: u32 = 31536000;
// --- Regex for cleaning Queries ---//
static RE_CLEAN: Lazy<Regex> =
    Lazy::new(|| Regex::new("[^a-zA-Z0-9/\\-\\.:,]").expect("Invalid regex pattern"));

#[async_trait]
/// Trait for DNS service plugins. Provides async methods for handling queries and exporting raw data.
pub trait Service: Send + Sync {
    /// Handle a cleaned query string and return response strings for DNS records.
    async fn query(&self, query: &str) -> Result<Vec<String>>;

    /// Export raw service data for debugging or monitoring.
    async fn dump(&self) -> Result<Vec<u8>>;
}

pub struct DnsHandlers {
    pub services: HashMap<String, Box<dyn Service>>, // Mapping from DNS query suffix (e.g., "ip", "pi") to the corresponding service handler.
    pub domain: LowerName, // The authoritative domain for which this handler is responsible.
    pub help_records: Vec<Record>, // Pre-generated TXT records describing available DNS services and usage.
}

impl DnsHandlers {
    /// Creates a new DnsHandlers instance with the specified authoritative domain.
    ///
    /// This constructor initializes the service registry and generates help records
    /// that describe available services to users. The help records provide usage
    /// examples and instructions for accessing different services.
    ///
    /// This is the "setup" function that prepares the DNS server for operation.
    /// It creates the foundation upon which services can be registered and queries
    /// can be processed.
    ///
    /// ## Arguments
    /// * `domain` - The authoritative domain this handler will manage
    ///
    /// ## Returns
    /// * `Ok(DnsHandlers)` - A fully initialized handler instance
    /// * `Err(anyhow::Error)` - If help record generation fails
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
    /// This method adds a service implementation to the registry, making it available
    /// for handling DNS queries that match the specified suffix. Once registered,
    /// queries like "mumbai.time.example.com" will be routed to the service
    /// registered with suffix "time".
    ///
    /// This is the "plugin" mechanism - it allows new services to be added to the
    /// DNS server dynamically. Each service becomes available at its own subdomain
    /// within the authoritative domain.
    ///
    /// ## Arguments
    /// * `suffix` - The DNS query suffix (e.g., "ip", "pi", "time") to associate with the service
    /// * `service` - A boxed implementation of the Service trait to handle queries for this suffix
    ///
    /// ## Example
    /// ```
    /// let mut handlers = DnsHandlers::new(domain)?;
    /// handlers.register("ip".to_string(), Box::new(IpService::new()));
    /// ```
    pub fn register(&mut self, suffix: String, service: Box<dyn Service>) {
        self.services.insert(suffix.clone(), service);
        tracing::info!("Registered service for suffix: {}", suffix);
    }

    /// Creates help TXT records that describe available DNS services.
    ///
    /// This method generates informative TXT records that users can query to learn
    /// about available services and how to use them. The records provide examples
    /// of valid DNS queries for each service.
    ///
    /// This is the "user manual" generator - it creates DNS records that serve as
    /// documentation, helping users understand what services are available and how
    /// to access them through DNS queries.
    ///
    /// ## Arguments
    /// * `domain` - The domain name for which to generate help records
    ///
    /// ## Returns
    /// * `Ok(Vec<Record>)` - Vector of TXT records containing help information
    /// * `Err(anyhow::Error)` - If record creation fails
    fn create_help_records(domain: &str) -> Result<Vec<Record>> {
        let help_texts = vec![
            "Welcome! Available DNS services:".to_string(),
            format!("dig TXT ip.{}", domain),
            format!("dig A pi.{}", domain),
            format!("dig TXT <location>.time.{}", domain),
            format!("dig TXT <number>.uuid.{}", domain),
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
    /// This function provides IP echo functionality, allowing clients to discover
    /// their own IP address through DNS queries. It supports both TXT and A record
    /// types, returning the client's IP address in the appropriate format.
    ///
    /// This is a "self-discovery" service - clients can query their own IP address
    /// through DNS, which is useful for network diagnostics, automation scripts,
    /// or determining external IP addresses from behind NAT/firewalls.
    ///
    /// ## Arguments
    /// * `request` - The DNS request, used to extract the client's source IP address
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (TXT or A)
    ///
    /// ## Returns
    /// * `Some(Record)` - DNS record containing the client's IP address
    /// * `None` - If the query type is not supported or IPv6 is requested as A record
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
                // Return as an A record if it's IPv4
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

    /// Handles Pi constant queries, returning Pi in different formats.
    ///
    /// This function provides access to the mathematical constant Pi (π) through
    /// DNS queries. It supports multiple record types, returning Pi as text,
    /// IPv4 address, or IPv6 address depending on the query type.
    ///
    /// This is a "mathematical constant" service - it provides easy access to Pi
    /// through DNS, which can be useful for educational purposes, mathematical
    /// calculations, or as a demonstration of DNS-based data retrieval.
    ///
    /// ## Arguments
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (TXT, A, or AAAA)
    ///
    /// ## Returns
    /// * `Some(Record)` - DNS record containing Pi in the requested format
    /// * `None` - If the query type is not supported
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

    /// Processes service requests by routing them to the appropriate service implementation.
    ///
    /// This function is the core request processor that handles dynamic service queries.
    /// It validates incoming requests, extracts the meaningful query portion, looks up
    /// the appropriate service, and formats the response as DNS records.
    ///
    /// This function acts as a "DNS router" - it takes incoming DNS queries like
    /// "mumbai.time.example.com", determines that "time" is the service suffix,
    /// extracts "mumbai" as the actual query, calls the TimeService with "mumbai",
    /// and returns the formatted response as DNS records.
    ///
    /// The function implements the exact logic from the Go `register` function's closure:
    /// 1. Validates the request is a query operation
    /// 2. Limits queries to prevent abuse (max 5 questions)
    /// 3. Processes each question, filtering for TXT and A record types
    /// 4. Cleans the query name to extract the meaningful portion
    /// 5. Looks up the appropriate service and calls its query method
    /// 6. Converts service responses to DNS records with proper names
    ///
    /// ## Arguments
    /// * `request` - The incoming DNS request
    /// * `suffix` - The service suffix to route the request to
    ///
    /// ## Returns
    /// * `Ok(Vec<Record>)` - Vector of DNS records containing the service response
    /// * `Err(anyhow::Error)` - If request processing fails
    async fn process_service_request(
        &self,
        request: &Request,
        suffix: &str,
    ) -> Result<Vec<Record>> {
        if request.op_code() != OpCode::Query {
            return Err(anyhow!("Not a query operation"));
        }

        if request.queries().len() > 5 {
            return Err(anyhow!("Too many queries"));
        }

        let mut output_records = Vec::new();

        for query in request.queries() {
            let query_type = query.query_type();

            if query_type != RecordType::TXT && query_type != RecordType::A {
                continue;
            }

            let query_name = query.name();

            let domain_suffix = format!(".{}.{}", suffix, self.domain.to_string());
            let cleaned_query = Self::clean_query(&query_name.to_string(), &domain_suffix);

            if let Some(service) = self.services.get(suffix) {
                match service.query(&cleaned_query).await {
                    Ok(answers) => match Self::create_response(answers) {
                        Ok(mut dns_records) => {
                            for record in &mut dns_records {
                                record.set_name(query_name.clone().into());
                            }
                            output_records.extend(dns_records);
                        }
                        Err(e) => {
                            tracing::error!("Error preparing response: {}", e);
                            return Err(anyhow!("Error preparing a response"));
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(output_records)
    }

    /// Cleans a DNS query string by removing the domain suffix and sanitizing the input.
    ///
    /// This function extracts the meaningful portion of a DNS query by removing
    /// the service suffix and domain, then sanitizes the remaining text to ensure
    /// it only contains safe characters.
    ///
    /// This is a "query parser" - it takes a full DNS query like "mumbai.time.example.com"
    /// and extracts just "mumbai" as the actual query for the service. It also ensures
    /// the extracted query is safe by removing potentially dangerous characters.
    ///
    /// ## Arguments
    /// * `query` - The full DNS query string (e.g., "mumbai.time.example.com.")
    /// * `suffix` - The suffix to remove from the end of the query (e.g., ".time.example.com.")
    ///
    /// ## Returns
    /// A cleaned version of the query string, containing only allowed characters.
    ///
    /// ## Example
    /// ```
    /// let cleaned = clean_query("mumbai.time.example.com.", ".time.example.com.");
    /// assert_eq!(cleaned, "mumbai");
    /// ```
    pub fn clean_query(query: &str, suffix: &str) -> String {
        let trimmed = query.trim_end_matches(suffix).trim_end_matches('.');
        RE_CLEAN.replace_all(trimmed, "").to_string()
    }

    /// Converts a vector of answer strings into a vector of DNS TXT records.
    ///
    /// This function takes the raw response strings from a service and converts
    /// them into proper DNS TXT records. Each string becomes a separate TXT record
    /// with a standard TTL and the root name as the record name.
    ///
    /// This is a "response formatter" - it bridges the gap between service logic
    /// (which returns simple strings) and DNS protocol requirements (which need
    /// properly formatted records). It ensures service responses are compatible
    /// with DNS standards.
    ///
    /// ## Arguments
    /// * `answers` - A vector of strings, each representing a TXT record's data
    ///
    /// ## Returns
    /// * `Ok(Vec<Record>)` - A vector of DNS TXT records if all answers are successfully converted
    /// * `Err(anyhow::Error)` - If there is an error parsing the root name
    ///
    /// ## Example
    /// ```
    /// let records = make_response(vec!["Hello, world!".to_string()]).unwrap();
    /// assert_eq!(records.len(), 1);
    /// ```
    pub fn create_response(answers: Vec<String>) -> Result<Vec<Record>> {
        let mut records = Vec::with_capacity(answers.len());
        for answer in answers {
            // Each answer is wrapped as a TXT record with TTL 60 and root name.
            let record = Record::from_rdata(
                Name::from_str(".")?,
                60,
                RData::TXT(rdata::TXT::new(vec![answer])),
            );
            records.push(record);
        }
        Ok(records)
    }
    /// Creates an error response as a TXT record.
    ///
    /// This function creates a standardized error response that can be returned
    /// to DNS clients when something goes wrong. It uses a short TTL (1 second)
    /// to ensure errors don't get cached for long periods.
    ///
    /// ## Arguments
    /// * `query_name` - The DNS name that was queried
    /// * `error_msg` - The error message to include in the response
    ///
    /// ## Returns
    /// A DNS TXT record containing the error message
    pub fn create_error_response(query_name: &Name, error_msg: &str) -> Record {
        Record::from_rdata(
            query_name.clone(),
            1,
            RData::TXT(rdata::TXT::new(vec![format!("error: {}", error_msg)])),
        )
    }

    /// Handles unknown DNS queries with an error message.
    ///
    /// This function provides a helpful error response when a query doesn't match
    /// any known service. It directs users to the help system for guidance.
    ///
    /// ## Arguments
    /// * `query_name` - The DNS name that was queried
    ///
    /// ## Returns
    /// A DNS TXT record with error message and help instructions
    pub fn handle_default_query(&self, query_name: &Name) -> Record {
        Self::create_error_response(
            query_name,
            &format!("unknown query, try: dig help @{}", self.domain.to_string()),
        )
    }

    /// Handles DNS queries for help information.
    ///
    /// This function returns pre-generated help records that describe available
    /// services and provide usage examples. It's the equivalent of a DNS-based
    /// user manual.
    ///
    /// ## Arguments
    /// * `query_name` - The DNS name that was queried (typically "help.domain")
    ///
    /// ## Returns
    /// A vector of DNS TXT records containing help information
    pub fn handle_help_query(&self, query_name: &Name) -> Vec<Record> {
        self.help_records
            .iter()
            .map(|record| {
                let mut new_record = record.clone();
                new_record.set_name(query_name.clone());
                new_record
            })
            .collect()
    }

    /// Main DNS request handler that routes queries to appropriate services.
    ///
    /// This is the entry point for all DNS queries. It determines which service
    /// to use based on the query name and delegates to the appropriate handler.
    ///
    /// ## Arguments
    /// * `request` - The incoming DNS request
    ///
    /// ## Returns
    /// * `Ok(Vec<Record>)` - Vector of DNS records to return to the client
    /// * `Err(anyhow::Error)` - If request processing fails
    pub async fn handle_request(&self, request: &Request) -> Result<Vec<Record>> {
        if request.queries().is_empty() {
            return Err(anyhow!("No queries in request"));
        }

        let query = &request.queries()[0];
        let query_name = query.name();
        let query_type = query.query_type();
        let query_str = query_name.to_string();

        // Handle help queries
        if query_str.ends_with(&format!("help.{}", self.domain.to_string())) {
            return Ok(self.handle_help_query(query_name));
        }

        // Handle IP service queries
        if query_str.ends_with(&format!("ip.{}", self.domain.to_string())) {
            if let Some(record) = self.handle_ip_query(request, query_name, query_type).await {
                return Ok(vec![record]);
            }
        }

        // Handle Pi service queries
        if query_str.ends_with(&format!("pi.{}", self.domain.to_string())) {
            if let Some(record) = self.handle_pi_query(query_name, query_type).await {
                return Ok(vec![record]);
            }
        }

        // Handle service queries (uuid, time, etc.)
        for (suffix, _) in &self.services {
            if query_str.ends_with(&format!(".{}.{}", suffix, self.domain.to_string())) {
                return self.process_service_request(request, suffix).await;
            }
        }

        // Handle unknown queries
        Ok(vec![self.handle_default_query(query_name)])
    }
}
