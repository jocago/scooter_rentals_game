use crate::cli::ProcessingError::IoReadError;
use crate::cli::ProcessingError::ConvertStringToIntError;
use std::io;
use crate::USE_ANY_KEY_LABEL;

#[derive(Debug)]
pub enum ProcessingError {
    ConvertStringToIntError,
    IoReadError,
}

pub fn get_input_u32() -> Result<u32, ProcessingError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<u32>() {
                Ok(val) => Ok(val),
                Err(_) => Err(ConvertStringToIntError),
            }
        },
        Err(_) => Err(IoReadError),
    }
}

pub fn get_input_f32() -> Result<f32, ProcessingError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<f32>() {
                Ok(val) => Ok(val),
                Err(_) => Err(ConvertStringToIntError),
            }
        },
        Err(_) => Err(IoReadError),
    }
}

pub fn get_input_string() -> Result<String, ProcessingError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err(IoReadError),
    }
}

pub fn get_input_nothing() {
    let _ = get_input_string();
}

pub fn output(line: String) {
    println!("{line}");
}


pub fn say_any_key() {
    if USE_ANY_KEY_LABEL {
        output("Press return to continue.".to_string());
    }
}