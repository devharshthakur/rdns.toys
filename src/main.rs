use std::iter;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;

use hickory_proto::op::Header;
use hickory_proto::rr::LowerName;
use hickory_server::authority::MessageResponseBuilder;

use tokio::net::UdpSocket;

use hickory_server::{
    ServerFuture,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use rdns_toys::handlers::DnsHandlers;
use rdns_toys::services;

/// Custom request handler for Rdns Project
//It is best way to integrate our own DnsHandler with the hickory server
pub struct RdnsRequestHandler {
    handlers: DnsHandlers,
}

impl RdnsRequestHandler {
    pub fn new(handlers: DnsHandlers) -> Self {
        Self { handlers }
    }

    /// Creates a response header with minimal configuration.
    /// Uses the built-in response_from_request which already handles most fields
    fn create_response_header(&self, request: &Request, is_error: bool) -> Header {
        let mut header = Header::response_from_request(request.header());
        header.set_authoritative(true);
        header.set_recursion_available(false);

        if is_error {
            header.set_response_code(hickory_proto::op::ResponseCode::ServFail);
        }

        header
    }
}

#[async_trait::async_trait]
impl RequestHandler for RdnsRequestHandler {
    /// Handles incoming DNS requests by routing them to appropriate services and sending responses.
    ///
    /// Processes requests through the DnsHandlers, creates proper DNS response headers,
    /// and sends the response back to the client. Uses built-in Hickory functions for
    /// simplified header construction.
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        // Process the request using our custom handlers
        match self.handlers.process_dns_query(request).await {
            Ok(records) => {
                // Create response header using built-in function (much simpler!)
                let response_header = self.create_response_header(request, false);

                // Create a MessageResponse with the records
                let response = MessageResponseBuilder::from_message_request(request).build(
                    response_header,
                    records.iter(),
                    iter::empty(),
                    iter::empty(),
                    iter::empty(),
                );

                // Send the response
                response_handle
                    .send_response(response)
                    .await
                    .unwrap_or_else(|_err| {
                        tracing::error!("Failed to send response");
                        ResponseInfo::from(Header::new())
                    })
            }
            Err(err) => {
                tracing::error!("Error handling request: {}", err);

                // Create error response header using built-in function
                let response_header = self.create_response_header(request, true);

                // Create an error MessageResponse with no records
                let response = MessageResponseBuilder::from_message_request(request)
                    .build_no_records(response_header);

                // Send the error response
                response_handle
                    .send_response(response)
                    .await
                    .unwrap_or_else(|_err| {
                        tracing::error!("Failed to send error response");
                        ResponseInfo::from(hickory_proto::op::Header::new())
                    })
            }
        }
    }
}

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
