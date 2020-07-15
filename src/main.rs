#![windows_subsystem = "windows"]
mod proxy;
use std::{env, fs, path::PathBuf, thread, time::Duration};
use win32_notification::NotificationBuilder;

fn main() {
    // Creates temp file to store user requested proxy status (as we can't share data between threads with the systray crate...)
    // At start : user has not changed anything, so user status matches current system status.
    write_user_status(proxy::get());
    status_notification();
    // Checking system proxy every second (in case of a nasty system policy sets it...)
    proxy::check(1);

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

    app.add_menu_item("Proxy enable", |_| {
        write_user_status(1);
        notification("enabled");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Proxy disable", |_| {
        write_user_status(0);
        notification("disabled");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Display proxy status", |_| {
        status_notification();
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

fn write_user_status(status: u32) {
    let mut d = PathBuf::new();
    d.push(env::temp_dir());
    d.push("user_status.txt");
    fs::write(d, status.to_string()).unwrap();
}

fn notification(message: &'static str) {
    thread::spawn(move || {
        let notification = NotificationBuilder::new()
        .title_text("System proxy")
        .info_text(message)
        .build()
        .expect("Could not create notification");

    notification.show().expect("Failed to show notification");
    thread::sleep(Duration::from_secs(3));
    notification
        .delete()
        .expect("Failed to delete notification");
    });
}

fn status_notification() {
    match proxy::get() {
        0 => notification("Proxy currently disabled"),
        _ => notification("Proxy currently enabled")
    };
}