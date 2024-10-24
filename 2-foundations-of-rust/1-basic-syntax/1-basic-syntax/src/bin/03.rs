fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut max = 0;
    let mut min = 100;
    for i in input {
        if i > max {
            max = i;
        }
        if i < min {
             min = i;
        }
    }
    // TODO

    println!("{} is largest and {} is smallest", input.iter().max().unwrap(), input.iter().min().unwrap());
}
