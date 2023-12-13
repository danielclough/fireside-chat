pub fn get_path(protocol: &str) -> String {
    // Instantiate addr websocket_server_address with .env or default values.
    let mut url = dotenv!("BACKEND_URL");
    let mut port = dotenv!("BACKEND_PORT");
    if url.is_empty() {
        url = "127.0.0.1";
    }
    if port.is_empty() {
        port = "3000";
    }
    if protocol == "http" {
        if port == "443" {
            format!("https://{}:{}/inference", url, port)
        } else {
            format!("http://{}:{}/inference", url, port)
        }
    } else {
        match port == "443" {
            true => format!("wss://{}:{}/websocket", url, port),
            false => format!("ws://{}:{}/websocket", url, port),
        }
    }
}
