pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // iterate over minefield
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| {
                    print!("{}", c);
                    return c;
                })
                .collect()
        })
        .collect()
}
