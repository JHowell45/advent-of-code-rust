mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use day_1::Elves;
use day_2::{Game, GameTwo};
use day_3::{ElfGroups, RucksackPriorities};
use day_4::Sections;
use day_5::{Supplies, Crane};


fn day_1_solution() {
    let elves: Elves = Elves::load_elves(String::from("src/day_1/source.txt"));
    println!("{:#?}", elves);
    println!("Number of Elves: {}", elves.get_total_elves());
    let elf_id = elves.get_elf_id_with_most_calories();
    println!("Elf with the most calories: {}", elf_id);
    println!("Elf calories: {}", elves.get_elf_total_calories(elf_id));

    let n = 3;
    let top_n_elf_ids: Vec<i32> = elves.get_top_n_calories_elf_ids(n);
    println!("Top {} Elves: {:#?}", n, top_n_elf_ids);
    println!("Top {} Elves Calories: {:#?}", n, elves.get_total_calories_for_elf_ids(top_n_elf_ids));
    
}

fn day_2_solution() {
    let game: Game = Game::load_strategy_game("src/day_2/strategy_guide.txt");
    println!("{:#?}", game);
    println!("Current Score: {}", game.get_current_score());

    let game: GameTwo = GameTwo::load_strategy_game("src/day_2/strategy_guide.txt");
    println!("{:#?}", game);
    println!("Current Score: {}", game.get_current_score());
}

fn day_3_solution() {
    let rsp = RucksackPriorities::load_from_file("src/day_3/rucksack_items.txt");
    println!("{:#?}", rsp);

    let mut eg = ElfGroups::load_from_file("src/day_3/rucksack_items.txt");
    println!("Priority Points: {:#?}", eg.get_priority_points());
}

fn day_4_solution() {
    let sections = Sections::from_file("src/day_4/section_assignments.txt");
    println!("{:#?}", sections);
}

fn day_5_solution() {
    let data: Vec<Vec<char>> = vec![
        vec!['W', 'M', 'L', 'F'],
        vec!['B', 'Z', 'V', 'M', 'F'],
        vec!['H', 'V', 'R', 'S', 'L', 'Q'],
        vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'],
        vec!['L', 'S', 'W'],
        vec!['F', 'V', 'P', 'M', 'R', 'J', 'W'],
        vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F'],
        vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'],
        vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'],
        ];
    let mut crane: Crane = Crane::from_array(data);
    crane.parse_instructions_file("src/day_5/crane_instructions.txt");
    println!("The Top Crates: {}", crane.get_top_crates());
}


fn main() {
    // day_1_solution();
    // day_2_solution();
    // day_3_solution();
    // day_4_solution();
    day_5_solution()
}
