use anyhow::Result;

fn count_num_trees(map: &str, slope: (usize, usize)) -> usize {
    let lines: Vec<&str> = map.lines().collect();
    let height = lines.len();
    let width = lines.iter().next().unwrap().chars().count();

    let mut num_trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < height {
        if lines[y].chars().nth(x % width).unwrap() == '#' {
            num_trees += 1;
        }
        y += slope.1;
        x += slope.0;
    }
    num_trees
}

pub fn day3() -> Result<()> {
    /*
        let map = r#"..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#"#;
        */
    let map = include_str!("day3_input.txt");
    let num_trees = count_num_trees(&map, (3, 1));
    println!("num_trees: {}", num_trees);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let val = slopes
        .iter()
        .map(|slope| count_num_trees(&map, *slope))
        .fold(1, |acc, x| acc * x);
    println!("multi all values: {}", val);

    Ok(())
}
