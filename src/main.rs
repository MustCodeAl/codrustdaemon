mod ui;
mod api_attach;

use std::fs;
use clap::Parser;
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    println!("Hello, world!");
    let pb = indicatif::ProgressBar::new(100); // making progres bar load while lines rae being filed
    for num in 0..100 {
        // println!("{num}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        pb.inc(1);
    }

    // let learn = LearnCommand::parse();
    //     if learn {
    //         println!("learn");
    // add to completions
// add to fig completitions
// add to zsh completions
//         }
// else {
//
//     println!("not learn");
// }
    // let args = Cli::parse();
    //
    // for line in &args.list {
    //     eprintln!("test list P{}", line);
    //
    // }

    pb.finish_with_message("done");


    // println!("{:?}",args);
    println!("test");
}


/// use std::io::{self, Write};

/// let stdout = io::stdout(); // get the global stdout entity
/// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
/// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

// let result = std::fs::read_to_string("test.txt");
// let content = match result {
// Ok(content) => { content },
// Err(error) => { panic!("Can't deal with {}, just exit here", error); Err(error.info()) }
// };
// println!("file content: {}", content);

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let result = std::fs::read_to_string("test.txt");
//     let content = match result {
//         Ok(content) => { content },
//         Err(error) => { return Err(error.into()); }
//     };
//     println!("file content: {}", content);
//     Ok(())
// }
#[derive(Debug)]
struct CliError(String);

// example use if string.empty() { CliError{"".to_string() }}
// or use this
// .map_err(|err| CliError(format!("Error reading `{}`: {}", path, err)))?;
// println!("file content: {}", content);

// This pattern is in fact very common. It has one problem, though: We don’t store the original error, only its string representation. The often used anyhow library has a neat solution for that: similar to our CustomError type, its Context trait can be used to add a description. Additionally, it also keeps the original error, so we get a “chain” of error messages pointing out the root cause.
//
// Let’s first import the anyhow crate by adding anyhow = "1.0" to the [dependencies] section of our Cargo.toml file.
//
// The full example will then look like this:


// /use anyhow::{Context, Result};
///
/// fn main() -> Result<()> {
///     let path = "test.txt";
///     let content = std::fs::read_to_string(path)
///         .`with_context`(|| format!("could not read file `{}`", path))?;
///     println!("file content: {}", content);
///     Ok(())
/// }
#[derive(Debug, Parser)]
struct Cli {
    // help: String,

    learn: String,
    // or if you taking in a struct you can parse to a strig
    list: Vec<String>,

}


#[derive(Parser)]
struct LearnCommand {
    bool: bool,
}

#[derive(Parser)]
struct List {}

#[derive(Parser)]
struct Remove {}


#[derive(Parser)]
struct Update {}

#[derive(Parser)]
struct Init {}

#[derive(Parser)]
struct Daemon {}

// help [<command>...]
// learn <subject>...
// list [<selector>...]
// remove <selector>...
// update <selector>...
// init <pid> <shell>
// example-config [<flags>]
// daemon [<flags>]

use std::fs::File;
use std::io::copy;
use reqwest::{Response};
use futures_util::StreamExt;




