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
    let mut second_most_calories: u32 = 0;
    let mut elf_with_second_most_calories = 1;
    let mut third_most_calories: u32 = 0;
    let mut elf_with_third_most_calories: u16 = 0;
    let mut top_3_calorie_count: u32 = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            if current_elf_calories > max_calories {
                third_most_calories = second_most_calories;
                elf_with_third_most_calories = elf_with_second_most_calories;

                second_most_calories = max_calories;
                elf_with_second_most_calories = elf_with_max_calories;

                max_calories = current_elf_calories;
                elf_with_max_calories = current_elf;
            } else if current_elf_calories > second_most_calories {
                third_most_calories = second_most_calories;
                elf_with_third_most_calories = elf_with_second_most_calories;

                second_most_calories = current_elf_calories;
                elf_with_second_most_calories = current_elf;
            } else if current_elf_calories > third_most_calories {
                third_most_calories = current_elf_calories;
                elf_with_third_most_calories = current_elf;
            }
            current_elf += 1;
            current_elf_calories = 0;
            continue;
        }
        let current_calories: u32 = line.parse().unwrap();

        current_elf_calories += current_calories;
    }

    top_3_calorie_count = max_calories + second_most_calories + third_most_calories;

    println!("The elf with the most calories is: {elf_with_max_calories}!");
    println!("They are carrying {max_calories} calories!");
    println!("The elf with the second most calories is: {elf_with_second_most_calories}!");
    println!("They are carrying {second_most_calories} calories!");
    println!("The elf with the third most calories is: {elf_with_third_most_calories}!");
    println!("They are carrying {third_most_calories} calories!");
    println!("The top 3 elves put together are carrying {top_3_calorie_count}!");
}
