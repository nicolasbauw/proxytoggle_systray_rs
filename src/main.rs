#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{error::Error, sync::Arc, sync::Mutex};
mod proxy;

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    proxy_on: nwg::Icon,
    proxy_off: nwg::Icon,
    proxy_unkn: nwg::Icon,

    #[nwg_control(icon: Some(&data.proxy_unkn))]
    #[nwg_events(OnContextMenu: [SystemTray::show_menu])]
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

    fn set_initial_icon(&self) -> Result<(), Box<dyn Error>> {
        let initial_state = proxy::get()?;
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
        let mut us = self.user_proxy_status.lock().unwrap();
        *us = initial_state;
        Ok(())
    }

    fn proxy_on(&self) {
        let mut us = self.user_proxy_status.lock().unwrap();
        *us = 1;
        self.tray.set_icon(&self.proxy_on);
        self.tray.set_tip("Proxy is enabled");
    }

    fn proxy_off(&self) {
        let mut us = self.user_proxy_status.lock().unwrap();
        *us = 0;
        self.tray.set_icon(&self.proxy_off);
        self.tray.set_tip("Proxy is disabled");
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn load_icon(data: &[u8]) -> nwg::Icon {
    let mut icon = nwg::Icon::default();

    nwg::Icon::builder()
        .source_bin(Some(data))
        .strict(true)
        .build(&mut icon)
        .unwrap();

    icon
}

fn main() -> Result<(), Box<dyn Error>> {
    let _enabled_icon = include_bytes!("../assets/check-mark-16.ico");
    let disabled_icon = include_bytes!("../assets/x-mark-16.ico");
    let _unknown_icon = include_bytes!("../assets/question-mark-16.ico");

    // Building the systray
    nwg::init()?;
    let ui = SystemTray::build_ui(Default::default())?;
    ui.proxy_off = load_icon(disabled_icon);


    // Setting initial icon + starts event loop
    ui.set_initial_icon()?;

    // Setting initial status + starts periodic check
    let us = Arc::clone(&ui.user_proxy_status);
    proxy::check(1, us);

    nwg::dispatch_thread_events();
    Ok(())
}
