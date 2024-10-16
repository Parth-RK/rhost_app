use std::error::Error;
// use slint::prelude::*;

slint::include_modules!();

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    // ui.set_title("Counter App");
    // ui.set_width(800);
    // ui.set_height(600);
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}