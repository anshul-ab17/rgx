use std::path::PathBuf;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]struct Cli{
    syntax:String,
    path:PathBuf
}

fn main() {
    /*  operating system usually represents command-line-arguments as a list of strings.
        manually getting the args using os
        let syntax = env::args().nth(1).expect("no syntax!");  ---    //std::env::args() that gives you an iterator of the given arguments.
        let path = env::args().nth(2).expect("no path given");
        let args= Cli{
            // CLI arguments as a custom data type that represents the inputs to your program.
            syntax,
            path:PathBuf::from(path),
        };
    */ 
    // Parsing CLI arguments with Clap
    let args = Cli::parse();
    println!("syntax :{:?}, path :{:?}",args.syntax, args.path);

}
