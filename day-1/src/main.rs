use std::fs;

fn main() {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    let file_path = user_input.trim();
    let poo

    let mut highest_cals: i32 = 0;
    let mut second_highest_cals: i32 = 0;
    let mut third_highest_cals: i32 = 0;
    let mut current_elf: i32 = 0;

    println!("In file {}", file_path);
    for line in fs::read_to_string(file_path)
        .expect("Should have been able to read file...")
        .lines()
    {
        if line.trim().is_empty() {
            if current_elf > highest_cals {
                highest_cals = current_elf;
            } else if current_elf > second_highest_cals {
                second_highest_cals = current_elf;
            } else if current_elf > third_highest_cals {
                third_highest_cals = current_elf;
            }
            current_elf = 0;
            continue;
        }
        let parsed_calories: i32 = line.parse().unwrap();
        current_elf += parsed_calories;
    }

    println!("Highest calories: {}", highest_cals);
    println!("Second highest calories: {}", second_highest_cals);
    println!("Third highest calories: {}", third_highest_cals);
    println!(
        "Top three combined calories: {}",
        highest_cals + second_highest_cals + third_highest_cals
    );
}
