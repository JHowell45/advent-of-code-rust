use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// An instance for storing all of the elves and their calories info.
#[derive(Debug)]
pub struct Elves {
    /// All of the elves stored in this instance.
    elves: HashMap<i32, Elf>,
    /// the index of the last elf added to the Elves instance.
    last_index: i32,
}

impl Elves {
    /// Returns an Elves instance with an empty HashMap for the elves and a last index tracker.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// ```
    pub fn new() -> Self {
        Self {
            elves: HashMap::new(),
            last_index: -1,
        }
    }

    /// Returns an Elves instance with a filled elves HashMap loaded from a text file.
    /// 
    /// # Arguments
    /// 
    /// * `filepath` - The full path to the file to load into the Elves instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::load_elves("path/to/file.txt");
    /// ```
    pub fn load_elves(filepath: String) -> Self {
        let mut instance = Self::new();
        let file = File::open(filepath).expect("Unable to load file!");
        let reader = BufReader::new(file);
        let mut elf_id = instance.get_last_index();

        for line in reader.lines() {
            if let Ok(ip) = line {
                if ip == String::from("") {
                    elf_id = instance.add_empty_elf();
                } else {
                    instance.add_food_to_elf(Food::new(ip.parse::<i32>().unwrap()), elf_id);
                }
            }
        }
        return instance;
    }

    /// Returns the elf ID for the elf with the most calories.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id_1: i32 = elves.add_empty_elf();
    /// let elf_id_2: i32 = elves.add_empty_elf();
    /// let elf_id_3: i32 = elves.add_empty_elf();
    /// 
    /// elves.add_food_to_elf(Food::new(100), elf_id_1);
    /// elves.add_food_to_elf(Food::new(150), elf_id_2);
    /// elves.add_food_to_elf(Food::new(200), elf_id_3);
    /// 
    /// let elf_id = elves.get_elf_id_with_most_calories();
    /// ```
    pub fn get_elf_id_with_most_calories(&self) -> i32 {
        return self.get_highest_calories_elf_id(&Vec::new());
    }

    /// Return the Elf IDs for the Elves with the top N calories.
    /// 
    /// # Arguments
    /// 
    /// * `n` - the number of Elf IDs to return.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id_1: i32 = elves.add_empty_elf();
    /// let elf_id_2: i32 = elves.add_empty_elf();
    /// let elf_id_3: i32 = elves.add_empty_elf();
    /// 
    /// 
    /// elves.add_food_to_elf(Food::new(100), elf_id_1);
    /// elves.add_food_to_elf(Food::new(150), elf_id_2);
    /// elves.add_food_to_elf(Food::new(200), elf_id_3);
    /// 
    /// let ids: Vec<i32> = elves.get_top_n_calories_elf_ids(2);
    /// ass
    /// ```
    pub fn get_top_n_calories_elf_ids(&self, n: u8) -> Vec<i32> {
        let mut elf_ids:Vec<i32> = Vec::new();

        for _ in 1..=n {
            let elf_id = self.get_highest_calories_elf_id(&elf_ids);
            elf_ids.push(elf_id);
        }
        return elf_ids;
    }

    /// Base function for getting the elf ID for the elf with the highest calories, ignores specified IDs.
    /// 
    /// # Arguments
    /// 
    /// * `ignore_list` - the list of Elf IDs to ignore.
    fn get_highest_calories_elf_id(&self, ignore_list: &Vec<i32>) -> i32 {
        let mut result: i32 = -1;
        let mut max_calories: i32 = 0;
        for (elf_id, elf) in self.elves.iter() {
            if !ignore_list.contains(elf_id) && elf.get_total_calories() > max_calories {
                max_calories = elf.get_total_calories();
                result = elf_id.clone();
            }
        }
        return result;
    }

    pub fn get_total_calories_for_elf_ids(&self, elf_ids: Vec<i32>) -> i32 {
        let mut total_calories: i32 = 0;
        for elf_id in elf_ids.iter() {
            if let Some(elf) = self.elves.get(&elf_id) {
                total_calories += elf.get_total_calories();
            }
        }
        return total_calories;
    }

