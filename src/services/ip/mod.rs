use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;
use hickory_proto::rr::{Name, RData, Record, RecordType, rdata};
use hickory_server::server::Request;

use crate::handlers::Service;

// Constants
const IP_TTL: u32 = 60;

/// IP service that returns the client's IP address in various formats.
///
/// This service provides IP echo functionality, allowing clients to discover
/// their own IP address through DNS queries. It supports both TXT and A record
/// types, returning the client's IP address in the appropriate format.
///
/// This is a "self-discovery" service - clients can query their own IP address
/// through DNS, which is useful for network diagnostics, automation scripts,
/// or determining external IP addresses from behind NAT/firewalls.
pub struct IpService;

impl IpService {
    /// Creates a new IP service instance.
    pub fn new() -> Self {
        Self
    }

    /// Handles IP queries, returning the client's IP address.
    ///
    /// This function provides IP echo functionality, allowing clients to discover
    /// their own IP address through DNS queries. It supports both TXT and A record
    /// types, returning the client's IP address in the appropriate format.
    ///
    /// ## Arguments
    /// * `request` - The DNS request, used to extract the client's source IP address
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (TXT or A)
    ///
    /// ## Returns
    /// * `Some(Record)` - DNS record containing the client's IP address
    /// * `None` - If the query type is not supported or IPv6 is requested as A record
    pub async fn handle_ip_query(
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
}

#[async_trait]
impl Service for IpService {
    /// Handles IP queries, returning the client's IP address in various formats.
    ///
    /// This method implements the Service trait for IP functionality, allowing
    /// it to be used as a plugin service. It extracts the client's IP address
    /// from the request and returns it in the appropriate DNS record format.
    ///
    /// ## Arguments
    /// * `request` - The DNS request containing client information
    /// * `query_name` - The DNS name being queried
    /// * `query_type` - The type of DNS record requested (TXT, A, etc.)
    /// * `_cleaned_query` - Not used for IP service (IP is extracted from request)
    ///
    /// ## Returns
    /// * `Some(Vec<Record>)` - Vector containing the client's IP address record
    /// * `None` - If the query type is not supported
    async fn query(
        &self,
        request: &Request,
        query_name: &Name,
        query_type: RecordType,
        _cleaned_query: &str,
    ) -> Option<Vec<Record>> {
        if let Some(record) = self.handle_ip_query(request, query_name, query_type).await {
            Some(vec![record])
        } else {
            None
        }
    }

    /// Exports service data for debugging or monitoring.
    ///
    /// For the IP service, this returns basic information about the service.
    ///
    /// ## Returns
    /// * `Ok(Vec<u8>)` - Serialized service information
    /// * `Err(anyhow::Error)` - If serialization fails
    async fn dump(&self) -> Result<Vec<u8>> {
        let service_info = "IP Service - Returns client IP address in TXT and A record formats";
        Ok(service_info.as_bytes().to_vec())
    }
}
