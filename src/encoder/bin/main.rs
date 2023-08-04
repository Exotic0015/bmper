use std::io::BufReader;
use std::{env::args, fs::File, io::Read, path::Path, time::Instant};

use image::Rgba;
use rand::{distributions::Alphanumeric, Rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a stopwatch for timing purposes.
    let timer: Instant = Instant::now();

    // Initialize variables.
    let mut buffer: Vec<u8> = Vec::new();
    let size: u32;
    let height: usize;

    let mut array_x: usize = 0;
    let mut array_y: usize = 0;

    let mut image_x: u32 = 0;
    let mut image_y: u32 = 0;

    // Collect args for later use.
    let args: Vec<String> = args().collect::<Vec<String>>();
    if args.len() == 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Target file location not provided!",
        )
        .into());
    }

    // Create the raw file path from args.
    let raw_file_path: &Path = Path::new(&args[1]);

    // Check if the file exists, if it does assign it to a variable.
    if !raw_file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Target file doesn't exist!",
        )
        .into());
    }

    // Fill the buffer with the raw file's buffer.
    let mut raw_file = BufReader::new(File::open(raw_file_path)?);
    let _ = raw_file.read_to_end(&mut buffer)?;

    let buffer_length: usize = buffer.len();

    // Calculate size and height.
    if buffer_length <= 4 {
        size = 1;
        height = 1;
    } else {
        size = ((((buffer_length / 4) as f64).sqrt() as u32 + 1) as f64).ceil() as u32;

        if buffer_length % 4 != 0 {
            height = ((buffer_length / 4 + 1) as f64).ceil() as usize;
        } else {
            height = ((buffer_length / 4) as f64).ceil() as usize;
        }
    }

    // Assign the image variable.
    let mut image = image::RgbaImage::new(size, size);

    // Calculate the pixel array.
    let mut array: Vec<Vec<u8>> = vec![vec![0; 4]; height];

    for character in buffer {
        array[array_y][array_x] = character;

        if array_x == 3 {
            array_x = 0;
            array_y += 1;
        } else {
            array_x += 1;
        }
    }

    // Draw pixels on the image.
    for pixel in array {
        image.put_pixel(
            image_x,
            image_y,
            Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]),
        );

        if image_x == size - 1 {
            image_x = 0;
            if image_y != size - 1 {
                image_y += 1;
            }
        } else {
            image_x += 1;
        }
    }

    // Generate a random filename for newly generated images.
    let random_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();

    // Save the image.
    let png_filename = format!(
        "{}.{}.png",
        random_name,
        raw_file_path.extension().unwrap().to_str().unwrap()
    );

    image.save(png_filename)?;

    // Print stats.
    println!(
        "Size: {}x{}\nCharacter count: {}\nSaved as {}.png",
        size, size, buffer_length, random_name
    );

    // Print execution time.
    println!("Done in {}ms", timer.elapsed().as_millis());

    Ok(())
}
