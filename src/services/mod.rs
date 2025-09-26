pub mod pi;
pub mod uuid;

use crate::handlers::DnsHandlers;
use crate::services::pi::PiService;
use crate::services::uuid::UUidService;
use anyhow::Result;
use hickory_proto::rr::{Name, RData, Record, rdata};
use std::str::FromStr;

// Constants
const IP_TTL: u32 = 60;

/// Registers all available DNS services with the handlers.
///
/// This function centralizes service registration, making it easy to add new services
/// and test them individually. Currently registers: uuid, ip, and pi services.
pub fn register_services(handlers: &mut DnsHandlers) -> Result<()> {
    // Register UUID service
    let uuid_service = UUidService::new(10); // Max 10 UUIDs per query
    handlers.register("uuid".to_string(), Box::new(uuid_service));
    tracing::info!("✅ Registered UUID service");

    // Register Pi service
    let pi_service = PiService::new();
    handlers.register("pi".to_string(), Box::new(pi_service));
    tracing::info!("✅ Registered Pi service");

    Ok(())
}

/// Creates help TXT records that describe available DNS services.
///
/// This function generates informative TXT records that users can query to learn
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
pub fn create_help_records(domain: &str) -> Result<Vec<Record>> {
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
