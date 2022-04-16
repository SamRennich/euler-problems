/*
Problem 15: Lattice Paths

Starting in the top left corner of a 2×2 grid,
and only being able to move to the right and down,
there are exactly 6 routes to the bottom right corner.
How many such routes are there through a 20×20 grid?

Answer: 137846528820
*/

const RANGE: i32 = 20;

fn main() {
    let mut paths: u64 = 2;

    let step = |path, pow| {
        let ratio = (1 + pow) as f64 / (2 + pow) as f64;
        (path + (path as f64 * ratio) as u64) * 2
    };

    for power in 0..(RANGE - 1) {
        paths = step(paths, power);
    }

    println!("{}", paths);
}
