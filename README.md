# bmper
A program that can convert theoretically any file to an image written in Rust

This program can convert any<sup>not confirmed, breakage may occur</sup> file into an image and back.

## Usage
Provide a path to a file or drag-and-drop it on the binary to encode/decode it (depends on the binary used).

## Notes
Because the resulting images are always square, in some cases decoded files may become corrupt as decoding them may append null bytes.
