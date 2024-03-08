# typo-wordlist-generator

A command-line utility written in Rust, designed to generate wordlists based on common typing errors. Given an input wordlist, `typo-wordlist-generator` produces an output file containing variations of the input words that simulate typing mistakes according to a predefined typo map.

## 🚀 Features

- 🚄 **Concurrency**: Utilizes Rayon for parallel processing, improving the execution speed on multi-core systems.
- 🔧 **Customizable Typo Depth**: Allows specifying the depth of typos to generate, controlling the thoroughness of the typo simulation.
- 🔛 **Prefix and Suffix Addition**: Offers options to append prefixes or suffixes to each term in the generated wordlist, enhancing flexibility for different use cases.
- 🛑 **Error Handling**: Gracefully handles I/O and parsing errors, ensuring robustness during operation.


## ⌨️ Typo Map 

| Key | Typos |
| --- | ----- |
| q   | w, a  |
| w   | q, e, a, s, d |
| e   | w, r, s, d, f |
| r   | e, t, d, f, g |
| t   | r, y, f, g, h |
| y   | t, u, g, h, j |
| u   | y, i, h, j, k |
| i   | u, o, j, k, l |
| o   | i, p, k, l |
| p   | o, l |
| a   | q, w, s, z, x |
| s   | q, w, e, a, d, z, x, c |
| d   | w, e, r, s, f, x, c, v |
| f   | e, r, t, d, g, c, v, b |
| g   | r, t, y, f, h, v, b, n |
| h   | t, y, u, g, j, b, n, m |
| j   | y, u, i, h, k, n, m |
| k   | u, i, o, j, l, m |
| l   | i, o, p, k | 
| z   | a, s, x |
| x   | z, a, s, d, c |
| c   | x, s, d, f, v |
| v   | c, d, f, g, b |
| b   | v, f, g, h, n |
| n   | b, g, h, j, m |
| m   | n, h, j, k |


##  📝 Requirements

- 🦀 Rust 2021 Edition or newer.
- 📦 Cargo package manager.
- Dependencies:
  - `clap` for parsing command-line arguments.
  - `rayon` for parallel data processing.


## 💾 Installation

First, ensure you have Rust and Cargo installed on your system. Then, follow these steps to build the program:

```bash
# Clone this repository (simulate this step as the source code is provided)
git clone https://github.com/tragadte/typo-wordlist-generator

# Change to the project directory
cd typo-wordlist-generator

# Build the project using Cargo
cargo build --release
```


## 📍 Usage

After building the project, you can run it using Cargo or directly from the executable file in the `target/release` directory. Here are some examples of how to use `typo-wordlist-generator`:

```bash
# Basic usage with required arguments
typo-wordlist-generator -i input.txt -o output.txt

# Specifying typo depth
typo-wordlist-generator -i input.txt -o output.txt -d 2

# Adding a prefix and suffix to each word
typo-wordlist-generator -i input.txt -o output.txt --prefix "pre" --suffix "suf"

# Showing help information
typo-wordlist-generator --help
```

### ⚙ Options

- `-i, --input <INPUT>`: Path to the input file containing the original wordlist.
- `-o, --output <OUTPUT>`: Path to the output file where the typo-inclusive wordlist will be saved.
- `-p, --prefix <PREFIX>`: Prefix to append to each word in the generated list.
- `-s, --suffix <SUFFIX>`: Suffix to append to each word in the generated list.
- `-d, --typo_depth <TYPO_DEPTH>`: Depth of typo generation. Can be a number or 'full' for maximum depth.

## 🥷 Author
[Trag Date](https://tragdate.ninja)

## 📜 License

This project is licensed under the BSD 3-Clause “New” or “Revised” License.