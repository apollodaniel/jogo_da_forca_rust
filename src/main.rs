use rand::Rng;
use std::io::{self};

fn main() {
    let mut rng = rand::thread_rng();
    
    let all_words_array: [&str;90] = ["apple", "banana", "carrot", "dog", "elephant", "fish", "guitar", "happiness", "internet", "jungle", "kangaroo", "lemon", "mountain", "notebook", "ocean", "piano", "queen", "rainbow", "sunflower", "tiger", "cat", "tree", "house", "chair", "table", "book", "pen", "computer", "phone", "flower", "sun", "moon", "star", "cloud", "rain", "snow", "beach", "river", "music", "art", "food", "coffee", "tea", "friend", "family", "love", "smile", "laughter", "dream", "adventure", "peace", "hope", "success", "freedom", "kindness", "inspiration", "butterfly", "bird", "bicycle", "fire", "candle", "diamond", "island", "key", "mirror", "puzzle", "shadow", "time", "whisper", "yoga", "zebra", "oasis", "victory", "magic", "serenity", "garden", "joy", "wonder", "harmony", "sunset", "moonlight", "silence", "breeze", "wisdom", "courage", "imagination", "tranquility", "reflection", "serendipity", "gratitude"];
    let random_index = rng.gen_range(0..all_words_array.len());
    let selected_word = all_words_array[random_index];
    let mut guessed_word: Vec<String> = selected_word.chars().map(|_s| "_".to_string()).collect();

    println!("{} Bem vindo ao Jogo da Forca {}", "-".repeat(10), "-".repeat(10));
    
    let mut chances: usize = selected_word.chars().count() * 2;

    while chances > 0{
        println!("\nChute: {}", guessed_word.join(" "));
        println!("Chances: {}", chances);
        
        if !guessed_word.contains(&"_".to_string()) {
            break;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let contain = input.as_str().trim() != "" && selected_word.contains(&input.as_str().trim());
        
        if contain {
            let mut index: usize = 0;
            for letter in selected_word.chars(){
                if selected_word.chars().map(|s| s.to_string()).nth(index) == Some(input.trim().to_string()) {
                    // contains
                    guessed_word[index] = letter.to_string();
                }
                index+=1;
            }
        }else 
        {
            chances-=1;
        };
    }

    let ganhou: bool = selected_word == guessed_word.join("");
    if(ganhou){
        println!("\n{} Parabéns você ganhou! {}\n", "-".repeat(10), "-".repeat(10));
    }else{
        println!("\n{} Que pena, você perdeu! {}", "-".repeat(10), "-".repeat(10));
        println!("A palavra secreta era: {}!\n", selected_word);
    }


}
