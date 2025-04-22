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
        ```
node index.js compress|decompress <input file> <outputfile> --rle|lz
        ```

## Using the docker image.
* **For Rust:**

```bash
docker pull ghcr.io/leghadjeu-christian/rust-compressor:latest
```

* **For JS:**

```bash
docker pull ghcr.io/leghadjeu-christian/js-compressor:latest
```

