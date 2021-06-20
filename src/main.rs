extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("papir")
        .version("0.1")
        .about("Research paper and bibliography manager")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("library")
                .short("l")
                .long("lib")
                .value_name("PATH")
                .help("Sets a custom path for the libraries")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a document into a given library")
                .arg(
                    Arg::with_name("from")
                        .help("Add document from a specific importer")
                        .short("f")
                        .long("from")
                        .takes_value(true)
                        .value_name("TYPE"),
                )
                .arg(
                    Arg::with_name("input")
                        .help("the file to add")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit document information from a given library")
        )
        .subcommand(
            SubCommand::with_name("browse")
                .about("Open document's url in a browser")
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("Export a document from a given library")
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Move a document into some other path")
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a document from a library")
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List to STDOUT documents of a library")
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show all documents in a TUI environment")
        )
        .subcommand(
            SubCommand::with_name("help")
                .about("Show all documents in a TUI environment")
        )
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}
