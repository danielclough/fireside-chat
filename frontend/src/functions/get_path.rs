pub fn get_llm_path(route: &str, ipv4: String) -> String {
    println!("ws://{}:443/{}",ipv4, route);
    println!("http://{}:443/{}",ipv4, route);

    if route == "websocket" {
        format!("ws://{}:443/{}",ipv4, route)
    } else {
        format!("http://{}:443/{}",ipv4, route)
    }
}

pub fn get_database_path(route: &str) -> String {
    println!("http://fireside-database.shuttleapp.rs:443/{}", route);
    format!("http://fireside-database.shuttleapp.rs:443/{}", route)
}