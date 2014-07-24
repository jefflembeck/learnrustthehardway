fn main(){
    // Prints the intro

    println!("I will now count my chickens:");

    println!("Hens {:d}", 25i + 30i / 6i);
    println!("Roosters {:d}", 100i - 25i * 3i % 4i);

    println!("Now I will count the eggs:");

    println!("Is it true that 3 + 2 < 5 - 7");

    println!("{}", 3i + 2i < 5i - 7i);

    println!("What is 3 + 2? {:d}", 3i + 2i);
    println!("What is 5 - 7? {:d}", 5i - 7i);

    println!("Oh, that's why it's false.");

    println!("How about some more.");

    println!("Is it greater? {}", 5i > -2i);
    println!("Is it greater or equal? {}", 5i >= -2i);
    println!("Is it less or equal? {}", 5i <= -2i);
}
