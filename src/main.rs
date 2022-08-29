use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let mars_weight:f32 = calculate_weight_on_mars(46.0);

    println!("Weight on Mars: {}Kg", mars_weight);
}


fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}