    /// Return the total number of calories for an elf.
    /// 
    /// # Arguments
    /// 
    /// * `elf_id` - the ID for the elf.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id_1: i32 = elves.add_empty_elf();
    /// elves.add_food_to_elf(Food::new(100), elf_id_1);
    /// 
    /// let total_calories = elves.get_elf_total_calories(elf_id_1);
    /// ```
    pub fn get_elf_total_calories(&self, elf_id: i32) -> i32 {
        if let Some(elf) = self.elves.get(&elf_id) {
            return elf.get_total_calories();
        } else {
            return -1;
        }
    }

    /// Returns the total number of elves in the vector.
    /// 
    /// # Examples
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// assert_eq!(elves.get_total_elves(), 0);
    /// 
    /// elves.add_empty_elf();
    /// elves.add_empty_elf();
    /// elves.add_empty_elf();
    /// assert_eq!(elves.get_total_elves(), 3);
    /// ```
    pub fn get_total_elves(&self) -> usize {
        return self.elves.len();
    }

    /// Add a new elf to the Elves instance.
    /// 
    /// # Arguments
    /// 
    /// * `new_elf` - add new Elf instance to the vector of elves.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id: i32 = elves.add_empty_elf();
    /// ```
    pub fn add_elf(&mut self, new_elf: Elf) -> i32 {
        self.increment_last_index();
        let index = self.get_last_index();
        self.elves.insert(index, new_elf);
        return index;
    }

    /// Add a new empty elf to the Elves instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id: i32 = elves.add_empty_elf();
    /// ```
    pub fn add_empty_elf(&mut self) -> i32 {
        self.increment_last_index();
        let index = self.get_last_index();
        self.elves.insert(index, Elf::new());
        return index;
    }

    /// Return the ID for the last elf added to the vector.
    /// 
    /// # Arguments
    /// 
    /// ```
    /// use day_1::Elves;
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id_1: i32 = elves.add_empty_elf();
    /// assert_eq!(elves.get_last_index(), elf_id_1);
    /// 
    /// let elf_id_2: i32 = elves.add_empty_elf();
    /// assert_eq!(elves.get_last_index(), elf_id_2);
    /// 
    /// let elf_id_3: i32 = elves.add_empty_elf();
    /// assert_eq!(elves.get_last_index(), elf_id_3);
    /// ```
    pub fn get_last_index(&self) -> i32 {
        return self.last_index;
    }


    /// Increase the `last_index` param by one.
    fn increment_last_index(&mut self) {
        self.last_index += 1;
    }

    /// Add food to a specified elf.
    /// 
    /// # Arguments
    /// 
    /// * `food` - the food to be added to the specified elf.
    /// * `elf_id` - the ID for the specified elf.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use day_1::{Elves, Food};
    /// let mut elves: Elves = Elves::new();
    /// 
    /// let elf_id: i32 = elves.add_empty_elf();
    /// elves.add_food_to_elf(Food::new(1000), elf_id);
    /// ```
    pub fn add_food_to_elf(&mut self, food: Food, elf_id: i32) {
        if let Some(elf) = self.elves.get_mut(&elf_id) {
            elf.add_food(food);
        }
    }
}

#[derive(Debug)]
pub struct Elf {
    foods: Vec<Food>,
}

impl Elf {
    pub fn new() -> Self {
        Self { foods: Vec::new() }
    }

    pub fn add_food(&mut self, new_food: Food) {
        self.foods.push(new_food)
    }

    pub fn add_food_by_calories(&mut self, calories: i32) {
        self.foods.push(Food::new(calories));
    }

    pub fn get_total_calories(&self) -> i32 {
        let mut total_calories: i32 = 0;
        for food in self.foods.iter() {
            total_calories += food.get_calories();
        }
        return total_calories;
    }
}

#[derive(Debug)]
pub struct Food {
    calories: i32,
}

impl Food {
    pub fn new(calories: i32) -> Self {
        Self { calories: calories }
    }

    pub fn get_calories(&self) -> i32 {
        return self.calories;
    }
}
