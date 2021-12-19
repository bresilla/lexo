pub mod colors;

use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        // .after_help("Does really amazing things to great people...but be careful with -R")
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        // .global_setting(AppSettings::NextLineHelp)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        // .global_setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
}
