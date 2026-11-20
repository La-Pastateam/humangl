use anyhow::Result;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppData;

pub enum Signal {
    Exit,
    Nothing,
    ToggleUi,
}

pub fn handle_input(input: PhysicalKey, data: &mut AppData) -> Result<Signal> {
    match input {
        PhysicalKey::Code(KeyCode::KeyI) => Ok(Signal::ToggleUi),
        PhysicalKey::Code(KeyCode::Escape) => Ok(Signal::Exit),
        _ => Ok(Signal::Nothing),
    }
}
