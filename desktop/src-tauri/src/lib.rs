use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

use codefart_core::config::{BUILTIN_THEMES, Config};

#[derive(serde::Serialize)]
struct ThemeInfo {
    name: &'static str,
    desc: &'static str,
}

#[derive(serde::Serialize)]
struct DesktopState {
    theme: String,
    custom_sound: Option<String>,
    hook_installed: bool,
    autostart: bool,
    themes: Vec<ThemeInfo>,
}

fn command_err(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn desktop_state(app: &AppHandle) -> Result<DesktopState, String> {
    let config = Config::load().map_err(command_err)?;
    let hook_installed = codefart_core::setup::check_hook_installed().unwrap_or(false);
    let autostart = app.autolaunch().is_enabled().map_err(command_err)?;
    let themes = BUILTIN_THEMES
        .iter()
        .map(|(name, desc)| ThemeInfo { name, desc })
        .collect();

    Ok(DesktopState {
        theme: config.active_theme().to_string(),
        custom_sound: config.custom_sound,
        hook_installed,
        autostart,
        themes,
    })
}

#[tauri::command]
fn get_state(app: AppHandle) -> Result<DesktopState, String> {
    desktop_state(&app)
}

#[tauri::command]
fn set_theme(app: AppHandle, theme: String) -> Result<DesktopState, String> {
    let mut config = Config::load().map_err(command_err)?;
    config.set_theme(&theme).map_err(command_err)?;
    config.save().map_err(command_err)?;
    desktop_state(&app)
}

#[tauri::command]
fn preview_theme(theme: String) -> Result<(), String> {
    codefart_core::audio::play_theme(&theme).map_err(command_err)
}

#[tauri::command]
fn set_custom_sound(app: AppHandle, path: String) -> Result<DesktopState, String> {
    let mut config = Config::load().map_err(command_err)?;
    config
        .set_custom_sound_from_path(&path)
        .map_err(command_err)?;
    config.save().map_err(command_err)?;
    desktop_state(&app)
}

#[tauri::command]
fn clear_custom_sound(app: AppHandle) -> Result<DesktopState, String> {
    let mut config = Config::load().map_err(command_err)?;
    config.clear_custom_sound();
    config.save().map_err(command_err)?;
    desktop_state(&app)
}

#[tauri::command]
fn install_hook(app: AppHandle) -> Result<DesktopState, String> {
    codefart_core::setup::install_hook().map_err(command_err)?;
    desktop_state(&app)
}

#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<DesktopState, String> {
    let autolaunch = app.autolaunch();
    if enabled {
        autolaunch.enable().map_err(command_err)?;
    } else {
        autolaunch.disable().map_err(command_err)?;
    }
    desktop_state(&app)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_state,
            set_theme,
            preview_theme,
            set_custom_sound,
            clear_custom_sound,
            install_hook,
            set_autostart,
        ])
        .setup(|app| {
            // Make title bar transparent overlay (traffic lights on content)
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title_bar_style(tauri::TitleBarStyle::Overlay);
                let _ = window.set_title("");
            }
            let show = MenuItemBuilder::with_id("show", "Preferences").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).item(&show).item(&quit).build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("CodeFart")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
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
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Close → hide, don't quit
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { .. } = event {
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
