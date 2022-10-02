// eventually have this class be a parent to dealer and user to implement playing into

pub struct Dealer {
    pub count_low: i32,
    pub count_high: i32,
}



impl Dealer {
    pub fn build() -> Dealer {
        return Dealer{count_low: 0, count_high: 0}
    }

    pub fn playing(&mut self, new_card: i32) -> bool {
        self.count_high += new_card;
        // self.count_low += if new_card == 11 {1} else {new_card};

        if self.count_high > 16 || self.count_low > 16 {
            return false;
        }

        return true;
    }
}