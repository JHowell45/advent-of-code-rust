mod day_1;
mod day_2;

use day_1::Elves;
use day_2::{Game, GameTwo};

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

fn main() {
    // day_1_solution();
    day_2_solution();
}
