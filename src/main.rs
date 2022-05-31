pub mod lib;

use clap::{App, AppSettings, Arg};

use simple_storage::Storage;
use std::env;
use std::fs::OpenOptions;
use std::io::Result;

use crate::lib::{commands, line, models::Alias, path_helpers};

fn main() -> Result<()> {
    let mut storage: Storage =
        Storage::new(&path_helpers::format_path("~/.config/alias-config.json"));

    storage.pull()?;

    let matches = App::new("AddAlias")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("name").about("Name of the alias").index(1))
        .arg(
            Arg::new("value")
                .about("Value of the alias")
                .multiple(true)
                .index(2)
                .requires("name"),
        )
        .subcommand(
            App::new("setup")
                .about("Setup basic configuration")
                .alias("config"),
        )
        .subcommand(App::new("list").about("List alla aliases").alias("ls"))
        .subcommand(
            App::new("enable")
                .about("Enable an alias")
                .arg(Arg::new("alias-name").required(true)),
        )
        .subcommand(
            App::new("disable")
                .about("Disable an alias")
                .arg(Arg::new("alias-name").required(true)),
        )
        .subcommand(
            App::new("delete")
                .about("Delete an alias")
                .alias("del")
                .arg(Arg::new("alias-name").required(true)),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Stuff
    let alias_file_storage_key = "alias-file".to_string();

    let file: String = match storage.get(alias_file_storage_key.to_string()) {
        Ok(val) => val.to_string().replace("\"", ""),
        Err(_er) => path_helpers::format_path("~/.bashrc"),
    };

    let cmd = commands::Commands;

    if let Some(_) = matches.subcommand_matches("list") {
        return cmd.list(file);
    }

    // DISABLE
    if let Some(m) = matches.subcommand_matches("disable") {
        return cmd.disable(m, file.as_str());
    }

    // ENABLE
    if let Some(m) = matches.subcommand_matches("enable") {
        return cmd.enable(m, file.as_str());
    }

    // SETUP
    if let Some(_) = matches.subcommand_matches("setup") {
        return cmd.setup(file, &mut storage, alias_file_storage_key);
    }

    // ADD
    commands::add(&matches, file)
}
