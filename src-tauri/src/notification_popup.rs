use tauri::{
    window::{Effect, EffectsBuilder},
    LogicalSize, Manager, PhysicalPosition, Position,
};

pub struct Notification {}
impl Notification {
    pub fn new(app_handle: &tauri::AppHandle) {
        // let popup_window = app_handle
        //     .get_webview_window("notification-window")
        //     .unwrap();

        let is_window_open = app_handle
            .get_webview_window("notification-window")
            .is_some();

        if is_window_open {
            return;
        }

        let config = app_handle.config().app.windows.get(1).unwrap();
        let popup_window = tauri::WebviewWindowBuilder::from_config(
            app_handle,
            config
        ).unwrap().build().unwrap();
        let current_monitor = popup_window.primary_monitor().unwrap().unwrap();
        let monitor_size = current_monitor.size();
        println!("Monitor size: {:?}", monitor_size);

        popup_window
            .set_size(LogicalSize {
                width: 400,
                height: 200,
            })
            .unwrap();
        let current_window_size = popup_window.outer_size().unwrap();
        const MARGIN: u32 = 40;

        println!("{:?}", current_window_size);

        let window_position = Position::new(PhysicalPosition {
            x: monitor_size.width - current_window_size.width - MARGIN,
            y: monitor_size.height - current_window_size.height - 200, // TODO: find a better way to set this value
        });

        // popup_window.set_resizable(false).unwrap();
        // popup_window.set_decorations(false).unwrap();
        // popup_window.hide_menu().unwrap();
        // popup_window.set_resizable(false).unwrap();
        // popup_window.set_always_on_top(true).unwrap();

        popup_window.set_position(window_position).unwrap();
        // popup_window.hide().unwrap();
    }
}
