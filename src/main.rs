use image::Pixel;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const REJECT_LEAST: u8 = 0b1111_1100;
const ACCEPT_LEAST: u8 = !REJECT_LEAST;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let image_path = &args[2];

    if command == "encode" {
        let input_path = &args[3];
        encode(input_path, image_path)
    } else if command == "decode" {
        let decoded = decode(image_path)?;
        println!("{}", decoded);
        Ok(())
    } else {
        eprintln!("Unknown command {:?}", command);
        Ok(())
    }
}

fn decode(image: &str) -> Result<String, Box<dyn Error>> {
    eprintln!("Loading image...");
    let img = image::open(image)?.to_rgba();
    let (dim_x, dim_y) = img.dimensions();

    let mut extracted_bytes: Vec<u8> = Vec::new();

    let mut i = 0;
    loop {
        let x = i as u32 % dim_x;
        let y = i as u32 / dim_y;
        let pixel = img.get_pixel(x, y);
        let byte = pixel[0] & ACCEPT_LEAST
            | (pixel[1] & ACCEPT_LEAST) << 2
            | (pixel[2] & ACCEPT_LEAST) << 4
            | (pixel[3] & ACCEPT_LEAST) << 6;

        if byte == 0 {
            break;
        }
        extracted_bytes.push(byte);
        i += 1;
    }

    Ok(String::from_utf8(extracted_bytes).unwrap())
}

fn encode(file: &str, image: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("Loading file...");
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let contents = contents.into_bytes();

    eprintln!("Loading image...");
    let mut img = image::open(image)?.to_rgba();
    let (dim_x, dim_y) = img.dimensions();
    let max_bytes = dim_x * dim_y;

    eprintln!("dimensions: {:?}x{:?}", dim_x, dim_y);
    eprintln!("max bytes to encode: {:?}", max_bytes);
    eprintln!("requested bytes to encode: {:?}", contents.len());

    assert!(contents.len() < max_bytes as usize);

    eprintln!("writing contents");
    for i in 0..contents.len() {
        let x = i as u32 % dim_x;
        let y = i as u32 / dim_y;
        let pixel = img.get_pixel(x, y);
        let r = (pixel[0] & REJECT_LEAST) | (contents[i] & ACCEPT_LEAST);
        let g = (pixel[1] & REJECT_LEAST) | ((contents[i] >> 2) & ACCEPT_LEAST);
        let b = (pixel[2] & REJECT_LEAST) | ((contents[i] >> 4) & ACCEPT_LEAST);
        let a = (pixel[3] & REJECT_LEAST) | ((contents[i] >> 6) & ACCEPT_LEAST);
        img.put_pixel(x, y, Pixel::from_channels(r, g, b, a));
    }

    // Write a NULL byte at the end of the string
    let x = contents.len() as u32 % dim_x;
    let y = contents.len() as u32 / dim_y;
    let pixel = img.get_pixel_mut(x, y);
    let r = pixel[0] & REJECT_LEAST;
    let g = pixel[1] & REJECT_LEAST;
    let b = pixel[2] & REJECT_LEAST;
    let a = pixel[3] & REJECT_LEAST;
    img.put_pixel(x, y, Pixel::from_channels(r, g, b, a));

    eprintln!("Saving...");
    img.save("out.png")?;

    Ok(())
}
