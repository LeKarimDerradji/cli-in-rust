use clap::Parser;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    // Reading the file using the std::fs::read_to_string library, with an argument
    // equal to the args.path passed by the user
    // Here we are borrowing the args.path variable.
    let content = std::fs::read_to_string(&args.path);
    match content {
        Ok(content) => { println!("File content {}", content); },
        Err(error) => { println!("Error! {}", error); },
    }

    // Iterate over the content, line by line, when a line is matching the pattern, display it.
    for line in content.lines()  {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
