$env:RUSTFLAGS="--emit=asm" 
cargo +nightly build --release -Z build-std --target x86_64-pc-windows-msvc