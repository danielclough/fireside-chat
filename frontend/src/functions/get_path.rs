pub fn get_llm_path(route: &str, url: String) -> String {
    let ssl = url.as_str() != "127.0.0.1";

    if ssl {
        if route == "websocket" {
            format!("wss://{}/{}", url, route)
        } else {
            format!("https://{}/{}", url, route)
        }
    } else if route == "websocket" {
        format!("ws://{}:16981/{}", url, route)
    } else {
        format!("http://{}:16981/{}", url, route)
    }
}

pub fn get_database_path(route: &str, url: String) -> String {
    let ssl = url.as_str() != "127.0.0.1";

    if ssl {
        format!("https://{}/{}", url, route)
    } else {
        format!("http://{}:16980/{}", url, route)
    }
}
