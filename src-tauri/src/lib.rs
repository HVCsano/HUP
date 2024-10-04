// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_exchange_rates() -> String {
    let body = ureq::get("https://open.er-api.com/v6/latest/HUF")
        .call()
        .expect("Change rate get failed")
        .into_string()
        .unwrap();
    body
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_exchange_rates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
