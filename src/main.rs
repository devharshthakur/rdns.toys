use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;

use hickory_proto::rr::LowerName;

use tokio::net::UdpSocket;

use hickory_server::ServerFuture;

use rdns_toys::handlers::{DnsHandlers, RdnsRequestHandler};
use rdns_toys::services;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing only in debug builds
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt::init();
    }

    let domain = "localhost";
    let port = 8053;

    println!("🚀 Starting rdns-toys DNS server on {}:{}", domain, port);

    // Create DNS handlers
    let domain_name = LowerName::from_str(domain)?;
    let mut handlers = DnsHandlers::new(domain_name.clone())?;

    // Register all services
    services::register_services(&mut handlers)?;

    // Create our custom request handler
    let request_handler = RdnsRequestHandler::new(handlers);

    // Create server future with our custom handler
    let mut server = ServerFuture::new(request_handler);

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
