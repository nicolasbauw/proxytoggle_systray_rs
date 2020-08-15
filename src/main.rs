#![windows_subsystem = "windows"]
mod proxy;
use std::{process, sync::Arc, sync::Mutex};

fn main() {
    // Creates the systray and menus
    if create_systray().is_err() {
        process::exit(1);
    };
}

fn create_systray() -> Result<(), systray::Error> {
    let enabled_icon = include_bytes!("../assets/check-mark-16.ico");
    let disabled_icon = include_bytes!("../assets/x-mark-16.ico");
    let unknown_icon = include_bytes!("../assets/question-mark-16.ico");
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
    match proxy::get() {
        Ok(1) => app.set_icon_from_buffer(enabled_icon, 128, 128)?,
        Ok(0) => app.set_icon_from_buffer(disabled_icon, 128, 128)?,
        Ok(_) => app.set_icon_from_buffer(unknown_icon, 128, 128)?,
        Err(_) => app.set_icon_from_buffer(unknown_icon, 128, 128)?
    }

    let us = Arc::clone(&user_status);
    app.add_menu_item("Proxy enable", move |window| {
        let mut us = match us.lock(){
            Ok(u) => u,
            Err(_) => return Err(systray::Error::UnknownError)
        };
        *us = 1;
        window.set_icon_from_buffer(enabled_icon, 128, 128)?;
        Ok::<_, systray::Error>(())
    })?;

    let us = Arc::clone(&user_status);
    app.add_menu_item("Proxy disable", move |window| {
        let mut us = match us.lock(){
            Ok(u) => u,
            Err(_) => return Err(systray::Error::UnknownError)
        };
        *us = 0;
        window.set_icon_from_buffer(disabled_icon, 128, 128)?;
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
