mod fun;
mod cli;
mod scheme;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use scheme::*;


fn main() {
    let mut scheme = SCHEME::init();

    let app = cli::build_cli().get_matches();
    // var::concatinate(&app, &mut scheme);

    if let Some(subcommand) = app.subcommand_name() {
        match subcommand {
            "colors" => cli::colors::run(&app, &mut scheme),
            _ => Ok(())
        }.ok();
    }
}
