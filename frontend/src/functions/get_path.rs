pub fn get_llm_path(route: &str, url: String) -> String {
    println!("ws://{}:16981/{}", url, route);
    println!("http://{}:16981/{}", url, route);

    if route == "websocket" {
        format!("ws://{}:16981/{}", url, route)
    } else {
        format!("http://{}:16981/{}", url, route)
    }
}

pub fn get_database_path(route: &str, url: String) -> String {
    println!("http://{}:16980/{}", url, route);
    format!("http://{}:16980/{}", url, route)
}
