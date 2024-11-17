
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

slint::include_modules!();

#[derive(Default, Clone)]
struct CalculatorState {
    first_number: Option<f64>,
    operation: Option<char>,
    new_number: bool,
}

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let calc_state = Rc::new(RefCell::new(CalculatorState::default()));
    
    // Handle number and operator buttons
    ui.on_button_pressed({
        let ui_handle = ui.as_weak();
        let calc_state = calc_state.clone();
        move |value: slint::SharedString| {
            let ui = ui_handle.unwrap();
            let mut state = calc_state.borrow_mut();

            let value_str = value.as_str();
            if value_str.chars().all(|c| c.is_digit(10) || c == '.') {
                // Handle numbers
                let current_display = if state.new_number { String::new() } else { ui.get_display().to_string() };
                let updated_display = format!("{}{}", current_display, value_str);
                ui.set_display(updated_display.into());
                state.new_number = false;
            } else if let Some(op) = value_str.chars().next() {
                // Handle operators
                if let Ok(number) = ui.get_display().parse() {
                    state.first_number = Some(number);
                    state.operation = Some(op);
                    state.new_number = true;
                }
            }
        }
    });
    
    // Handle clear button
    ui.on_clear_pressed({
        let ui_handle = ui.as_weak();
        let calc_state = calc_state.clone();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_display("0".into());
            *calc_state.borrow_mut() = CalculatorState::default();
        }
    });
    
    // Handle equals button
    ui.on_equals_pressed({
        let ui_handle = ui.as_weak();
        let calc_state = calc_state.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut state = calc_state.borrow_mut();
            
            if let Some(op) = state.operation {
                if let Ok(second_number) = ui.get_display().parse::<f64>() {
                    if let Some(first_number) = state.first_number {
                        let result = match op {
                            '+' => first_number + second_number,
                            '-' => first_number - second_number,
                            '*' => first_number * second_number,
                            '/' => {
                                if second_number.abs() > std::f64::EPSILON {
                                    first_number / second_number
                                } else {
                                    f64::NAN
                                }
                            },
                            _ => f64::NAN,
                        };
                        ui.set_display(result.to_string().into());
                    }
                }
            }
            *state = CalculatorState::default();
        }
    });

    ui.run()?;
    Ok(())
}
