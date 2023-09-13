use clap::{Parser, Subcommand};
use std::path::PathBuf;

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Parser, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[command(author, version, about, long_about = None)]
#[command(name = "Lexo")]
#[command(author = "bresilla <trim.bresilla@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A command line tool to manage scientific papers and books")]
#[command(long_about = "A command line tool to manage scientific papers and books ")]
// #[command(about = "Does awesome things", long_about = None)]
// #[clap(disable_help_flag = true)]
// #[clap(disable_version_flag =true)]
#[clap(disable_help_subcommand = true)]
// #[clap(disable_version_subcommand = true)]
#[clap(disable_colored_help = false)]
#[clap(color = clap::ColorChoice::Always)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    /// Set the config file path
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Set the library file path
    #[arg(long, value_name = "FILE")]
    library: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum AddSubcommand {
    #[command(name = "doi")]
    #[command(about = "Add document from a doi")]
    Doi {
        #[arg(long, value_name = "BIB")]
        bib: Option<PathBuf>,
    },
    #[command(name = "xiv")]
    #[command(about = "Add document from an arxiv id")]
    Arxiv {
        #[arg(long, value_name = "BIB")]
        bib: Option<String>,
    },
    #[command(name = "bib")]
    #[command(about = "Add document from a bib file")]
    Bib {
        #[arg(long, value_name = "BIB")]
        bib: Option<PathBuf>,
    },
    #[command(name = "pdf")]
    #[command(about = "Add document from a pdf file")]
    Pdf {
        #[arg(long, value_name = "BIB")]
        bib: Option<PathBuf>,
    },
    #[command(name = "url")]
    #[command(about = "Add document from a url")]
    Url {
        #[arg(long, value_name = "BIB")]
        bib: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "add")]
    #[command(about = "Add document to the library")]
    Add {
        #[command(subcommand)]
        subcommand: Option<AddSubcommand>,
    },
    #[command(name = "edit")]
    #[command(about = "Edit document information in the library")]
    Edit {
        /// Edit document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "browse")]
    #[command(about = "Open document's url in a browser")]
    Browse {
        /// Browse document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "search")]
    #[command(about = "Search document from the library")]
    Search {
        /// Search document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "export")]
    #[command(about = "Export document from the library")]
    Export {
        /// Export document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "move")]
    #[command(about = "Move document from the library")]
    Move {
        /// Move document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "remove", aliases = &["delete"])]
    #[command(about = "Remove document from the library")]
    Remove {
        /// Remove document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "list")]
    #[command(about = "List document from the library")]
    List {
        /// List document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "show")]
    #[command(about = "Show document from the library")]
    Show {
        /// Show document from a specific importer
        #[arg(short, long)]
        list: bool,
    },
}
