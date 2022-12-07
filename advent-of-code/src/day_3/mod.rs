use core::slice::Iter;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Compartment {
    items: Vec<String>,
}

impl Compartment {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_line(line: String) -> Self {
        let mut items: Vec<String> = Vec::new();
        for item in line.chars().into_iter() {
            items.push(String::from(item))
        }
        return Self { items: items };
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn items_as_string(&self) -> String {
        let mut items_s = String::new();
        for item in self.items.iter() {
            items_s.push_str(item);
        }
        return items_s;
    }

    pub fn iter(&self) -> Iter<String> {
        return self.items.iter();
    }

    pub fn contains(&self, item: &String) -> bool {
        return self.items.contains(item);
    }
}

#[derive(Debug)]
pub struct Rucksack {
    left_compartment: Compartment,
    right_compartment: Compartment,
}

impl Rucksack {
    pub fn new() -> Self {
        Self {
            left_compartment: Compartment::new(),
            right_compartment: Compartment::new(),
        }
    }

    pub fn from_text_line(line: &str) -> Self {
        let mut left: String = String::from(line);
        let right = left.split_off(left.len() / 2);
        return Self {
            left_compartment: Compartment::from_line(left),
            right_compartment: Compartment::from_line(right),
        };
    }

    pub fn add_left_item(&mut self, item: &str) {
        self.left_compartment.add_item(item);
    }

    pub fn add_right_item(&mut self, item: &str) {
        self.right_compartment.add_item(item);
    }

    pub fn get_shared_items(&self) -> Vec<String> {
        let mut shared_items: Vec<String> = Vec::new();
        for left_item in self.left_compartment.iter() {
            if !shared_items.contains(&left_item) && self.right_compartment.contains(&left_item) {
                shared_items.push(left_item.clone());
            }
        }
        return shared_items;
    }

    pub fn all_items(&self) -> String {
        let mut items = self.left_compartment.items_as_string();
        items.push_str(&self.right_compartment.items_as_string());
        return items;
    }
}

#[derive(Debug)]
pub struct RucksackPriorities {
    priority_points: u32,
    alphabet: [char; 26],
}

impl RucksackPriorities {
    pub fn new() -> Self {
        Self {
            priority_points: 0,
            alphabet: Self::generate_alphabet(),
        }
    }

    pub fn load_from_file(filepath: &str) -> Self {
        let path = String::from(filepath);
        let mut instance = Self::new();
        let file = File::open(path).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                let rucksack = Rucksack::from_text_line(&ip);
                instance.add_rucksack_priority(rucksack);
            }
        }
        return instance;
    }

    pub fn add_rucksack_priority(&mut self, rucksack: Rucksack) {
        let shared_items: Vec<String> = rucksack.get_shared_items();
        for item in shared_items.iter() {
            self.priority_points += self.get_item_priority(item);
        }
    }

  pub fn add_shared_item_priority(&mut self, item: &str) {
      self.priority_points += self.get_item_priority(item);
  }

    pub fn get_priority_points(&self) -> u32 {
        return self.priority_points;
    }

    fn get_item_priority(&self, item: &str) -> u32 {
        let mut priority: u32 = 0;
        let mut letter: char = item.chars().next().unwrap();
        if !letter.is_lowercase() {
            priority += 26;
            letter = letter.to_ascii_lowercase();
        }
        let pos: u32 = (self.alphabet.iter().position(|&x| x == letter).unwrap() + 1)
            .try_into()
            .unwrap();
        priority += pos;
        return priority;
    }

    fn generate_alphabet() -> [char; 26] {
        return [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
    }
}

#[derive(Debug)]
pub struct ElfGroup {
  rucksacks: Vec<String>
}

impl ElfGroup {
  pub fn new() -> Self {
    Self { rucksacks: Vec::new() }
  }

  pub fn add_rucksack(&mut self, rucksack: &str) {
    self.rucksacks.push(String::from(rucksack));
  }

  pub fn find_badge(&mut self) -> String {
    let mut common_items: Vec<char> = Vec::new();
    let first_rucksack = self.rucksacks.remove(0);
    let second_rucksack = self.rucksacks.remove(0);
    for item in first_rucksack.chars() {
      if !common_items.contains(&item) && second_rucksack.contains(&String::from(item)) {
        common_items.push(item)
      }
    }
    let third_rucksack = self.rucksacks.remove(0);
    for item in common_items.iter() {
      if third_rucksack.contains(&String::from(*item)) {
        return String::from(*item);
      }
    }
    return String::from("");
  }
}

