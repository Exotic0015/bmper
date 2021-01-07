use std::{
    env::args,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};

use image::DynamicImage;

fn main() {
    // Initialize a stopwatch for timing purposes.
    let timer: Instant = Instant::now();

    // Initialize variables.
    let raw_file_location: &String;
    let raw_file_path: &Path;
    let new_file_filename: String;
    let new_file_path: &Path;
    let image: DynamicImage;

    let mut buffer: Vec<u8> = Vec::new();

    // Collect args for later use.
    let args: Vec<String> = args().collect::<Vec<String>>();

    if args.len() == 1 {
        eprintln!("Target file location not provided!");
        quit::with_code(3);
    }

    // Create the raw file path from args.
    raw_file_location = &args[1];
    raw_file_path = Path::new(&raw_file_location);

    if !raw_file_path.exists() {
        eprintln!("Target file doesn't exist!");
        quit::with_code(4);
    }

    // Assign the image variable
    image = image::open(raw_file_location).unwrap();

    // Generate the new file's filename and path.
    new_file_filename = raw_file_path.to_str().unwrap().to_owned() + ".txt";
    new_file_path = Path::new(&new_file_filename);

    // Create the new file.
    let mut new_file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&new_file_filename)
        .unwrap();

    // Decode the image.
    for pixel in image.to_rgb8().pixels() {
        //let _ = new_file.write_all(&[pixel[0]]);
        //let _ = new_file.write_all(&[pixel[1]]);
        //let _ = new_file.write_all(&[pixel[2]]);
        buffer.push(pixel[0]);
        buffer.push(pixel[1]);
        buffer.push(pixel[2]);
    }

    new_file.write_all(&buffer).unwrap();

    // Print stats
    println!(
        "Saved output as {}",
        &new_file_path.file_name().unwrap().to_str().unwrap()
    );
    println!("Done in {}{}", timer.elapsed().as_millis(), "ms");
}
