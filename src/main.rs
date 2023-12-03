mod day1;

fn main() {
    use std::time::Instant;

    // day 1
    {
        let input_day_1 = read_lines(String::from("day-1-input.txt"));

        // part 1
        {
            let time_now = Instant::now();

            let sum = day1::part1(input_day_1.clone());
            println!("[day1.1:info] answer: {}", sum);

            let time_elapsed = time_now.elapsed();
            println!("[day1.1:perf] time: {:?}\n", time_elapsed);
        }

        // part 2
        {
            let time_now = Instant::now();

            let sum = day1::part2(input_day_1.clone());
            println!("[day1.2:info] answer: {}", sum);

            let time_elapsed = time_now.elapsed();
            println!("[day1.2:perf] time: {:?}\n", time_elapsed);
        }
    }
}

// read input file and return line-separated strings in vector
fn read_lines(input_path: String) -> Vec<String> {
    use std::fs::read_to_string;
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
