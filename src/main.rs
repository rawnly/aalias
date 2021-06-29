use clap::{App, Arg, AppSettings};
use std::{env, fs::{OpenOptions, read_to_string}, result, io::{Result, prelude::*, stdin, stdout}, path::{Path}};
use simple_storage::{Storage, Value};
use regex::Regex;

use colored::*;
use dirs;

// extern crate dirs;

fn remove_line(input: &str, idx: usize) -> String {
    let lines: Vec<&str> = input
        .split('\n')
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index != &(idx - 1))
        .map(|(_, l)| l)
        .collect();

    return String::from(lines
        .join(&String::from('\n')));
}

fn comment_line(input: &str, line_number: usize) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            if idx == line_number - 1 {
                return format!("#{}", line)
            }

            return String::from(line);
        })
        .collect();

    return String::from(lines
        .join(&String::from('\n')));
}

fn uncomment_line(input: &str, line_number: usize) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            if idx == line_number - 1 {
                return format!("{:2} ", line.replacen("#", "", 1));
            }

            return String::from(line);
        })
        .collect();

    return String::from(lines
        .join(&String::from('\n')));
}

fn line_from_pos(input: &str, pos: usize) -> usize {
    let mut line: usize = 1;
    let mut i: usize = 0;

    loop {
        if i >= pos {
            break;
        };

        let ch = input
            .chars()
            .nth(i)
            .unwrap();

        if ch == '\n' {
            line = line + 1;
        }

        i = i + 1;
    }

    return line;
}

fn overwrite_file(filepath: &str, new_content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(&filepath)?;

    file.write_all(new_content.as_bytes())?;
    file.flush()?;

    return Ok(());
}

fn get_alias_line_number(alias: &str, filecontent: &str, filepath: &str) -> result::Result<usize, String> {
    let re_string = format!(r#"alias\s{}=['"](?P<value>[\w -_]+)['"]?"#, alias);
    let re = Regex::new(&re_string).unwrap();

    if !re.is_match(&filecontent) {
        return Err(
            format!(
                "{} Alias '{}' not found in '{}'", "[ERROR]"
                    .black()
                    .on_red()
                    .blink()
                    .bold(),
                alias
                    .yellow()
                    .bold(),
                filepath
                    .black()
                    .on_yellow()
                    .bold()
            )
        );
    }

    let caps= re.captures(&filecontent).unwrap();
    let match_idx = caps.get(0).unwrap().start();
    let line = line_from_pos(&filecontent, match_idx);

    return Ok(line);
}

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

struct Alias {
    name: String,
    value: String,
    enabled: bool
}



fn read_aliases(content: &str) {
    let re = Regex::new(r#"(?P<disabled>[# ]?)alias\s(?P<alias>\w+)=['"](?P<value>[\w -_]+)['"]"#)
        .expect("Invalid Regex");

    let aliases : Vec<Alias> = re.captures_iter(content).map(|caps| {
        let alias = caps
            .name("alias")
            .expect("NO Alias")
            .as_str();

        let value = caps
            .name("value")
            .expect("NO Value")
            .as_str();

        let enabled = match caps.name("disabled").expect("NO Value").as_str() {
            "#" => false,
            _ => true
        };

        return Alias {
            name: String::from(alias),
            value: String::from(value),
            enabled
        }
    }).collect();

    let all_aliases_count = aliases.len();
    let enabled_aliases_count: usize = aliases
        .iter()
        .filter(|&a| a.enabled)
        .collect::<Vec<&Alias>>()
        .len();


    println!("{}", "===========================".dimmed());
    println!("Found {:2} aliases. {:2} enabled.", all_aliases_count, enabled_aliases_count);
    println!("{}", "===========================".dimmed());

    for item in aliases {
        if item.enabled {
            println!("[x] {:12}{}", item.name.green().bold(), item.value.underline());
        } else {
            println!("[ ] {:12}{}", item.name.bold().dimmed(), item.value.dimmed());
        }
    }
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
                    .about("Setup basic configuration")
                    .alias("config")
        )
        .subcommand(
            App::new("list")
                .about("List alla aliases")
                .alias("ls")
        )
        .subcommand(
            App::new("enable")
                .about("Enable an alias")
                .arg(
                    Arg::new("alias-name")
                        .required(true)
                )
        )
        .subcommand(
            App::new("disable")
                .about("Disable an alias")
                .arg(
                    Arg::new("alias-name")
                        .required(true)
                )
        )
        // .subcommand(
        //     App::new("delete")
        //         .about("Delete an alias")
        //         .alias("del")
        //         .arg(
        //             Arg::new("alias-name")
        //                 .required(true)
        //         )
        // )
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

    if let Some(_) = matches.subcommand_matches("list") {
        let filecontent = read_to_string(Path::new(&file))
            .expect("Error reading the file.");

        read_aliases(&filecontent);

        return Ok(());
    }

    // DISABLE
    if let Some(m) = matches.subcommand_matches("disable") {
        let filecontent = read_to_string(Path::new(&file))
            .expect("Error reading the file.");

        let alias_to_remove = m.value_of("alias-name").unwrap();
        println!("Disabling: {:2}", alias_to_remove.yellow().dimmed());

        let line = get_alias_line_number(alias_to_remove, &filecontent, &file).unwrap();
        let updated_content = comment_line(&filecontent, line);

        overwrite_file(&file, &updated_content)?;

        return Ok(());
    }

    // ENABLE
    if let Some(m) = matches.subcommand_matches("enable") {
        let filecontent = read_to_string(Path::new(&file))
            .expect("Error reading the file.");

        let alias_to_remove = m
            .value_of("alias-name")
            .unwrap();

        println!("Enabling: {:2}", alias_to_remove.green());


        let line = get_alias_line_number(alias_to_remove, &filecontent, &file).unwrap();
        let updated_content = uncomment_line(&filecontent, line);

        overwrite_file(&file, &updated_content)?;

        return Ok(());
    }

    // DELETE
    if let Some(m) = matches.subcommand_matches("delete") {
        let filecontent = read_to_string(Path::new(&file))
            .expect("Error reading the file.");

        let alias_to_remove = m.value_of("alias-name").unwrap();
        println!("Deleting: {:2}", alias_to_remove.red().bold());

        let line = get_alias_line_number(alias_to_remove, &filecontent, &file).unwrap();
        let updated_content = remove_line(&filecontent, line);

        println!("Updated: {}", updated_content);
        overwrite_file(&file, &updated_content)?;

        return Ok(());
    }

    // SETUP
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


    // ADD
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
