# Compression Tool in Rust and Javascript
A compression CLI tool using both Rust and JavaScript, supporting two compression algorithms: Run-Length Encoding (RLE) and Simplified LZ77.

# Compress with RLE
cargo run -- compress input.txt output.txt --rle

# Decompress with RLE
cargo run -- decompress output.txt restored.txt --rle

node compression.js compress input.txt compressed.dat --rle
