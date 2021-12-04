fn count_nbr_of_depth_increase(depths: &Vec<u32>) -> u32 {
    let mut iter = depths.iter();
    let mut previous_value: u32 = *iter.next().unwrap();

    iter.copied().fold(0, |accum, item| {
        let previous_previous_value = previous_value;
        previous_value = item;

        if previous_previous_value < item {
            accum + 1
        } else {
            accum
        }
    })
}

fn count_nbr_of_depth_increase_sliding_window(depths: &Vec<u32>, window_size: usize) -> u32 {
    let sliding_window_depths: Vec<u32> = depths
        .windows(window_size)
        .map(|v| v[0] + v[1] + v[2])
        .collect();
    count_nbr_of_depth_increase(&sliding_window_depths)
}

fn main() {
    let input = include_str!("../input.txt");

    let depths: Vec<u32> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();

    println!("part 1: {}", count_nbr_of_depth_increase(&depths));
    println!(
        "part 2: {}",
        count_nbr_of_depth_increase_sliding_window(&depths, 3)
    );
}
