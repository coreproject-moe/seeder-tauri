#[tauri::command]
pub fn login_function(username: String, password: String) -> String {
    return "I was invoked from JS!".to_string();
}