#[derive(Debug)]
pub struct ElfGroups {
    elf_badge_groups: Vec<ElfGroup>,
  current_group_count: u8,
}

impl ElfGroups {
    pub fn new() -> Self {
        Self {
            elf_badge_groups: Vec::new(),
          current_group_count: 0,
        }
    }

  pub fn load_from_file(filepath: &str) -> Self {
        let path = String::from(filepath);
        let mut instance = Self::new();
        let file = File::open(path).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                instance.add_rucksack(&ip);
            }
        }
        return instance;
    }

  pub fn add_rucksack(&mut self, rucksack: &str) {
    self.check_create_new_group();
    let last_index = self.elf_badge_groups.len() - 1;
    if let Some(group) = self.elf_badge_groups.get_mut(last_index) {
      group.add_rucksack(rucksack);
      self.update_current_group_count();
    }
  }

  fn check_create_new_group(&mut self) {
    if self.current_group_count >= 3 || self.elf_badge_groups.len() == 0 {
      self.current_group_count = 0;
      self.create_new_group();
    }
  }

  fn create_new_group(&mut self) {
    self.elf_badge_groups.push(ElfGroup::new());
  }

  fn update_current_group_count(&mut self) {
    self.current_group_count += 1;
  }

  pub fn get_priority_points(&mut self) -> u32 {
    let mut points: u32 = 0;
    for group in self.elf_badge_groups.iter_mut() {
      let mut priority = RucksackPriorities::new();
      let shared_item = group.find_badge();
      priority.add_shared_item_priority(&shared_item);
      points += priority.get_priority_points();
    }
    return points;
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_rucksack() {
        let rs = Rucksack::from_text_line("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!("vJrwpWtwJgWr", rs.left_compartment.items_as_string());
        assert_eq!("hcsFMMfFFhFp", rs.right_compartment.items_as_string());
        assert_eq!(vec!["p"], rs.get_shared_items());
    }

    #[test]
    fn test_second_rucksack() {
        let rs = Rucksack::from_text_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!("jqHRNqRjqzjGDLGL", rs.left_compartment.items_as_string());
        assert_eq!("rsFMfFZSrLrFZsSL", rs.right_compartment.items_as_string());
        assert_eq!(vec!["L"], rs.get_shared_items());
    }

    #[test]
    fn test_third_rucksack() {
        let rs = Rucksack::from_text_line("PmmdzqPrVvPwwTWBwg");
        assert_eq!("PmmdzqPrV", rs.left_compartment.items_as_string());
        assert_eq!("vPwwTWBwg", rs.right_compartment.items_as_string());
        assert_eq!(vec!["P"], rs.get_shared_items());
    }

    #[test]
    fn test_fourth_rucksack() {
        let rs = Rucksack::from_text_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!(vec!["v"], rs.get_shared_items());
    }

    #[test]
    fn test_fifth_rucksack() {
        let rs = Rucksack::from_text_line("ttgJtRGJQctTZtZT");
        assert_eq!(vec!["t"], rs.get_shared_items());
    }

    #[test]
    fn test_sixth_rucksack() {
        let rs = Rucksack::from_text_line("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(vec!["s"], rs.get_shared_items());
    }

    #[test]
    fn test_rucksack_priority() {
        let mut rsp = RucksackPriorities::new();
        rsp.add_rucksack_priority(Rucksack::from_text_line("vJrwpWtwJgWrhcsFMMfFFhFp"));
        rsp.add_rucksack_priority(Rucksack::from_text_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        rsp.add_rucksack_priority(Rucksack::from_text_line("PmmdzqPrVvPwwTWBwg"));
        rsp.add_rucksack_priority(Rucksack::from_text_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        rsp.add_rucksack_priority(Rucksack::from_text_line("ttgJtRGJQctTZtZT"));
        rsp.add_rucksack_priority(Rucksack::from_text_line("CrZsJsPPZsGzwwsLwLmpwMDw"));
        assert_eq!(157, rsp.get_priority_points());
    }
}
