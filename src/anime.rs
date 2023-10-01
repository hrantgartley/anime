use rand::prelude::*;
use std::io;

pub trait Anime {
    fn name(&self) -> String;
    fn episode(&self) -> u32;
    fn generate_score(&self) -> i32;
    fn rating(&self) -> String;
    fn print_anime(&self, name: String, episode: u32, score: i32, rating: String);
    fn run(&self);
}

impl Anime for String {
    fn name(&self) -> String {
        let mut line = String::new();
        println!("Enter anime name: ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_string()
    }

    fn episode(&self) -> u32 {
        let mut line = String::new();
        println!("Enter number of episodes: ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line: u32 = line.trim().parse().expect("Please type a number!");
        line
    }

    fn generate_score(&self) -> i32 {
        rand::thread_rng().gen_range(1..10)
    }

    fn rating(&self) -> String {
        let mut line = String::new();
        println!("Enter the watch rating");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_string()
    }

    fn print_anime(&self, name: String, episode: u32, score: i32, rating: String) {
        println!();
        println!("Number of episodes: {}", episode);
        println!("Anime name: {}", name);
        println!("Rating: {}/10", score);
        println!("Watch rating: {}", rating);
    }
    fn run(&self) {
        let name = self.name();
        let episode = self.episode();
        let score = self.generate_score();
        let rating = self.rating();
        self.print_anime(name, episode, score, rating);
    }
}
