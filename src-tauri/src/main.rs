#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use std::fs;
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose};

fn get_data_path() -> PathBuf {
    std::env::current_dir().unwrap().join("data.json")
}

#[tauri::command]
fn load_data() -> Result<String, String> {
    let path = get_data_path();
    if !path.exists() {
        return Ok(String::from("{\"weekly\":[], \"daily\":[], \"notes\":[], \"weekly_progress\":0, \"daily_progress\":0}"));
    }
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_data(data: String) -> Result<(), String> {
    fs::write(get_data_path(), data).map_err(|e| e.to_string())
}

#[tauri::command]
fn apply_wallpaper(base64_img: String) -> Result<(), String> {
    // Strip the data prefix provided by the frontend
    let b64_data = base64_img.split(',').nth(1).unwrap_or(&base64_img);
    let image_bytes = general_purpose::STANDARD.decode(b64_data).map_err(|e| e.to_string())?;
    
    let out_path = std::env::current_dir().unwrap().join("output.png");
    fs::write(&out_path, image_bytes).map_err(|e| e.to_string())?;

    // Set the wallpaper using native Windows API
    wallpaper::set_from_path(out_path.to_str().unwrap()).map_err(|e| e.to_string())?;
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Build the system tray menu
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close(); // Keeps app running in tray when X is clicked
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![load_data, save_data, apply_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}