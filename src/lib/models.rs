use colored::*;
use regex::Regex;
use std::result;

use crate::lib::line;

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub value: String,
    pub enabled: bool,
    pub position: usize,
    pub line_number: usize,
}

impl Alias {
    pub fn to_string(self) -> String {
        let mut alias_string = String::new();
        alias_string.push_str(&format!("{}={}", self.name, self.value));
        alias_string
    }

    pub fn all(content: &str) -> Vec<Alias> {
        let re =
            Regex::new(r#"(?P<disabled>[# ]?)alias\s(?P<alias>\w+)=['"](?P<value>[\w -_]+)['"]"#)
                .expect("Invalid Regex");

        re.captures_iter(content)
            .map(|caps| {
                let position = caps.get(0).unwrap().start();
                let line_number = line::from_pos(content, position);

                let alias = caps.name("alias").expect("NO Alias").as_str();

                let value = caps.name("value").expect("NO Value").as_str();

                let enabled = match caps.name("disabled").expect("NO Value").as_str() {
                    "#" => false,
                    _ => true,
                };

                return Alias {
                    name: String::from(alias),
                    value: String::from(value),
                    position,
                    line_number,
                    enabled,
                };
            })
            .collect()
    }

    pub fn with_name(content: &str, name: &str) -> Vec<Alias> {
        Alias::all(content)
            .into_iter()
            .filter(|alias| alias.name == name)
            .collect()
    }

    pub fn read_all(content: &str) {
        let aliases: Vec<Alias> = Alias::all(content);

        let all_aliases_count = aliases.len();
        let enabled_aliases_count: usize = aliases
            .iter()
            .filter(|&a| a.enabled)
            .collect::<Vec<&Alias>>()
            .len();

        println!("{}", "===========================".dimmed());
        println!(
            "Found {:2} aliases. {:2} enabled.",
            all_aliases_count, enabled_aliases_count
        );
        println!("{}", "===========================".dimmed());

        for item in aliases {
            if item.enabled {
                println!(
                    "[x] {:12}{}",
                    item.name.green().bold(),
                    item.value.underline()
                );
            } else {
                println!(
                    "[ ] {:12}{}",
                    item.name.bold().dimmed(),
                    item.value.dimmed()
                );
            }
        }
    }

    pub fn count(alias: &str, content: &str) -> usize {
        let re_string = format!(r#"alias\s{}=['"](?P<value>[\w -_]+)['"]?"#, alias);
        let re = Regex::new(&re_string).unwrap();

        if !re.is_match(&content) {
            return 0;
        }

        let caps = re.captures(&content).unwrap();

        return caps.len();
    }

    pub fn get_rows(alias: &str, content: &str) -> Vec<usize> {
        let mut rows = Vec::new();

        let re_string = format!(r#"alias\s{}=['"](?P<value>[\w -_]+)['"]?"#, alias);
        let re = Regex::new(&re_string).unwrap();

        for cap in re.captures_iter(&content) {
            let pos = cap.get(0).unwrap().start();
            rows.push(line::from_pos(&content, pos));
        }

        return rows;
    }

    pub fn get_line_number(
        alias: &str,
        content: &str,
        path: &str,
    ) -> result::Result<usize, String> {
        let re_string = format!(r#"alias\s{}=['"](?P<value>[\w -_]+)['"]?"#, alias);
        let re = Regex::new(&re_string).expect("Invalid Regex!");

        if !re.is_match(&content) {
            return Err(format!(
                "{} Alias '{}' not found in '{}'",
                "[ERROR]".black().on_red().blink().bold(),
                alias.yellow().bold(),
                path.black().on_yellow().bold()
            ));
        }

        let caps = re.captures(&content).unwrap();
        let position = caps.get(0).unwrap().start();
        let line = line::from_pos(&content, position);

        return Ok(line);
    }
}
