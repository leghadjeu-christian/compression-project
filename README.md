# Compression Tool in Rust and Javascript
A compression CLI tool using both Rust and JavaScript, supporting two compression algorithms: Run-Length Encoding (RLE) and Simplified LZ77.

### Installation

-  **Clone the repository:**
    ```bash
    git clone https://github.com/leghadjeu-christiancompression-project.git
    cd compression-project
    ```

-  **Run the algorithms:**
    * **For Rust:**
        ```bash
        cargo run -- compress|decompress <input file> <outputfile> --rle|lz
        ```
    * **For JS:**
```bash
node index.js compress|decompress <input file> <outputfile> --rle|lz
```

## Using the docker image.
Using the docker images and running the algorithms without necessarily cloning the Repository.
* **For Rust:**

```bash
docker pull ghcr.io/leghadjeu-christian/rust-compressor:latest
```

* **For JS:**

```bash
docker pull ghcr.io/leghadjeu-christian/js-compressor:latest
```
After pulling the images run them with their corresponding arguments as specified above.

```bash
docker run compress|decompress <input file> <outputfile> --rle|lz
```

