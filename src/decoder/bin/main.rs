use std::{
    env::args,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};

use image::DynamicImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a stopwatch for timing purposes.
    let timer: Instant = Instant::now();

    // Initialize variables.
    let mut buffer: Vec<u8> = Vec::new();

    // Collect args for later use.
    let args: Vec<String> = args().collect::<Vec<String>>();

    if args.len() == 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Target file location not provided!",
        )
        .into());
    }

    //let raw_file_location: &String = &args[1];

    // Create the raw file path from args.
    let raw_file_path = Path::new(&args[1]);

    if !raw_file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Target file doesn't exist!",
        )
        .into());
    }

    // Assign the image variable
    let image: DynamicImage = image::open(raw_file_path)?;

    // Generate the new file's filename and path.
    let new_file_filename: String = raw_file_path.to_str().unwrap().to_owned() + ".txt";
    let new_file_path: &Path = Path::new(&new_file_filename);

    // Create the new file.
    let mut new_file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&new_file_filename)?;

    // Decode the image.
    for pixel in image.to_rgba8().pixels() {
        buffer.push(pixel[0]);
        buffer.push(pixel[1]);
        buffer.push(pixel[2]);
        buffer.push(pixel[3]);
    }

    new_file.write_all(&buffer)?;

    // Print stats
    println!(
        "Saved output as {}",
        &new_file_path.file_name().unwrap().to_str().unwrap()
    );
    println!("Done in {}ms", timer.elapsed().as_millis());

    Ok(())
}
