use rand::Rng;

pub struct Card {
    value: i32,
    num: i32,
}

pub struct Shoe {
    cards: Vec<Card>,
    pub total: i32,
}

impl Shoe {
    pub fn build(decks: i32) -> Shoe {
        let mut cards = vec![
            Card {value: 2, num: 4}, 
            Card {value: 3, num: 4}, 
            Card {value: 4, num: 4},
            Card {value: 5, num: 4},
            Card {value: 6, num: 4}, 
            Card {value: 7, num: 4}, 
            Card {value: 8, num: 4}, 
            Card {value: 9, num: 4}, 
            Card {value: 10, num: 12}, 
            Card {value: 11, num: 4}, 
        ];

        for element in cards.iter_mut() {
            element.num *= decks;
        }

        let total = cards.iter().map(|card| card.num).sum();
        return Shoe { cards, total }
    }

    pub fn pop_card(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let key = rng.gen_range(0..self.total);


        let mut running_sum = 0;
        for element in self.cards.iter_mut() {
            running_sum += element.num;
            if running_sum > key {
                element.num -= 1;
                self.total -= 1;
                return element.value;
            }
        }

        return -1;
    }

}