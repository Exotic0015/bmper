use std::{env::args, fs::File, io::Read, path::Path, time::Instant};

use image::Rgb;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
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
        eprintln!("Target file location not provided!");
        quit::with_code(3);
    }

    // Create the raw file path from args.
    let raw_file_path: &Path = Path::new(&args[1]);

    // Check if the file exists, if it does assign it to a variable.
    let mut raw_file: File = if raw_file_path.exists() {
        File::open(raw_file_path).unwrap()
    } else {
        eprintln!("Target file doesn't exist!");
        quit::with_code(4);
    };

    // Fill the buffer with the raw file's buffer.
    let _ = raw_file.read_to_end(&mut buffer).unwrap();
    let buffer_length: usize = buffer.len();

    // Calculate size and height.
    if buffer_length < 4 {
        size = 1;
        height = 1;
    } else {
        size = ((((buffer_length / 3) as f64).sqrt() as u32 + 1) as f64).ceil() as u32;

        if buffer_length % 3 != 0 {
            height = ((buffer_length / 3 + 1) as f64).ceil() as usize;
        } else {
            height = ((buffer_length / 3) as f64).ceil() as usize;
        }
    }

    // Assign the image variable.
    let mut image = image::RgbImage::new(size, size);

    // Calculate the pixel array.
    let mut array: Vec<Vec<u8>> = vec![vec![0; 3]; height];

    for character in buffer {
        array[array_y][array_x] = character;

        if array_x == 2 {
            array_x = 0;
            array_y += 1;
        } else {
            array_x += 1;
        }
    }

    // Draw pixels on the image.
    for pixel in array {
        image.put_pixel(image_x, image_y, Rgb([pixel[0], pixel[1], pixel[2]]));

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
    let rname: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();

    // Save the image.
    let png_filename =
        format! {"{}.{}.png", rname, raw_file_path.extension().unwrap().to_str().unwrap()};

    image.save(png_filename).unwrap();

    // Print stats.
    println!(
        "Size: {}x{}\nCharacter count: {}\nSaved as {}.png",
        size, size, buffer_length, rname
    );

    // Print execution time.
    println!("Done in {}ms", timer.elapsed().as_millis());
}
