use std::fs::read_to_string;

fn main() {
    println!("Finding the elf that is carrying the most calories!");

    find_elf_with_max_calories();
}

fn find_elf_with_max_calories() {
    let mut current_elf: u16 = 1;
    let mut current_elf_calories: u32 = 0;
    let mut max_calories: u32 = 0;
    let mut elf_with_max_calories: u16 = 1;

    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            if current_elf_calories > max_calories {
                max_calories = current_elf_calories;
                elf_with_max_calories = current_elf;
            }
            current_elf += 1;
            current_elf_calories = 0;
            continue;
        }
        let current_calories: u32 = line.parse().unwrap();

        current_elf_calories += current_calories;
    }

    println!("The elf with the most calories is: {elf_with_max_calories}");
    println!("They are carrying {max_calories} calories!");
}
