fn main() {
    let args = std::env::args().skip(1);

    match analyzer_cli::run(args) {
        Ok(output) => println!("{output}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
