fn idx(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

fn get_grid(input: &str) -> Vec<char> {
    let grid: Vec<char> = input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    grid
}

fn part_one(grid: Vec<char>, width: usize) {
    grid.iter().enumerate().for_each(|(index, item)| {
        if *item != ' ' {
            println!(
                "CURRENT ITEM: {item} INDEX -> {index}\n-> ( {} , {} )",
                index % width,
                index / width
            );
        }
    });
    grid.iter().enumerate().for_each(|(index, item)| {
        println!("ITEM: {item}");
    });
}

// vec[y * grid_width + x]

fn main() {
    let input = include_str!("../puzzle_input_test.txt");
    // 2. Derive the grid's dimensions
    let width = input.lines().next().unwrap_or("").chars().count();
    let grid: Vec<char> = get_grid(input);
    part_one(grid, width);
}
