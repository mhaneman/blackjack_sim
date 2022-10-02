// have different stratgies inherit this parent

pub struct Strat {
    diff: f32, // how confident the alg is on winning or losing
}

impl Strat {
    pub fn build() -> Strat {
        return Strat {}
    }

    pub fn update_stats(new_card: i32) {
        
    }
}