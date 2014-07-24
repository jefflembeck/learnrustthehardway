fn main(){
    let name: &str = "Jeffrey M. Lembeck";
    let age: int = 31; // Not a lie
    let height: int = 67; // inches
    let weight: int = 160; // lbs
    let eyes: &str = "Brown";
    let teeth: &str = "White";
    let hair: &str = "Brown";

    println!("Let's talk about {:s}", name);
    println!("He's {:d} inches tall.", height);
    println!("He's {:d} pounds heavy.", weight);
    println!("Actually that's not too heavy.");
    println!("He's got {:s} eyes and {:s} hair.", eyes, hair);
    println!("His teeth are usually {:s} depending on the coffee", teeth);

    // this line is tricky, try to get it exactly right
    println!("If I add {:d}, {:d}, and {:d} I get {:d}.", age, height, weight, age + height + weight);
}
