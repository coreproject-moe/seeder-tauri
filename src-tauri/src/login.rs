use reqwest;
use std::collections::HashMap;

#[tauri::command]
pub fn login_function(username: String, password: String) -> String {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);

    // let res = client.post("http://httpbin.org/post").json(&map).send()?;
    return "I was invoked from JS!".to_string();
}
