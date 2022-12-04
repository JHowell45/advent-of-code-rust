mod day_1;

use day_1::Elves;

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

fn main() {
    day_1_solution();
}
