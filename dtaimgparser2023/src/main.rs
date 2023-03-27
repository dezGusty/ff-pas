use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use image::{ImageBuffer, Rgb, RgbImage};

mod cli;

fn main() {
    let cli = cli::parse_cli();

    let palette_file_name = match cli.palette.as_deref() {
        Some(palette) => palette.to_string_lossy().to_string(),
        None => "./error.pal".to_string(),
    };

    let raw_image_file = match cli.rawimg.as_deref() {
        Some(rawimg) => rawimg.to_string_lossy().to_string(),
        None => "./schedulite.json".to_string(),
    };

    let output_image_file = match cli.output.as_deref() {
        Some(output) => output.to_string_lossy().to_string(),
        None => "./schedulite.json".to_string(),
    };

    let width = cli.width;
    let height = cli.height;

    println!("* palette file name {}", &palette_file_name);
    println!("* raw image file name {}", &raw_image_file);
    println!("* output image file name {}", &output_image_file);
    println!("* size {} x {}", width, height);

    let palette = read_palette(&palette_file_name).expect("Failed to read palette file");
    let dta_data = read_dta(&raw_image_file).expect("Failed to read dta file");

    let output_file = &output_image_file;

    let img = dta_to_image(&palette, &dta_data, width, height);
    img.save(output_file).expect("Failed to save image");
}

fn read_palette(filename: &str) -> Result<Vec<Rgb<u8>>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut colors = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i >= 3 {
            let rgb: Vec<u8> = line
                .split_whitespace()
                .map(|s| u8::from_str(s).unwrap())
                .collect();
            if rgb.len() >= 3 {
                colors.push(Rgb([rgb[0], rgb[1], rgb[2]]));
            }
        }
    }

    Ok(colors)
}

fn read_dta(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn dta_to_image(palette: &[Rgb<u8>], dta_data: &[u8], width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);

    for (i, &byte) in dta_data.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;

        if y < height {
            let color_index = byte as usize % palette.len();
            let pixel = palette[color_index];
            img.put_pixel(x, y, pixel);
        } else {
            break;
        }
    }

    img
}
