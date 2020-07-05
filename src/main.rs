//#![windows_subsystem = "windows"]

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_tooltip(&"Proxy Toggle".to_string())?;
    app.set_icon_from_file("assets\\skull-icon.ico")?;

    app.add_menu_item("Proxy state", |_| {
        println!("Enabled|Disabled");
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
