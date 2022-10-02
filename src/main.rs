
mod shoe;
mod dealer;
fn main() {
    let mut shoe = shoe::Shoe::build(2);
    let shoe_depth = (shoe.total as f32 * 0.50) as i32; // what percentage of the deck is played

    let mut card_count = 0;
    for _ in 0..shoe_depth {
        let card = shoe.pop_card();

        card_count += if card > 7 { 1 } else { -1 };
        println!("{card}");
    }

    println!("\n\n{card_count}");
}