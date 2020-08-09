use std::{sync::Arc, sync::Mutex, thread, time::Duration};
use winreg::enums::*;
use winreg::RegKey;

pub fn get() -> u32 {
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

pub fn set(proxy: u32) {
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
pub fn check(interval: u64, user_status: Arc<Mutex<u32>>) {
    thread::spawn(move || {
        let check_interval = Duration::new(interval, 0);
        loop {
            thread::sleep(check_interval);
            let us = user_status.lock().unwrap();
            let user_proxy_state: u32 = *us;
            #[cfg(debug_assertions)]
            {
                println!(
                    "System proxy : {}\nUser requested proxy state : {}\n",
                    get(),
                    user_proxy_state
                );
            }
            if get() != user_proxy_state {
                set(user_proxy_state);
            }
        }
    });
}
