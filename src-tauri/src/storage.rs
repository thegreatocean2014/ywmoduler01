use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
// use tauri::State;

fn get_data_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    let data_dir = home.join(".dbmoduler");
    fs::create_dir_all(&data_dir).ok();
    data_dir.join("data.json")
}

fn load_storage() -> Value {
    let path = get_data_dir();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or(json!({}))
    } else {
        json!({})
    }
}

fn save_storage(data: &Value) {
    let path = get_data_dir();
    let content = serde_json::to_string_pretty(data).unwrap_or_default();
    fs::write(path, content).ok();
}

#[tauri::command]
pub fn get_storage(key: String) -> Option<String> {
    let data = load_storage();
    data.get(&key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

#[tauri::command]
pub fn set_storage(key: String, value: String) {
    let mut data = load_storage();
    data[key] = Value::String(value);
    save_storage(&data);
}

#[tauri::command]
pub fn remove_storage(key: String) {
    let mut data = load_storage();
    if let Some(obj) = data.as_object_mut() {
        obj.remove(&key);
    }
    save_storage(&data);
}

#[tauri::command]
pub fn clear_storage() {
    let path = get_data_dir();
    fs::remove_file(path).ok();
}


