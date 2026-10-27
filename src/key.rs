use anyhow::Result;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

use crate::app::AppData;

pub enum Signal {
    Nothing,
    ReloadModel,
}

fn file_manager(data: &mut AppData) -> Result<Signal> {
    let path = std::env::current_dir()?.join("resources/model");

    let file = rfd::FileDialog::new()
        .add_filter("model", &["obj"])
        .set_directory(&path)
        .pick_file();

    match file {
        Some(path) => {
            data.image_path = path;
            Ok(Signal::ReloadModel)
        }
        None => Ok(Signal::Nothing),
    }
}

pub fn handle_input(input: PhysicalKey, data: &mut AppData) -> Result<Signal> {
    match input {
        PhysicalKey::Code(KeyCode::Enter) => file_manager(data),
        _ => Ok(Signal::Nothing),
    }
}
