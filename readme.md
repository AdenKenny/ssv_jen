# SsvJen

This is a simple CLI that generates ssv files with a fixed structure, with random data.

It is basically a wrapper around [Jen](https://github.com/whitfin/jen), and therefore uses Jen's template syntax which is based on [Tera](https://github.com/Keats/tera). You can see
examples of the templating in the examples folder.

To build SsvJen you need to have Rust and Cargo [installed](https://www.rust-lang.org/tools/install).

Then run the following inside the SsvJen folder:

```
cargo build
```

You can then run SsvJen like so:

```
$ ./ssv_jen -r 1000000 -t /home/akenny/ssv_jen/templates/other.tera -f /home/akenny/ssv_jen/output/output.csv
```
The following flags are required:

```
FLAGS:
    -r, --rows       The number of rows to generate
    -t, --template   The template file which the structure is derived from
    -f, --file       The file where the csv will be saved to
    -c, --threads    The number of threads to generate data with
```
