#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{error::Error, sync::Arc, sync::Mutex};
mod proxy;

const ENABLED_ICON: &[u8] = include_bytes!("../assets/plus-16.ico");
const DISABLED_ICON: &[u8] = include_bytes!("../assets/minus-16.ico");
const UNKNOWN_ICON: &[u8] = include_bytes!("../assets/question-mark-4-16.ico");

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_bin: Some(ENABLED_ICON))]
    proxy_on: nwg::Icon,

    #[nwg_resource(source_bin: Some(DISABLED_ICON))]
    proxy_off: nwg::Icon,

    #[nwg_resource(source_bin: Some(UNKNOWN_ICON))]
    proxy_unkn: nwg::Icon,

    #[nwg_control(icon: Some(&data.proxy_unkn))]
    #[nwg_events(OnMousePress: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Proxy ON")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::proxy_on])]
    tray_item1: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Proxy OFF")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::proxy_off])]
    tray_item2: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item3: nwg::MenuItem,

    user_proxy_status: Arc<Mutex<u32>>,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn set_initial_icon(&self) {
        let initial_state = match proxy::get() {
            Ok(0) => 0,
            Ok(1) => 1,
            _ => return
        };
        #[cfg(debug_assertions)]
        {
            println!("Initial proxy state : {:?}\n", initial_state);
        }
        if let 0 = initial_state {
            self.tray.set_icon(&self.proxy_off);
            self.tray.set_tip("Proxy is disabled");
        } else {
            self.tray.set_icon(&self.proxy_on);
            self.tray.set_tip("Proxy is enabled");
        }
        let mut us = match self.user_proxy_status.lock() {
            Ok(us) => us,
            _ => return
        };
        *us = initial_state;
    }

    fn proxy_on(&self) {
        let mut us = match self.user_proxy_status.lock() {
            Ok(us) => us,
            _ => return
        };
        *us = 1;
        self.tray.set_icon(&self.proxy_on);
        self.tray.set_tip("Proxy is enabled");
    }

    fn proxy_off(&self) {
        let mut us = match self.user_proxy_status.lock() {
            Ok(us) => us,
            _ => return
        };
        *us = 0;
        self.tray.set_icon(&self.proxy_off);
        self.tray.set_tip("Proxy is disabled");
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Building the systray
    nwg::init()?;
    let ui = SystemTray::build_ui(Default::default())?;
    
    // Setting initial icon
    ui.set_initial_icon();

    // Setting initial status + starts periodic check
    let us = Arc::clone(&ui.user_proxy_status);
    proxy::check(1, us);

    // Starts event loop
    nwg::dispatch_thread_events();
    Ok(())
}
