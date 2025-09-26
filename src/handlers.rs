use std::collections::HashMap;
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

use crate::services;

// --- Regex for cleaning Queries ---//
static RE_CLEAN: Lazy<Regex> =
    Lazy::new(|| Regex::new("[^a-zA-Z0-9/\\-\\.:,]").expect("Invalid regex pattern"));

#[async_trait]
/// Trait for DNS service plugins.
pub trait Service: Send + Sync {
    /// Handle a DNS query and return DNS records directly.
    /// This unified method can handle both text-based queries and direct DNS record creation.
    ///
    /// ## Arguments
    /// * `request` - The full DNS request
    /// * `query_name` - The DNS name being queried  
    /// * `query_type` - The type of DNS record requested (TXT, A, AAAA, etc.)
    /// * `cleaned_query` - The cleaned query string (for text-based services)
    ///
    /// ## Returns
    /// * `Some(Vec<Record>)` - Vector of DNS records to return
    /// * `None` - If the service doesn't support this query/record type
    async fn query(
        &self,
        request: &Request,
        query_name: &Name,
        query_type: RecordType,
        cleaned_query: &str,
    ) -> Option<Vec<Record>>;

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
    /// ## Arguments
    /// * `domain` - The authoritative domain this handler will manage
    ///
    /// ## Returns
    /// * `Ok(DnsHandlers)` - A fully initialized handler instance
    /// * `Err(anyhow::Error)` - If help record generation fails
    pub fn new(domain: LowerName) -> Result<Self> {
        let help_records = services::create_help_records(&domain.to_string())?;
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

    /// Routes service requests to the correct service implementation and formats the DNS response.
    ///
    /// This function is the main dynamic DNS service router. It:
    /// - Validates that the request is a standard query (``OpCode::Query``)
    /// - Enforces a maximum of 5 queries per request to prevent abuse
    /// - Iterates over each question, only processing ``TXT`` and ``A`` record types
    /// - Extracts the relevant query portion by removing the service suffix and domain
    /// - Looks up the registered service for the given suffix and invokes its async ``query`` method
    /// - Converts the service's response into DNS records, setting the correct query name
    ///
    /// Example: For a query like ``mumbai.time.example.com``, this function will:
    /// - Recognize ``time`` as the service suffix
    /// - Extract ``mumbai`` as the query argument
    /// - Call the TimeService with ``mumbai``
    /// - Return the result as properly named DNS records
    ///
    /// # Arguments
    /// * `request` - The incoming DNS request to process
    /// * `suffix` - The service suffix (e.g., "time", "uuid") to route to
    ///
    /// # Returns
    /// * `Ok(Vec<Record>)` - DNS records with the service's response
    /// * `Err(anyhow::Error)` - If validation or service processing fails
    async fn process_service_request(
        &self,
        request: &Request,
        suffix: &str,
    ) -> Result<Vec<Record>> {
        // Validate OpCode is Query
        if request.op_code() != OpCode::Query {
            return Err(anyhow!("Not a query operation"));
        }

        // Enforce maximum of 5 queries per request
        if request.queries().len() > 5 {
            return Err(anyhow!("Too many queries"));
        }

        // Prepare output records (as vector)
        let mut output_records = Vec::new();

        // Iterate over each query in the request
        for query in request.queries() {
            let query_type = query.query_type();

            // Only process TXT and A record types
            if query_type != RecordType::TXT && query_type != RecordType::A {
                continue;
            }

            // Extract query name
            let query_name = query.name();

            // Build domain suffix and clean the query
            let domain_suffix = format!(".{}.{}", suffix, self.domain.to_string());
            let cleaned_query = Self::clean_query(&query_name.to_string(), &domain_suffix);

            // Lookup the registered service for the given suffix
            if let Some(service) = self.services.get(suffix) {
                // Call the unified query method
                if let Some(records) = service
                    .query(request, query_name, query_type, &cleaned_query)
                    .await
                {
                    // Set the correct query name for each record
                    let mut named_records = Vec::new();
                    for mut record in records {
                        record.set_name(query_name.clone().into());
                        named_records.push(record);
                    }
                    output_records.extend(named_records);
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
        let without_trailing_dot = query.trim_end_matches('.');
        let trimmed = without_trailing_dot.trim_end_matches(suffix);
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

    /// Processes DNS queries by routing them to appropriate services.
    ///
    /// This is the business logic entry point for all DNS queries. It determines which service
    /// to use based on the query name and delegates to the appropriate handler.
    ///
    /// ## Arguments
    /// * `request` - The incoming DNS request
    ///
    /// ## Returns
    /// * `Ok(Vec<Record>)` - Vector of DNS records to return to the client
    /// * `Err(anyhow::Error)` - If request processing fails
    pub async fn process_dns_query(&self, request: &Request) -> Result<Vec<Record>> {
        if request.queries().is_empty() {
            return Err(anyhow!("No queries in request"));
        }

        let query = &request.queries()[0];
        let query_name = query.name();
        let query_str = query_name.to_string();

        tracing::info!(
            "Handling request - query_str: '{}', domain: '{}'",
            query_str,
            self.domain.to_string()
        );

        // Handle help queries
        if query_str.ends_with(&format!("help.{}.", self.domain.to_string())) {
            return Ok(self.handle_help_query(query_name));
        }

        // Handle service queries (ip, uuid, time, etc.)
        tracing::debug!("Checking services for query: '{}'", query_str);
        for (suffix, _) in &self.services {
            // Check for both formats: "suffix.domain." and ".suffix.domain."
            let expected_with_dot = format!(".{}.{}.", suffix, self.domain.to_string());
            let expected_without_dot = format!("{}.{}.", suffix, self.domain.to_string());

            if query_str.ends_with(&expected_with_dot) || query_str.ends_with(&expected_without_dot)
            {
                tracing::debug!("  Match found for suffix: '{}'", suffix);
                return self.process_service_request(request, suffix).await;
            }
        }
        tracing::debug!("No service matched, returning default error");

        // Handle unknown queries using default query case
        Ok(vec![self.handle_default_query(query_name)])
    }
}
