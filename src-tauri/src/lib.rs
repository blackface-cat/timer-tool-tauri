use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    pub track_names: Vec<String>,
    pub lap_records: Vec<LapRecord>,
    pub record_groups: Vec<RecordGroup>,
    pub settings: AppSettings,
}

#[derive(Serialize, Deserialize)]
pub struct LapRecord {
    pub id: String,
    pub name: String,
    pub time: f64,
    pub timestamp: f64,
    pub group_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecordGroup {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { theme: "light".to_string() }
    }
}

fn get_data_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from(".")).join("data.json")
}

#[tauri::command]
fn minimize_window<R: Runtime>(window: tauri::Window<R>) { let _ = window.minimize(); }

#[tauri::command]
fn maximize_window<R: Runtime>(window: tauri::Window<R>) {
    if window.is_maximized().unwrap_or(false) { let _ = window.unmaximize(); }
    else { let _ = window.maximize(); }
}

#[tauri::command]
fn close_window<R: Runtime>(window: tauri::Window<R>) {
    let _ = window.hide();
}

#[tauri::command]
fn is_maximized<R: Runtime>(window: tauri::Window<R>) -> bool { window.is_maximized().unwrap_or(false) }

#[tauri::command]
fn load_data<R: Runtime>(app: AppHandle<R>) -> Result<AppData, String> {
    let path = get_data_path(&app);
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else { Ok(AppData::default()) }
}

#[tauri::command]
fn save_data<R: Runtime>(app: AppHandle<R>, data: AppData) -> Result<(), String> {
    let path = get_data_path(&app);
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    let content = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_asset_path<R: Runtime>(app: AppHandle<R>, name: String) -> Result<String, String> {
    let rp = app.path().resource_dir().map_err(|e| e.to_string())?;
    Ok(rp.join("assets").join(&name).to_string_lossy().to_string())
}

#[tauri::command]
fn show_window<R: Runtime>(window: tauri::Window<R>) -> Result<(), String> {
    let _ = window.show();
    let _ = window.unminimize();
    let _ = window.set_focus();
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())
        .setup(|app| {
            // 创建系统托盘
            let show_item = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip("计时器")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.unminimize();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    tauri::tray::TrayIconEvent::DoubleClick { .. } => {
                        if let Some(app) = tray.app_handle().get_webview_window("main") {
                            let _ = app.show();
                            let _ = app.unminimize();
                            let _ = app.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            minimize_window, maximize_window, close_window, is_maximized,
            load_data, save_data, get_asset_path,
            show_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}