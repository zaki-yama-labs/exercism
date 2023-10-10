pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // iterate over minefield
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '*' => '*',
                    _ => {
                        let mut count = 0;
                        for x in i.saturating_sub(1)..=i + 1 {
                            for y in j.saturating_sub(1)..=j + 1 {
                                if x < minefield.len()
                                    && y < row.len()
                                    && minefield[x].chars().nth(y).unwrap() == '*'
                                {
                                    count += 1;
                                }
                            }
                        }
                        if count > 0 {
                            std::char::from_digit(count, 10).unwrap()
                        } else {
                            ' '
                        }
                    }
                })
                .collect()
        })
        .collect()
}
