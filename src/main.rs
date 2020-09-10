extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{env, error::Error};
mod proxy;

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./assets/check-mark-16.ico"))]
    proxy_on: nwg::Icon,

    #[nwg_resource(source_file: Some("./assets/x-mark-16.ico"))]
    proxy_off: nwg::Icon,

    #[nwg_resource(source_file: Some("./assets/question-mark-16.ico"))]
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
}

impl SystemTray {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn set_initial_icon (&self) -> Result<(), Box<dyn Error>> {
        #[cfg(debug_assertions)]
            {
                println!(
                    "Initial icon proxy state : {:?}\n",
                    env::var("USER_PROXY_STATUS")
                );
            }
        if let 0 = env::var("USER_PROXY_STATUS")?.parse()? { self.tray.set_icon(&self.proxy_off); } else { self.tray.set_icon(&self.proxy_on); }
        Ok(())
    }

    fn proxy_on(&self) {
        self.tray.set_icon(&self.proxy_on);
        env::set_var("USER_PROXY_STATUS", "1");
    }
    
    fn proxy_off(&self) {
        self.tray.set_icon(&self.proxy_off);
        env::set_var("USER_PROXY_STATUS", "0");
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // Setting initial status + starts periodic check
    env::set_var("USER_PROXY_STATUS", proxy::get()?.to_string());
    proxy::check(1);

    nwg::init()?;
    let ui = SystemTray::build_ui(Default::default())?;
    ui.set_initial_icon()?;
    nwg::dispatch_thread_events();
    Ok(())
}