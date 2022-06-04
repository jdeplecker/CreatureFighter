use crate::Creature;
use rand::prelude::SliceRandom;
use std::collections::HashMap;
use std::ops::Add;

pub struct World {
    pub creatures: Vec<Creature>,
    pub winners: HashMap<String, u32>,
}

impl World {
    pub fn fight(&mut self, num_fights: u32) {
        let mut iterations = 1;
        while iterations < num_fights {
            let (creature1_opt, creature2_opt) = self.rand_contenders();

            if creature1_opt.is_none() || creature2_opt.is_none() {
                self.handle_win(&mut iterations, creature1_opt, creature2_opt);
                return;
            }

            let creature1 = creature1_opt.unwrap();
            let creature2 = creature2_opt.unwrap();

            let creature_one_won_opt = self.creature_one_won(&creature1, &creature2);

            if creature_one_won_opt.is_none() {
                self.creatures.push(creature1);
                self.creatures.push(creature2);
            } else if creature_one_won_opt.unwrap() {
                self.increment_winner_score(creature1.name.clone());
                self.creatures.push(creature1);
            } else {
                self.increment_winner_score(creature2.name.clone());
                self.creatures.push(creature2)
            }
            iterations += 1;
        }
    }

    fn increment_winner_score(&mut self, creature_name: String) {
        let new_score = self.winners.get(&creature_name).get_or_insert(&0).add(1);
        self.winners.insert(creature_name, new_score);
    }

    fn creature_one_won(&self, creature1: &Creature, creature2: &Creature) -> Option<bool> {
        let first_roll = creature1.roll();
        let second_roll = creature2.roll();
        if first_roll == second_roll {
            return None;
        }
        Some(first_roll > second_roll)
    }

    fn handle_win(
        &self,
        iterations: &mut u32,
        creature1_opt: Option<Creature>,
        creature2_opt: Option<Creature>,
    ) {
        let winner = creature1_opt.or(creature2_opt).unwrap();
        println!(
            "Fight is over Creature {} won in {} iteration(s) with {} wins",
            winner.name,
            iterations,
            self.winners.get(&winner.name).unwrap()
        );
        let highest_winner = self
            .winners
            .iter()
            .max_by(|score1, score2| score1.1.cmp(&score2.1))
            .unwrap();
        println!(
            "Honorable mention: Creature {} won the most with {} wins",
            highest_winner.0, highest_winner.1
        );
    }

    fn rand_contenders(&mut self) -> (Option<Creature>, Option<Creature>) {
        self.creatures.shuffle(&mut rand::thread_rng());
        let creature1_opt = self.creatures.pop();
        let creature2_opt = self.creatures.pop();
        (creature1_opt, creature2_opt)
    }
}
