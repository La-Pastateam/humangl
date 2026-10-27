use anyhow::Result;
use winit::keyboard::PhysicalKey;

use crate::app::AppData;

pub enum Signal {
    Nothing,
}

pub fn handle_input(input: PhysicalKey, data: &mut AppData) -> Result<Signal> {
    match input {
        _ => Ok(Signal::Nothing),
    }
}
