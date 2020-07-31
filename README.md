# lsb-steg

Least-significant-bit steganography in Rust

## Usage
```
# Produces out.png
cargo run encode Image.png Text.txt

# Prints decoded text to STDOUT
cargo run decode out.png > decoded.txt
```
