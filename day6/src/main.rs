fn number_of_fish_after_days(days: usize, fish_ages: &[usize]) -> usize {
    let mut fish_trackers: Vec<usize> = vec![0; 9];
    for age in fish_ages {
        fish_trackers[*age] += 1;
    }

    let mut day_zero = 0;
    for _ in 0..days {
        let day_six = (day_zero + 7) % 9;
        fish_trackers[day_six] += fish_trackers[day_zero];
        day_zero = (day_zero + 1) % 9;
    }

    fish_trackers.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let fish_ages: Vec<usize> = input
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("part 1: {}", number_of_fish_after_days(80, &fish_ages));
    println!("part 2: {}", number_of_fish_after_days(256, &fish_ages));
}
