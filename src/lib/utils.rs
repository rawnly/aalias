use crate::Alias;
use colored::*;
use std::{
    fs::OpenOptions,
    io::{prelude::*, stdin, stdout, Result},
};

pub fn ask_question(question: &str) -> Result<String> {
    let mut input = String::new();

    print!("{} ", question);
    let _ = stdout().flush();
    stdin().read_line(&mut input)?;

    input.truncate(input.len() - 1);

    Ok(input)
}

pub fn pick_from_list(text: &str, aliases: Vec<Alias>) -> Option<usize> {
    let mut idx = 0;
    for alias in aliases {
        if alias.enabled {
            println!(
                "{}) [x] {:12}{}",
                idx + 1,
                alias.name.green().bold(),
                alias.value.underline()
            );
        } else {
            println!(
                "{}) [ ] {:12}{}",
                idx + 1,
                alias.name.bold().dimmed(),
                alias.value.dimmed()
            );
        }

        idx += 1;
    }

    let alias_num = ask_question(text).unwrap().parse::<usize>();

    let n = match alias_num {
        Ok(num) => num,
        _e => 0,
    };

    if idx < n || idx <= 0 {
        return None;
    }

    return Some(n);
}

pub fn overwrite_file(filepath: &str, new_content: &str) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open(&filepath)?;

    file.write_all(new_content.as_bytes())?;
    file.flush()?;

    return Ok(());
}
