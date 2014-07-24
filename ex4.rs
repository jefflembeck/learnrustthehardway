fn main(){
    let cars: int = 100;
    let space_in_car: int = 4;
    let drivers: int = 30;
    let passengers: f32 = 90.0;
    let cars_not_driven = cars - drivers;
    let cars_driven = drivers;
    let carpool_capacity = cars_driven * space_in_car;
    let average_passengers_per_car = passengers / cars_driven as f32;

    println!("There are {:d} cars available", cars);
    println!("There are only {:d} drivers available", drivers);
    println!("There will be {:d} empty cars today.", cars_not_driven);
    println!("We can transport {:d} people today.", carpool_capacity);
    println!("We have {:s} to carpool today.", passengers.to_string());
    println!("We need to put about {:s} in each car.", average_passengers_per_car.to_string());
}
