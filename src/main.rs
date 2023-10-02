mod anime;
use anime::AnimeData;

fn main() {
    let mut anime_array: Vec<String> = Vec::new();

    loop {
        let anime_data = AnimeData::new();
        anime_data.print();
        anime_data.add_to_array(&mut anime_array);

        println!("Do you want to add another anime? (y/n)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Anime list:");
    for anime in &anime_array {
        println!("{}", anime);
    }
}
