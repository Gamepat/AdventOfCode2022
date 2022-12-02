// https://adventofcode.com/2022/day/1

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let split = input.split("\n\n");

    let mut sums = Vec::<u32>::new();

    split.for_each(|elv| {
        let mut sum: u32 = 0;
        elv.split("\n")
            .for_each(|item| sum += item.parse::<u32>().unwrap());
        sums.push(sum);
    });

    sums.sort();
    sums.reverse();

    println!("Largest is: {:?}", sums[0]);
    println!(
        "First 3 together are: {:?}",
        sums[0..3].to_vec().iter().sum::<u32>()
    );
}
