//#![windows_subsystem = "windows"]
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    if create_systray().is_err() { println!("Systray creation error"); return };
    //println!("Proxy : {}", get_proxy());
}

fn create_systray() -> Result<(), systray::Error> {
    let enabled_icon = include_bytes!("../assets/Skull-blue-32.ico");
    //let disabled_icon = include_bytes!("../assets/Skull-red-32.ico");
    let tooltip = match get_proxy() {
        0 => "Proxy disabled",
        _ => "Proxy enabled"
    };
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_tooltip(&tooltip.to_string())?;
    app.set_icon_from_buffer(enabled_icon, 128, 128)?;

    app.add_menu_item("Proxy enable", |_| {
        set_proxy(1);
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Proxy disable", |_| {
        set_proxy(0);
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
    let ie_settings = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", KEY_READ).unwrap();
    let proxy: u32 = ie_settings.get_value("ProxyEnable").unwrap();
    proxy
}

fn set_proxy(proxy: u32) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", KEY_SET_VALUE).unwrap();
    ie_settings.set_value("ProxyEnable", &proxy).unwrap();
}