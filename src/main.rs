use std::io;

fn main() {
    println!("Enter your weight in Kg");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    // To remove white spaces and unwrap because parse return a Result or error
    let weight:f32 = input.trim().parse().unwrap();

    let mars_weight:f32 = calculate_weight_on_mars(weight);
    
    println!("Weight on Mars: {}Kg", mars_weight);
}


fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}