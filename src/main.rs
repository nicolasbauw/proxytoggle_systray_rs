#![windows_subsystem = "windows"]
mod proxy;
use std::{process, sync::Arc, sync::Mutex, thread, time::Duration};
use win32_notification::NotificationBuilder;

fn main() {
    // At start : displays current proxy status
    status_notification();

    // Creates the systray and menus
    if create_systray().is_err() {
        process::exit(1);
    };
}

fn create_systray() -> Result<(), systray::Error> {
    let enabled_icon = include_bytes!("../assets/checkmark.ico");
    // To share user proxy status between threads and closures
    let user_status = Arc::new(Mutex::new(proxy::get().unwrap_or(0)));

    // Checking system proxy every second (in case of a nasty system policy sets it...)
    let us = Arc::clone(&user_status);
    proxy::check(1, us);

    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => return Err(systray::Error::UnknownError),
    };
    app.set_tooltip(&"Proxy toggle".to_string())?;
    app.set_icon_from_buffer(enabled_icon, 128, 128)?;

    let us = Arc::clone(&user_status);
    app.add_menu_item("Proxy enable", move |_| {
        let mut us = match us.lock(){
            Ok(u) => u,
            Err(_) => return Err(systray::Error::UnknownError)
        };
        *us = 1;
        notification("enabled");
        Ok::<_, systray::Error>(())
    })?;

    let us = Arc::clone(&user_status);
    app.add_menu_item("Proxy disable", move |_| {
        let mut us = match us.lock(){
            Ok(u) => u,
            Err(_) => return Err(systray::Error::UnknownError)
        };
        *us = 0;
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

fn notification(message: &'static str) {
    thread::spawn(move || {
        let notification = NotificationBuilder::new()
            .title_text("System proxy")
            .info_text(message)
            .build()
            .unwrap();

        notification.show().unwrap();
        thread::sleep(Duration::from_secs(3));
        notification
            .delete()
            .unwrap();
    });
}

fn status_notification() {
    if let Ok(0) = proxy::get() {
        notification("Proxy currently disabled")
    } else {
        notification("Proxy currently enabled")
    }
}
