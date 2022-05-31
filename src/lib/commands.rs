use crate::OpenOptions;
use clap::ArgMatches;
use simple_storage::Storage;
use simple_storage::Value;
use std::{
    fs::read_to_string,
    io::{prelude::*, Result},
    path::Path,
};

use crate::{lib::utils, line, path_helpers, Alias};
use colored::*;

#[derive(Debug, Clone)]
pub struct Commands;

impl<'a> Commands {
    pub fn setup(self, file: String, storage: &mut Storage, key: String) -> Result<()> {
        let filename =
            path_helpers::format_path(&utils::ask_question(&format!("FILE PATH: ({})", file))?);

        if filename.len() > 0 {
            let path = Path::new(&filename);

            if !path.exists() {
                eprintln!("The specified path is not valid: '{}'", path.display());
                return Ok(());
            }

            storage.put(key, Value::String(filename))?;
        }

        return Ok(());
    }

    pub fn disable(self, matches: &ArgMatches, file: &str) -> Result<()> {
        let content = self.content(file.to_string());

        let alias_name = matches.value_of("alias-name").unwrap();

        println!("Disabling: {:2}", alias_name.yellow().dimmed());

        if Alias::count(alias_name, &content) > 1 {
            let aliases = Alias::with_name(&content, alias_name);
            let enabled_aliases = aliases
                .into_iter()
                .filter(|alias| alias.enabled)
                .collect::<Vec<Alias>>();

            if enabled_aliases.len() > 1 {
                println!();
                println!("{}", "Multiple aliases found!".red().bold());
                println!(
                    "{}",
                    "Please specify the alias you want to disable.".red().bold()
                );

                let pick = utils::pick_from_list(
                    "Write the number of the alias you want to disable: ",
                    enabled_aliases.to_vec(),
                );

                let alias = match pick.map(|n| enabled_aliases.get(n - 1)) {
                    Some(a) => a.or(None),
                    None => None,
                };

                return match alias {
                    Some(alias) => {
                        if !alias.enabled {
                            println!(
                                "{}{}{}",
                                "Alias \"".yellow().bold(),
                                alias_name,
                                "\" is already disabled!".yellow().bold()
                            );
                            return Ok(());
                        }

                        let updated_content = line::comment(&content, alias.line_number);
                        utils::overwrite_file(file, &updated_content)?;

                        Ok(())
                    }
                    None => {
                        println!("{}", "Alias not found!".yellow().bold());
                        Ok(())
                    }
                };
            }
        }

        return match Alias::with_name(&content, alias_name).first() {
            Some(alias) => {
                if !alias.enabled {
                    println!(
                        "{}{}{}",
                        "Alias \"".yellow().bold(),
                        alias_name,
                        "\" is already disabled!".yellow().bold()
                    );
                    return Ok(());
                }

                let updated_content = line::comment(&content, alias.line_number);
                utils::overwrite_file(&file, &updated_content)?;

                Ok(())
            }
            None => {
                println!("{}", "Alias not found!".red().bold());
                Ok(())
            }
        };
    }

    pub fn enable(self, matches: &ArgMatches, file: &str) -> Result<()> {
        let content = self.content(file.to_string());

        let alias_name = matches.value_of("alias-name").unwrap();

        println!("Enabling: {:2}", alias_name.green());

        if Alias::count(alias_name, &content) > 1 {
            let aliases = Alias::with_name(&content, alias_name);
            let disabled_aliases = aliases
                .into_iter()
                .filter(|alias| !alias.enabled)
                .collect::<Vec<Alias>>();

            if disabled_aliases.len() > 1 {
                println!();
                println!("{}", "Multiple aliases found!".red().bold());
                println!(
                    "{}",
                    "Please specify the alias you want to enable.".red().bold()
                );

                let alias = match utils::pick_from_list(
                    "Write the number of the alias you want to enable",
                    disabled_aliases.to_vec(),
                ) {
                    Some(n) => disabled_aliases.get(n - 1),
                    None => {
                        println!("{}", "Index out of range!".red().bold());
                        return Ok(());
                    }
                };

                return match alias {
                    Some(alias) => {
                        if alias.enabled {
                            println!(
                                "{}{}{}",
                                "Alias \"".yellow().bold(),
                                alias_name,
                                "\" is already enabled!".yellow().bold()
                            );
                            return Ok(());
                        }

                        let updated_content = line::uncomment(&content, alias.line_number);
                        utils::overwrite_file(file, &updated_content)?;

                        Ok(())
                    }
                    None => {
                        println!("{}", "Alias not found!".yellow().bold());
                        Ok(())
                    }
                };
            }
        }

        return match Alias::with_name(&content, alias_name).first() {
            Some(alias) => {
                if alias.enabled {
                    println!(
                        "{}{}{}",
                        "Alias \"".yellow().bold(),
                        alias_name,
                        "\" is already enabled!".yellow().bold()
                    );
                    return Ok(());
                }

                let updated_content = line::uncomment(&content, alias.line_number);
                utils::overwrite_file(file, &updated_content)?;

                Ok(())
            }
            None => {
                println!("{}", "Alias not found!".yellow().bold());
                Ok(())
            }
        };
    }

    pub fn list(self, file: String) -> Result<()> {
        Alias::read_all(&self.content(file));
        Ok(())
    }

    fn content(self, file: String) -> String {
        return read_to_string(Path::new(&file)).expect("Error reading the file.");
    }
}

pub fn add(matches: &ArgMatches, file: String) -> Result<()> {
    let mut alias_key: &str = "";
    let mut alias_value: String = String::new();

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
    let mut file = OpenOptions::new().write(true).append(true).open(file)?;

    if let Err(e) = writeln!(&mut file, "alias {}=\"{}\"", alias_key, alias_value) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}
