pub struct Dealer {
    count_high: i32,
    count_low: i32,
}



impl Dealer {
    fn hit(&mut self, new_card_val: i32) {
        self.count_high += new_card_val;
    }
}