use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;
// Removed unused imports
use hickory_proto::rr::LowerName;
use hickory_server::authority::MessageResponseBuilder;
// Removed unused Arc import
use tokio::net::UdpSocket;

use hickory_server::{
    ServerFuture,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use rdns_toys::handlers::DnsHandlers;
use rdns_toys::services::uuid::UUidService;

/// Custom request handler for Rdns Project : It is best way to integrate our own DnsHandler with the hickory server
pub struct RdnsRequestHandler {
    handlers: DnsHandlers,
}

impl RdnsRequestHandler {
    pub fn new(handlers: DnsHandlers) -> Self {
        Self { handlers }
    }
}

#[async_trait::async_trait]
impl RequestHandler for RdnsRequestHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        // Process the request using our custom handlers
        match self.handlers.handle_request(request).await {
            Ok(records) => {
                // Create a response header
                let mut response_header =
                    hickory_proto::op::Header::response_from_request(request.header());
                response_header.set_id(request.id());
                response_header.set_message_type(hickory_proto::op::MessageType::Response);
                response_header.set_op_code(hickory_proto::op::OpCode::Query);
                response_header.set_authoritative(true);
                response_header.set_recursion_available(false);
                response_header.set_recursion_desired(request.recursion_desired());
                response_header.set_checking_disabled(request.checking_disabled());

                // Create a MessageResponse with the records
                let response = MessageResponseBuilder::from_message_request(request).build(
                    response_header,
                    records.iter(),
                    std::iter::empty(),
                    std::iter::empty(),
                    std::iter::empty(),
                );

                // Send the response
                response_handle
                    .send_response(response)
                    .await
                    .unwrap_or_else(|_err| {
                        tracing::error!("Failed to send response");
                        ResponseInfo::from(hickory_proto::op::Header::new())
                    })
            }
            Err(err) => {
                tracing::error!("Error handling request: {}", err);

                // Create an error response header
                let mut response_header =
                    hickory_proto::op::Header::response_from_request(request.header());
                response_header.set_id(request.id());
                response_header.set_message_type(hickory_proto::op::MessageType::Response);
                response_header.set_op_code(hickory_proto::op::OpCode::Query);
                response_header.set_response_code(hickory_proto::op::ResponseCode::ServFail);
                response_header.set_authoritative(true);
                response_header.set_recursion_available(false);
                response_header.set_recursion_desired(request.recursion_desired());
                response_header.set_checking_disabled(request.checking_disabled());

                // Create an error MessageResponse with no records
                let response = MessageResponseBuilder::from_message_request(request)
                    .build_no_records(response_header);

                // Send the error response
                response_handle
                    .send_response(response)
                    .await
                    .unwrap_or_else(|_e| {
                        tracing::error!("Failed to send error response");
                        ResponseInfo::from(hickory_proto::op::Header::new())
                    })
            }
        }
    }
}

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
