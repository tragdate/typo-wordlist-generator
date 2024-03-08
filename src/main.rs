use clap::Parser;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    input: String,
    #[clap(value_parser)]
    output: String,
    #[clap(short, long, value_parser, default_value = "")]
    prefix: String,
    #[clap(short, long, value_parser, default_value = "")]
    suffix: String,
    #[clap(short, long, value_parser, default_value = "1")]
    typo_depth: String,
}

fn create_typo_map() -> HashMap<char, Vec<char>> {
    let mut typo_map: HashMap<char, Vec<char>> = HashMap::new();
    typo_map.insert('q', vec!['w', 'a']);
    typo_map.insert('w', vec!['q', 'e', 'a', 's', 'd']);
    typo_map.insert('e', vec!['w', 'r', 's', 'd', 'f']);
    typo_map.insert('r', vec!['e', 't', 'd', 'f', 'g']);
    typo_map.insert('t', vec!['r', 'y', 'f', 'g', 'h']);
    typo_map.insert('y', vec!['t', 'u', 'g', 'h', 'j']);
    typo_map.insert('u', vec!['y', 'i', 'h', 'j', 'k']);
    typo_map.insert('i', vec!['u', 'o', 'j', 'k', 'l']);
    typo_map.insert('o', vec!['i', 'p', 'k', 'l']);
    typo_map.insert('p', vec!['o', 'l']);
    typo_map.insert('a', vec!['q', 'w', 's', 'z', 'x']);
    typo_map.insert('s', vec!['q', 'w', 'e', 'a', 'd', 'z', 'x', 'c']);
    typo_map.insert('d', vec!['w', 'e', 'r', 's', 'f', 'x', 'c', 'v']);
    typo_map.insert('f', vec!['e', 'r', 't', 'd', 'g', 'c', 'v', 'b']);
    typo_map.insert('g', vec!['r', 't', 'y', 'f', 'h', 'v', 'b', 'n']);
    typo_map.insert('h', vec!['t', 'y', 'u', 'g', 'j', 'b', 'n', 'm']);
    typo_map.insert('j', vec!['y', 'u', 'i', 'h', 'k', 'n', 'm']);
    typo_map.insert('k', vec!['u', 'i', 'o', 'j', 'l', 'm']);
    typo_map.insert('l', vec!['i', 'o', 'p', 'k']);
    typo_map.insert('z', vec!['a', 's', 'x']);
    typo_map.insert('x', vec!['z', 'a', 's', 'd', 'c']);
    typo_map.insert('c', vec!['x', 's', 'd', 'f', 'v']);
    typo_map.insert('v', vec!['c', 'd', 'f', 'g', 'b']);
    typo_map.insert('b', vec!['v', 'f', 'g', 'h', 'n']);
    typo_map.insert('n', vec!['b', 'g', 'h', 'j', 'm']);
    typo_map.insert('m', vec!['n', 'h', 'j', 'k']); // Assume this function returns a complete typo_map.
    typo_map
}

fn generate_typos_recursive(
    word: &str,
    typo_map: &HashMap<char, Vec<char>>,
    depth: usize,
    start: usize,
    seen: &mut HashSet<String>,
) {
    if depth == 0 {
        seen.insert(word.to_string());
        return;
    }

    let chars: Vec<char> = word.chars().collect();
    for i in start..chars.len() {
        if let Some(replacements) = typo_map.get(&chars[i]) {
            for &replacement in replacements {
                let mut typo = word.to_string();
                typo.replace_range(i..=i, &replacement.to_string());
                if !seen.contains(&typo) {
                    generate_typos_recursive(&typo, typo_map, depth - 1, i + 1, seen);
                }
            }
        }
    }
}

fn process_words(
    input_path: &Path,
    output_path: &Path,
    typo_map: &HashMap<char, Vec<char>>,
    typo_depth: usize,
) -> io::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    let writer = Arc::new(Mutex::new(BufWriter::new(File::create(output_path)?)));
    lines.par_chunks(12).for_each(|chunk_of_lines| {
        let mut seen = HashSet::new();
        chunk_of_lines.iter().for_each(|line| {
            generate_typos_recursive(line, typo_map, typo_depth, 0, &mut seen);
        });
        if let Ok(mut writer) = writer.lock() {
            for typo in seen {
                writeln!(writer, "{}", typo).expect("Failed to write to output file");
            }
        }
    });
    if let Ok(mut writer) = writer.lock() {
        writer.flush()?;
    }
    Ok(())
}

fn main() {
    println!("Number of threads: {}", rayon::current_num_threads());
    let args = Args::parse();
    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);
    let typo_map = create_typo_map();
    let typo_depth = match args.typo_depth.to_lowercase().as_str() {
        "full" => usize::MAX,
        _ => args.typo_depth.parse::<usize>().unwrap_or(1),
    };
    if let Err(e) = process_words(&input_path, &output_path, &typo_map, typo_depth) {
        eprintln!("Error processing words: {}", e);
        std::process::exit(1);
    }
}
