// ui.rs
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
            let current_display = ui.get_display().to_string();
            let value_str = value.to_string();
            
            let mut state = calc_state.borrow_mut();
            if state.new_number {
                ui.set_display(value_str.clone().into());
                state.new_number = false;
            } else {
                if current_display == "0" && value_str != "." {
                    ui.set_display(value_str.clone().into());
                } else {
                    ui.set_display(format!("{}{}", current_display, value_str).into());
                }
            }
            
            // If it's an operator
            if ['+', '-', '*', '/'].contains(&value_str.chars().next().unwrap()) {
                state.first_number = Some(current_display.parse().unwrap_or(0.0));
                state.operation = Some(value_str.chars().next().unwrap());
                state.new_number = true;
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
            let current_display = ui.get_display().to_string();
            
            let mut state = calc_state.borrow_mut();
            if let (Some(first), Some(op)) = (state.first_number, state.operation) {
                let second: f64 = current_display.parse().unwrap_or(0.0);
                let result = match op {
                    '+' => first + second,
                    '-' => first - second,
                    '*' => first * second,
                    '/' => if second != 0.0 { first / second } else { f64::NAN },
                    _ => f64::NAN,
                };
                
                ui.set_display(format!("{}", result).into());
                state.new_number = true;
                state.first_number = None;
                state.operation = None;
            }
        }
    });

    ui.run()?;
    Ok(())
}