
fn main(){
    let x = format!("There are {:d} types of people.", 10i);
    let binary = "binary";
    let do_not = "don't";
    let y = format!("Those who know {:s} and those who {:s}", binary, do_not);

    println!("{}",x);
    println!("{}",y);

    println!("I said: {}.", x);
    println!("I also said: '{:s}'.", y);

    let hilarious = false;
    let joke_evaluation = format!("Isn't that jokes so funny?! {}", hilarious);

    println!("{}", joke_evaluation);

    let w = "This is the left side of...";
    let e = "a string with a right side.";

    println!("{:s} {:s}", w, e);

}
