use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Score {
    current_score: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { current_score: 0 }
    }

    pub fn get_current_score(&self) -> u32 {
        return self.current_score;
    }

    pub fn win(&mut self, player_handsign: &str) {
        let mut score: u32 = 6;
        score += self.get_handsign_score(player_handsign);
        self.current_score += score;
    }

    pub fn draw(&mut self, player_handsign: &str) {
        let mut score: u32 = 3;
        score += self.get_handsign_score(player_handsign);
        self.current_score += score;
    }

    pub fn lose(&mut self, player_handsign: &str) {
        let mut score: u32 = 0;
        score += self.get_handsign_score(player_handsign);
        self.current_score += score;
    }

    fn get_handsign_score(&self, handsign: &str) -> u32 {
        return match handsign {
            "A" | "X" => 1,
            "B" | "Y" => 2,
            "C" | "Z" => 3,
            _ => 0,
        };
    }
}

#[derive(Debug)]
pub struct Game {
    score: Score,
    current_round: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: Score::new(),
            current_round: 0,
        }
    }

    pub fn load_strategy_game(filepath: &str) -> Self {
        let path = Path::new(filepath);
        let mut instance = Self::new();

        let file = File::open(path.as_os_str()).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split_ascii_whitespace().collect();
                let opponent = split.first().unwrap();
                let player = split.last().unwrap();
                instance.add_round(opponent, player);
            }
        }

        return instance;
    }

    pub fn add_round(&mut self, opponents_hand: &str, players_hand: &str) {
        let hands = (players_hand, opponents_hand);

        match hands {
            ("Y", "A") | ("Z", "B") | ("X", "C") => self.score.win(players_hand),
            ("X", "A") | ("Y", "B") | ("Z", "C") => self.score.draw(players_hand),
            ("Z", "A") | ("X", "B") | ("Y", "C") => self.score.lose(players_hand),
            _ => (),
        }

        self.current_round += 1;
    }

    pub fn get_current_score(&self) -> u32 {
        return self.score.get_current_score();
    }
}

#[derive(Debug)]
pub struct GameTwo {
    score: Score,
    current_round: u32,
}

impl GameTwo {
    pub fn new() -> Self {
        Self {
            score: Score::new(),
            current_round: 0,
        }
    }

    pub fn load_strategy_game(filepath: &str) -> Self {
        let path = Path::new(filepath);
        let mut instance = Self::new();

        let file = File::open(path.as_os_str()).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split_ascii_whitespace().collect();
                let opponent = split.first().unwrap();
                let player = split.last().unwrap();
                instance.add_round(opponent, player);
            }
        }

        return instance;
    }

    pub fn add_round(&mut self, opponents_hand: &str, round_end: &str) {
        match round_end {
            "X" => self.score.lose(Self::losing_hand(opponents_hand)),
            "Y" => self.score.draw(opponents_hand),
            "Z" => self.score.win(Self::winning_hand(opponents_hand)),
            _ => (),
        };

        self.current_round += 1;
    }

    fn winning_hand(hand: &str) -> &str {
        return match hand {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => "",
        }
    }

    fn losing_hand(hand: &str) -> &str {
        return match hand {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => "",
        }
    }

    pub fn get_current_score(&self) -> u32 {
        return self.score.get_current_score();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_game() {
        let mut game: Game = Game::new();
        assert_eq!(0, game.get_current_score());
    }

    #[test]
    fn test_game_example_1() {
        let mut game: Game = Game::new();
        game.add_round("A", "Y");
        assert_eq!(8, game.get_current_score());
    }

    #[test]
    fn test_game_example_2() {
        let mut game: Game = Game::new();
        game.add_round("B", "X");
        assert_eq!(1, game.get_current_score());
    }

    #[test]
    fn test_game_example_3() {
        let mut game: Game = Game::new();
        game.add_round("C", "Z");
        assert_eq!(6, game.get_current_score());
    }

    #[test]
    fn test_game_example() {
        let mut game: Game = Game::new();
        game.add_round("A", "Y");
        game.add_round("B", "X");
        game.add_round("C", "Z");
        assert_eq!(15, game.get_current_score());
    }

    #[test]
    fn test_empty_game_two() {
        let mut game: GameTwo = GameTwo::new();
        assert_eq!(0, game.get_current_score());
    }

    #[test]
    fn test_game_two_example_1() {
        let mut game: GameTwo = GameTwo::new();
        game.add_round("A", "Y");
        assert_eq!(4, game.get_current_score());
    }

    #[test]
    fn test_game_two_example_2() {
        let mut game: GameTwo = GameTwo::new();
        game.add_round("B", "X");
        assert_eq!(1, game.get_current_score());
    }

    #[test]
    fn test_game_two_example_3() {
        let mut game: GameTwo = GameTwo::new();
        game.add_round("C", "Z");
        assert_eq!(7, game.get_current_score());
    }

    #[test]
    fn test_game_two_example() {
        let mut game: GameTwo = GameTwo::new();
        game.add_round("A", "Y");
        game.add_round("B", "X");
        game.add_round("C", "Z");
        assert_eq!(12, game.get_current_score());
    }
}
