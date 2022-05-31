pub fn remove(input: &str, idx: usize) -> String {
    let lines: Vec<&str> = input
        .split('\n')
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index != &(idx - 1))
        .map(|(_, l)| l)
        .collect();

    return String::from(lines.join(&String::from('\n')));
}

pub fn comment(input: &str, line_number: usize) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            if idx == line_number - 1 {
                return format!("#{}", line);
            }

            return String::from(line);
        })
        .collect();

    return String::from(lines.join(&String::from('\n')));
}

pub fn uncomment(input: &str, line_number: usize) -> String {
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

    return String::from(lines.join(&String::from('\n')));
}

pub fn from_pos(input: &str, pos: usize) -> usize {
    let mut line: usize = 1;
    let mut i: usize = 0;

    loop {
        if i >= pos {
            break;
        };

        let ch = input.chars().nth(i).unwrap();

        if ch == '\n' {
            line = line + 1;
        }

        i = i + 1;
    }

    return line;
}
