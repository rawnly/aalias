use clap::{App, Arg, AppSettings};
use std::{env, fs::OpenOptions, io::{Result, prelude::*, stdin, stdout}, path::{Path}};
use simple_storage::{Storage, Value};

extern crate dirs;

fn format_path(path: &str) -> String {
    let home: String = match dirs::home_dir() {
        Some(dir) => String::from(dir.to_str().expect("Error")),
        None => String::new()
    };

    return path.replace("~", home.as_str());
}

fn ask_question(question: &str) -> Result<String> {
    let mut input = String::new();

    print!("{} ", question);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)?;

    input.truncate(input.len() - 1);

    Ok(input)
}


fn main() -> Result<()> {
    let mut storage: Storage = Storage::new("/tmp/alias-config.json");
    storage.pull()?;

    let matches = App::new("AddAlias")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("name")
                .about("Name of the alias")
                .index(1)
        )
        .arg(
            Arg::new("value")
                .about("Value of the alias")
                .multiple(true)
                .index(2)
                .requires("name")
        )
        .subcommand(
            App::new("setup")
                    .about("setup basic configuration")
                    .alias("config")
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let mut alias_key: &str = "";
    let mut alias_value: String = String::new();
    let alias_file_storage_key : String = "alias-file".to_string();


    let homepath = format_path("~");

    let file: String = match storage.get(alias_file_storage_key.to_string()) {
        Ok(val) => val.to_string().replace("\"", ""),
        Err(_er) => format!("{}/.bashrc", homepath)
    };


    if let Some(_) = matches.subcommand_matches("setup") {
        let filename = format_path(&ask_question(&format!("FILE PATH: ({})", file))?);

        if filename.len() > 0 {
            let path = Path::new(&filename);

            if !path.exists() {
                eprintln!("The specified path is not valid: '{}'", path.display());
                return Ok(());
            }

            storage
                .put(alias_file_storage_key, Value::String(filename))?;
        }

        return Ok(());
    }

    if matches.value_of("name") == None && matches.value_of("value") == None {
        println!("aalias <name> <value>");
        println!("OR");
        println!("aalias [setup|config]");
        return Ok(());
    }

    if let Some(k) = matches.value_of("name") {
        alias_key = k;
    }

    if let Some(values) = matches.values_of("value") {
        for val in values {
            alias_value.push_str(&format!("{} ", val));
        }

        alias_value = String::from(alias_value.trim_end());
    }

    let filepath = Path::new(&file);
    println!("writing to file: {}", filepath.display());

    if !filepath.exists() {
        eprintln!("The specified file: {} cannot be found.", file);
    }

    // open the file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)?;

    if let Err(e) = writeln!(&mut file, "alias {}=\"{}\"", alias_key, alias_value) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}
