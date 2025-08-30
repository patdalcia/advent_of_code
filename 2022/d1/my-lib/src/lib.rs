pub fn solve_puzzle(input: &str) -> Vec<u32> {
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|group| group.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect();

    totals.sort_unstable_by(|a, b| b.cmp(a));
    totals
}
