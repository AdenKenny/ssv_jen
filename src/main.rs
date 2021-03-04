use jen::generator::Generator;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use clap::{Arg, App, ArgMatches};
use serde_json::{Value};


fn main() {
    let matches = get_matches();
    
    let num_rows = get_rows(&matches);
    let template_name = get_template(&matches);
    let output_name = get_output(&matches);

    let output_path = Path::new(&output_name);
    let output_file = File::create(&output_path).expect("file could be created");

    let gen = Generator::new(&template_name).expect("provided a value template");


    let header_rows = get_header_row(gen);
    let header_row_str: String = header_rows.0;
    let header_row_vec: Vec<String> = header_rows.1;

    write_to_file(header_row_str, &output_file);
        
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let num_threads = get_threads(&matches);

    for _i in 0..num_threads {
        let new_headers = header_row_vec.to_vec();
        let new_template_name = template_name.to_owned();
        let new_output_file = output_file.try_clone().unwrap();
        let handle = std::thread::spawn(move || {
            let data = generate_data(num_rows / num_threads, new_template_name, new_headers);
            write_vec_to_file(data, &new_output_file);
        });
        thread_vec.push(handle);
    }

    for thread in thread_vec {
        thread.join().unwrap();
    }
}

fn get_header_row(generator: Generator) -> (String, Vec<String>) {

    let raw_header_row: Vec<String> = generator.take(1).collect();
    let raw_header: String = raw_header_row[0].to_owned();
    
    let json_header: Value = serde_json::from_str(&raw_header).unwrap();
    let mut headers: Vec<String> = Vec::new();

    for (key, _v) in json_header.as_object().unwrap() {
        headers.push(key.to_owned());
    }

    headers.sort();
    (format!("{}{}", headers.join(","), "\n"), headers)
}

fn write_to_file(output: String, f: &File) {
    let mut bw = BufWriter::new(f);

    bw.write(output.as_bytes()).unwrap();
}

fn write_vec_to_file(output: Vec<String>, f: &File) {
    let mut bw = BufWriter::new(f);

    for l in output {
        bw.write(l.as_bytes()).unwrap();
    }
}

fn generate_data(number_of_rows: usize, template_path: String, header_vec: Vec<String>) -> Vec<String> {
    let gen = Generator::new(template_path).expect("provided a value template");

    let taken: Vec<String> = gen.take(number_of_rows).collect();

    let output_data: Vec<String> = taken.iter().map(|raw_json| {
        let v: Value = serde_json::from_str(raw_json).unwrap();
        let mut data: String = "".to_owned();
        
        for header in &header_vec {
            if data.len() == 0 {
                data = v[header].to_string();
            }

            else {
                data = format!("{},{}", data, v[header]);
            }
        }
        
        data.pop();

        let cleaned_data = (data + "\n").replace("\"", "");
        cleaned_data
    }).collect();

    output_data
}

fn get_matches() -> ArgMatches<'static> {
    App::new("ssv_jen")
    .version("0.1.2")
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
    .arg(Arg::with_name("threads")
    .short("c")
    .long("threads")
    .takes_value(true)
    .help("The number of threads to use"))
    .get_matches()
}

fn get_template(matches: &ArgMatches) -> String {
    let raw = matches.value_of("template").expect("template must be provided");
    raw.to_owned()
}

fn get_output(matches: &ArgMatches) -> String {
    let raw = matches.value_of("file").expect("output must be provided");
    raw.to_owned()
}

fn get_rows(matches: &ArgMatches) -> usize {
    let raw_rows = matches.value_of("rows").expect("--rows must be provided");
    let num_rows = raw_rows.parse::<i32>().expect("The number of rows must be a positive int.");

    num_rows as usize
}

fn get_threads(matches: &ArgMatches) -> usize {
    let raw_threads = matches.value_of("threads").expect("--threads must be provided");
    let num_threads = raw_threads.parse::<i32>().expect("The number of threads must be a positive int.");

    num_threads as usize
}

