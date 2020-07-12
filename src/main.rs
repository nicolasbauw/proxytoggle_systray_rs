#![windows_subsystem = "windows"]
use std::{env, fs, path::PathBuf, thread, time::Duration};
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    // Creates temp file to store user requested proxy status (as we can't share data between threads with the systray crate...)
    // At start : user has not changed anything, so user status matches current system status.
    write_user_status(get_proxy());
    // Checking system proxy every second (in case of a nasty system policy sets it...)
    check_proxy(1);

    // Creates the systray and menus
    if create_systray().is_err() {
        println!("Systray creation error");
        return;
    };
}

fn create_systray() -> Result<(), systray::Error> {
    let enabled_icon = include_bytes!("../assets/checkmark.ico");
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => return Err(systray::Error::UnknownError),
    };
    app.set_tooltip(&"Proxy toggle".to_string())?;
    app.set_icon_from_buffer(enabled_icon, 128, 128)?;

    app.add_menu_item("Proxy enable", move |_| {
        write_user_status(1);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Proxy disable", |_| {
        write_user_status(0);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}

fn get_proxy() -> u32 {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
            KEY_READ,
        )
        .unwrap();
    let proxy: u32 = ie_settings.get_value("ProxyEnable").unwrap();
    proxy
}

fn set_proxy(proxy: u32) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
            KEY_SET_VALUE,
        )
        .unwrap();
    ie_settings.set_value("ProxyEnable", &proxy).unwrap();
}

// interval : delay (in seconds) between checks (does system setting matches requested user setting ?)
fn check_proxy(interval: u64) {
    thread::spawn(move || {
        let check_interval = Duration::new(interval, 0);
        loop {
            thread::sleep(check_interval);
            let mut d = PathBuf::new();
            d.push(env::temp_dir());
            d.push("user_status.txt");
            let user_proxy_state: u32 = String::from_utf8(fs::read(d).unwrap())
                .unwrap()
                .parse()
                .unwrap();
            #[cfg(debug_assertions)]
            {
                println!(
                    "System proxy : {}\nUser requested proxy state : {}\n",
                    get_proxy(),
                    user_proxy_state
                );
            }
            if get_proxy() != user_proxy_state {
                set_proxy(user_proxy_state);
            }
        }
    });
}

fn write_user_status(status: u32) {
    let mut d = PathBuf::new();
    d.push(env::temp_dir());
    d.push("user_status.txt");
    fs::write(d, status.to_string()).unwrap();
}
