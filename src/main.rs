//Project: gptokenizer-cli
//Description: A simple cli GPT tokenizer 
//Author website: https://tragdate.ninja
//Author: @tragDate on github tiktok and youtube
//git repo: https://github.com/tragDate/gptokenizer-cli
//License: GPL-3.0

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Read};
use structopt::StructOpt;
use tiktoken_rs::cl100k_base;
use tiktoken_rs::r50k_base;

#[derive(StructOpt, Debug)]
#[structopt(name = "gptokenizer-cli", about = "A simple cli GPT tokenizer")]
struct Opt {
    #[structopt(short = "i", long, default_value = "")]
    input_file: String,
    #[structopt(short = "o", long, default_value = "")]
    output_file: String,
    #[structopt(short = "m", long, default_value = "gpt-4")]
    model: String,
}

fn main() {
    let opt = Opt::from_args();
    let input = read_input(&opt.input_file);
    let num_tokens = count_tokens(&input, &opt.model);

    let output = format_output(num_tokens);

    if opt.output_file.is_empty() {
        println!("{}", output);
    } else {
        write_output(&opt.output_file, &output);
    }
}

fn read_input(input_file: &str) -> String {
    if input_file.is_empty() {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut text = String::new();
        handle.read_to_string(&mut text).unwrap();
        text.trim().to_owned()
    } else {
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
        lines.join("\n")
    }
}

fn count_tokens(text: &str, model: &str) -> usize {
    let bpe = if model == "gpt-4" || model == "gpt-3.5-turbo" {
        cl100k_base().unwrap()
    } else if model == "davinci" {
        r50k_base().unwrap()
    } else {
        panic!("Unsupported model: {}", model);
    };

    bpe.encode_with_special_tokens(text).len()
}

fn format_output(num_tokens: usize) -> String {
    format!("{}", num_tokens)
}

fn write_output(output_file: &str, output: &str) {
    let mut file = File::create(output_file).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
