use std::{env, thread, time::Duration, error::Error};
use winreg::enums::*;
use winreg::RegKey;

pub fn get() -> Result<u32, Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
            KEY_READ,
        )?;
    let proxy: u32 = ie_settings.get_value("ProxyEnable")?;
    Ok(proxy)
}

pub fn set(proxy: u32) -> Result<(), Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
            KEY_SET_VALUE,
        )?;
    ie_settings.set_value("ProxyEnable", &proxy)?;
    Ok(())
}

// interval : delay (in seconds) between checks (does system setting matches requested user setting ?)
pub fn check(interval: u64) {
    thread::spawn(move || {
        let check_interval = Duration::new(interval, 0);
        loop {
            thread::sleep(check_interval);
            let user_proxy_status: u32 = env::var("USER_PROXY_STATUS").unwrap().parse().unwrap();
            #[cfg(debug_assertions)]
            {
                println!(
                    "System proxy : {}\nUser requested proxy state : {}\n",
                    get().unwrap(),
                    user_proxy_status
                );
            }
            if get().unwrap() != user_proxy_status {
                set(user_proxy_status).unwrap();
            }
        }
    });
}
