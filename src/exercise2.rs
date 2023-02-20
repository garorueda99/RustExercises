// const CARS_PER_SPEED: u8 = 221;

// pub fn production_rate_per_hour(speed: u8) -> f64 {
//     let brute_production:f64 = f64::from(CARS_PER_SPEED * speed); Incorrect
//     let brute_production:f64 = f64::from(CARS_PER_SPEED) * f64::from(speed); Correct
//     match speed {
//         x if x >=1 && x<=4 => brute_production,
//         x if x >=5 && x<=8 => brute_production * 0.9,
//         9 |10 => brute_production * 0.77,
//         _ => 0.0,
//     }
// }

// pub fn working_items_per_minute(speed: u8) -> u32 {
//   (production_rate_per_hour(speed) / f64::from(60)) as u32
// }


// fn main() {
//  println!("{}",production_rate_per_hour(6));
// }

const CARS_PER_SPEED: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let brute_production: f64 = f64::from(CARS_PER_SPEED) * f64::from(speed);
    match speed {
        1..=4 => brute_production,
        5..=8 => brute_production * 0.9,
        9 | 10 => brute_production * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}


fn main() {
    println!("{}", production_rate_per_hour(6));
    println!("{}", working_items_per_minute(6));
    
}