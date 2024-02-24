pub fn get_llm_path(route: &str, ipv4: String) -> String {
    println!("ws://{}:16981/{}",ipv4, route);
    println!("http://{}:16981/{}",ipv4, route);

    if route == "websocket" {
        format!("ws://{}:16981/{}",ipv4, route)
    } else {
        format!("http://{}:16981/{}",ipv4, route)
    }
}

pub fn get_database_path(route: &str) -> String {
    println!("http://127.0.0.1:16980/{}", route);
    format!("http://127.0.0.1:16980/{}", route)
}