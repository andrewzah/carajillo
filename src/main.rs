use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::result::Result;
use std::io::Write;

use ansi_term::Colour::{
    Cyan, Green, Red, Yellow
};
use argh::{self, FromArgs};
use tabwriter::TabWriter;

mod errors;
mod models;
mod utils;

use crate::errors::CarajilloError;
use crate::models::{OpenDictRequest, OpenDictResponse};

// https://opendict.korean.go.kr/api/search?certkey_no=2727&key=<key>&target_type=search&part=word&q=%EC%88%98%EB%A9%B4&sort=dict&start=1&num=10
static API_BASE_URL: &str = "https://opendict.korean.go.kr/api/";

/// application args
#[derive(FromArgs)]
struct Args {
    /// input file path: plaintext file with words, separated by newlines
    #[argh(positional, short = 'f')]
    input_file: PathBuf,

    /// cache file db path
    #[argh(option, short = 'c', default = "PathBuf::from(\"cache.sql\")")]
    cache_file: PathBuf,

    /// output csv file path
    #[argh(option, short = 'o', default = "PathBuf::from(\"out.csv\")")]
    out_file: PathBuf,
}

fn main() -> Result<(), CarajilloError> {
    let args: Args = argh::from_env();

    let kdict_key = env::var("OPEN_KDICT_KEY")?;

    let input_file = File::open(&args.input_file)?;
    let buf = BufReader::new(input_file);

    let words: Vec<String> = buf.lines()
        .filter_map(Result::ok)
        .collect();

    for word in words {
        process_word(&word, &kdict_key)?;
    }

    // let dict_results: Vec<()> = words
    //     .into_iter()
    //     .filter_map(|w| process_word(&w, &kdict_key).ok())
    //     .collect();

    Ok(())
}

// 1. make request to 
// 2. 
fn process_word(word: &str, key: &str) -> Result<(), CarajilloError> {
    let req_conf = OpenDictRequest::new(key, word);
    let url = req_conf.build_url();

    // let body: String = ureq::post(&url)
    //     .call()?
    //     .into_string()?;
    let body = std::fs::read_to_string("./tests/나무.xml")?;
    let parsed = quick_xml::de::from_str::<OpenDictResponse>(&body)?;
    let senses = parsed.senses();

    let mut sense_str = format!("Definitions for {}:\n", &word);

    for (idx, sense) in senses.iter().enumerate() {
        let pos = sense.pos.clone().unwrap_or(String::new());

        let formatted = format!("{}\t{}\t{}\t{}\n", idx,
            Red.paint(&sense.origin), Cyan.paint(pos), sense.definition);
        sense_str.push_str(&formatted);
    }

    let mut tw = TabWriter::new(io::stdout());
    writeln!(&mut tw, "{}", &sense_str)?;
    tw.flush()?;

    let mut parsed_input = vec![];
    loop {
        let user_input = utils::get_user_input()?;
        parsed_input = utils::parse_user_input(user_input);
        if parsed_input.len() > 0 {
            break;
        }
        println!("No valid inputs received.");
    }

    let mut selected_senses = vec![];
    for selection in parsed_input {
        if let Some(sense) = senses.iter().nth(selection as usize) {
            selected_senses.push(sense.clone());
        } else {
            println!("No definition for {} found. Ignoring!", selection);
        }
    }

    writeln!(&mut tw, "Selected definitions for {}", &word);
    for sense in selected_senses {
        let formatted_str = format!("{}\t{}", Red.paint(sense.origin),
            sense.definition);
        writeln!(&mut tw, "{}", &formatted_str)?;
    }
    tw.flush();

    Ok(())
}
