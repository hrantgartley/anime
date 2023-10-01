mod anime;
use anime::Anime;

fn main() {
    let anime = String::from("Naruto");
    let epi = anime.episode();
    let name = anime.name();
    let rating = anime.generate_score();
    let w_rating = anime.rating();
    println!();
    anime.print_anime(name, epi, rating, w_rating);
}
