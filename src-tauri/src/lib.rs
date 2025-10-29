use plotly::{Plot, Scatter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn plotly_scripts() -> String {
   Plot::offline_js_sources()
}

#[tauri::command]
fn plotly_example() -> String {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    plot.to_json()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![plotly_scripts,plotly_example])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
