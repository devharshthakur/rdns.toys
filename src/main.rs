use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;
use hickory_proto::rr::LowerName;
use hickory_server::ServerFuture;
use hickory_server::authority::{Catalog, ZoneType};
use hickory_server::store::in_memory::InMemoryAuthority;
use std::sync::Arc;
use tokio::net::UdpSocket;

use rdns_toys::handlers::DnsHandlers;
use rdns_toys::services::uuid::UUidService;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let domain = "localhost";
    let port = 8053;

    println!("🚀 Starting rdns-toys DNS server on {}:{}", domain, port);

    // Create DNS handlers
    let domain_name = LowerName::from_str(domain)?;
    let mut handlers = DnsHandlers::new(domain_name.clone())?;

    // Register UUID service
    let uuid_service = UUidService::new(10); // Max 10 UUIDs per query
    handlers.register("uuid".to_string(), Box::new(uuid_service));

    println!("✅ Registered UUID service");
    println!(" Test commands:");
    println!("   dig TXT uuid.{} @127.0.0.1 -p {}", domain, port);
    println!("   dig TXT 5.uuid.{} @127.0.0.1 -p {}", domain, port);
    println!("   dig TXT help.{} @127.0.0.1 -p {}", domain, port);

    // Create authority and catalog for DNS server
    let authority = InMemoryAuthority::empty(domain_name.clone().into(), ZoneType::Primary, false);
    let mut catalog = Catalog::new();
    catalog.upsert(domain_name.clone(), vec![Arc::new(authority)]);

    // Create server future
    let mut server = ServerFuture::new(catalog);

    // Bind UDP socket
    let udp_socket = UdpSocket::bind(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port,
    ))
    .await?;
    server.register_socket(udp_socket);

    println!(" DNS server listening on 127.0.0.1:{} (UDP only)", port);
    println!("⏹️  Press Ctrl+C to stop");

    // Start the server
    server.block_until_done().await?;

    Ok(())
}
