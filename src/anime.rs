use rand::prelude::*;
use std::io;

pub struct AnimeData {
    name: String,
    episode: u32,
    score: i32,
    rating: String,
}

impl AnimeData {
    pub fn new() -> AnimeData {
        let name = AnimeData::read_input("Enter anime name: ");
        let episode = AnimeData::read_input("Enter number of episodes: ")
            .parse()
            .expect("Please type a number!");
        let score = rand::thread_rng().gen_range(1..10);
        let rating = AnimeData::read_input("Enter the watch rating: ");

        AnimeData {
            name,
            episode,
            score,
            rating,
        }
    }

    fn read_input(prompt: &str) -> String {
        let mut line = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_string()
    }

    pub fn print(&self) {
        println!();
        println!("Number of episodes: {}", self.episode);
        println!("Anime name: {}", self.name);
        println!("Rating: {}/10", self.score);
        println!("Watch rating: {}", self.rating);
    }

    pub fn add_to_array(&self, anime_array: &mut Vec<String>) {
        let anime_info = format!(
            "Number of episodes: {}\nAnime name: {}\nRating: {}/10\nWatch rating: {}",
            self.episode, self.name, self.score, self.rating
        );
        anime_array.push(anime_info);
    }
}
