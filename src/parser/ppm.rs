use anyhow::{Result, anyhow};
use std::io::{BufRead, BufReader, Cursor, Read};

// ── PPM Image ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PPMImage {
    pub data: Vec<u8>,
    pub size: u64,
    pub width: u32,
    pub height: u32,
}

fn next_valid_line(reader: &mut BufReader<Cursor<Vec<u8>>>) -> String {
    let mut stop = false;
    let mut curr_line: String = String::new();

    while !stop {
        reader.read_line(&mut curr_line).unwrap();

        let sliced = curr_line.as_str();
        if sliced.trim().starts_with("#") {
            curr_line.clear();
            continue;
        } else {
            stop = true;
        }
    }
    curr_line
}

fn allocate_pixel(mut reader: BufReader<Cursor<Vec<u8>>>, data: &mut Vec<u8>) -> Result<()> {
    let mut rgb_data = Vec::new();
    reader.read_to_end(&mut rgb_data)?;

    for chunk in rgb_data.chunks(3) {
        data.push(chunk[0]);
        data.push(chunk[1]);
        data.push(chunk[2]);
        data.push(255);
    }

    Ok(())
}

pub fn parse_ppm(image_data: Vec<u8>) -> Result<PPMImage, anyhow::Error> {
    let cursor = Cursor::new(image_data);
    let mut reader = BufReader::new(cursor);

    let line = next_valid_line(&mut reader);
    if line.trim() != "P6" {
        return Err(anyhow!("Wrong ppm format"));
    }

    let line = next_valid_line(&mut reader);
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(anyhow!("Wrong size, width and height must be specified"));
    };
    let width = parts[0].parse::<u32>()?;
    let height = parts[1].parse::<u32>()?;
    let size = (width * height * 4) as u64;

    let line = next_valid_line(&mut reader);
    let maxval = line.trim().parse::<u16>()?;
    if maxval != 255 {
        return Err(anyhow!("Only maxval=255 is supported"));
    }

    let mut data = Vec::with_capacity((width * height * 4) as usize);
    allocate_pixel(reader, &mut data)?;

    Ok(PPMImage {
        data,
        size,
        width,
        height,
    })
}
