
# File Compressor

A simple Rust-based command-line tool to compress files using Gzip. This project leverages the `flate2` crate to perform file compression and calculates the compression ratio along with timing the process.

## Features
- Compresses any input file using Gzip.
- Provides output on file sizes and compression ratio.
- Times the compression process and reports the elapsed time.

## Requirements

- Rust (Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed)

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/BurakErguvn/Small-Project-File-Compressor
    ```

2. Navigate to the project directory:

    ```bash
    cd Small-Project-File-Compressor
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

## Usage

Run the tool using the following command structure:

```bash
cargo run --release -- <source_file> <output_file>
```

- `source_file`: The file you want to compress.
- `output_file`: The name of the compressed file to be generated.

### Example

To compress a file called `input.txt` and generate `output.gz`:

```bash
cargo run --release -- input.txt output.gz
```

### Output

The program provides the following output after compressing a file:
- **Source file size** in bytes.
- **Compressed file size** in bytes.
- **Compression ratio** in percentage.
- **Elapsed time** taken for compression.

### Example Output

```
Source file size: 2048 bytes
Compressed file size: 1024 bytes
Compression ratio: 50.00%
Elapsed time: 0.05s
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
