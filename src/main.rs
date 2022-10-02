
mod shoe;
mod dealer;

fn main() {
    let mut shoe = shoe::Shoe::build(2);
    let shoe_depth = (shoe.total as f32 * 0.50) as i32; // what percentage of the deck is played

    let mut dealer;

    
    dealer = dealer::Dealer::build();


    while dealer.playing(shoe.pop_card()) {
        let c = dealer.count_high;
        println!("betting: {c}");
    } 

    let c = dealer.count_high;
    println!("{c}");
  
}