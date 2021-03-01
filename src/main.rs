use jen::generator::Generator;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use clap::{Arg, App, ArgMatches};
use serde_json::{Value};
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() {
    let matches = get_matches();
    
    let num_rows = get_rows(&matches);
    let template_name = get_template(&matches);
    let output_name = get_output(&matches);

    let output_path = Path::new(&output_name);
    let output_file = File::create(&output_path).expect("file could be created");

    write_output(generate_data(num_rows as usize, &template_name), output_file);
}

fn write_output(output_and_headers: (Vec<String>, Vec<String>), f: std::fs::File) {
    println!("Started writing output");

    let mut bw = BufWriter::new(f);
    
    let output_data = output_and_headers.0;
    let headers = output_and_headers.1;

    let header_row = headers.join(",") + "\n";

    bw.write(header_row.as_bytes()).unwrap();

    for gen in output_data {
        bw.write(gen.as_bytes()).unwrap();
    }
}

fn generate_data(number_of_rows: usize, template_path: &str) -> (Vec<String>, Vec<String>) {
    let gen = Generator::new(template_path).expect("provided a value template");
        
    let taken: Vec<String> = gen.take(number_of_rows).collect();
    let mut header_vec: Vec<String> = Vec::new();

    let raw_headers: Vec<String> = taken.iter().take(1).map(|row| {
        let mut headers: HashSet<String> = HashSet::new();
        let v: Value = serde_json::from_str(row).unwrap();

        for (key, _value) in v.as_object().unwrap() {
            headers.insert(key.to_string());
        }

        header_vec = Vec::from_iter(headers.clone());
        header_vec.sort();
        header_vec.join(",")
    }).collect();

    let sorted_header_vec: Vec<String> = raw_headers[0].split(",").map(|s| s.to_string()).collect();

    let output_data: Vec<String> = taken.iter().map(|raw_json| {
        let v: Value = serde_json::from_str(raw_json).unwrap();
        let mut data: String = String::from("");
        
        for header in &sorted_header_vec {
            data += &(v[header].to_string() + ",");
        }
        
        data.pop();

        let cleaned_data = (data + "\n").replace("\"", "");
        cleaned_data
    }).collect();

    (output_data, header_vec)
}

fn get_matches() -> ArgMatches<'static> {
    App::new("ssv_jen")
    .version("0.1.1")
    .author("Aden Kenny <adenkenny@gmail.com>")
    .about("Generates random data from a template")
    .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .takes_value(true)
             .help("The file we'll output to"))
    .arg(Arg::with_name("template")
             .short("t")
             .long("template")
             .takes_value(true)
             .help("The template file"))
    .arg(Arg::with_name("rows")
            .short("r")
            .long("rows")
            .takes_value(true)
            .help("The number of rows to generate"))
    .get_matches()
}

fn get_template(matches: &ArgMatches) -> String {
    let raw = matches.value_of("template").expect("template must be provided");
    raw.to_string()
}

fn get_output(matches: &ArgMatches) -> String {
    let raw = matches.value_of("file").expect("output must be provided");
    raw.to_string()
}

fn get_rows(matches: &ArgMatches) -> usize {
    let raw_rows = matches.value_of("rows").expect("--rows must be provided");
    let num_rows = raw_rows.parse::<i32>().expect("The number of rows must be a positive int.");

    num_rows as usize
}
