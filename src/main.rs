mod cli;
mod db;
mod scheme;
mod tui;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use clap::Parser;
use scheme::SCHEME;

use cli::derive;
// use cli::builder;

fn main() {
    let _scheme = SCHEME::init();

    let cli = derive::Cli::parse();
    // let matches = builder::cli().get_matches();

    if let Some(config_path) = cli.config().as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // match matches.subcommand() {
    //     Some(_) => { unimplemented!() },
    //     None => {}
    // }

    match &cli.command() {
        Some(derive::Commands::Add { subcommand }) => match subcommand {
            Some(_) => {
                let _ = db::main();
                print!("--+---");
            }
            None => {
                unimplemented!()
            }
        },
        Some(derive::Commands::Edit { list: _ }) => {
            println!("Edit");
            // open_db();
        }
        // Some(cli::Commands::Browse { list }) => { unimplemented!() },
        // Some(cli::Commands::Search { list }) => { unimplemented!() },
        Some(_) => {
            unimplemented!()
        }
        None => {}
    }
}

// fn open_db() -> Result<()> {
//     let conn = Connection::open("temp/test.db")?;
//     let mut stmt = conn.prepare("SELECT uuid FROM papers")?;
//     // let person_iter = stmt.query_map([], |row| {
//     //     Ok(Person {
//     //         id: row.get(0)?,
//     //         name: row.get(1)?,
//     //         data: row.get(2)?,
//     //     })
//     // })?;
//     println!("{:?}", stmt);
//     let result: String = stmt.query_row([], |row|
//         row.get(0)
//     )?;

//     let results: Vec<String> = stmt.query_map([], |row|
//         row.get(0)
//     )?.map(|x| x.unwrap()).collect();

//     println!("{:?}", results);

//     println!("result: {}", result);

//     Ok(())
// }
