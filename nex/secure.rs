use std::env;
use std::str::FromStr;
use std::sync::Arc;
use nex::*;
use globals::*;
use log::error;

pub fn start_secure_server() {
    // Initialize the secure server
    globals::SECURE_SERVER.lock().unwrap() = Some(PRUDPServer::new());

    // Create and configure the secure endpoint
    let mut secure_endpoint = PRUDPEndPoint::new(1);
    secure_endpoint.is_secure_endpoint = true;
    secure_endpoint.server_account = globals::SECURE_SERVER_ACCOUNT.clone();
    secure_endpoint.account_details_by_jid = globals::ACCOUNT_DETAILS_BY_PID.clone();
    secure_endpoint.account_details_by_username = globals::ACCOUNT_DETAILS_BY_USERNAME.clone();
    
    // Set stream settings
    secure_endpoint.default_stream_settings.max_silence_time = 20000;
    
    // Bind the endpoint to the server
    globals::SECURE_SERVER.lock().unwrap().as_mut().unwrap().bind_prudp_endpoint(Arc::new(secure_endpoint.clone()));

    // Configure server settings
    globals::SECURE_SERVER.lock().unwrap().as_mut().unwrap().library_versions.set_default(LibraryVersion::new(2, 4, 3));
    globals::SECURE_SERVER.lock().unwrap().as_mut().unwrap().access_key = "6181dff1".to_string();

    // Set up data handler
    secure_endpoint.on_data(Box::new(|packet: Box<dyn PacketInterface>| {
        let request = packet.rmc_message();
        
        println!("=== MK7 - Secure ===");
        println!("Protocol ID: {:?}", request.protocol_id);
        println!("Method ID: {:?}", request.method_id);
        println!("====================");
    }));

    // Set up error handler
    secure_endpoint.on_error(Box::new(|err: &nex::Error| {
        error!("{}", err);
    }));

    // Register protocols
    register_common_secure_server_protocols();
    register_secure_server_nex_protocols();

    // Get port from environment and start listening
    let port = env::var("JN_MK7_SECURE_SERVER_PORT")
        .ok()
        .and_then(|p| u16::from_str(&p).ok())
        .unwrap_or(0); // Default to 0 if not set or invalid

    globals::SECURE_SERVER.lock().unwrap().as_mut().unwrap().listen(port);
}
