use serde::{Serialize, Deserialize};
use crate::mobs::Mob;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub mobs: Vec<Mob>,
}

impl World {
    pub fn new(mobs: Vec<Mob>) -> Self {
        Self { mobs }
    }
    pub fn sort_mobs_by_wealth(&mut self){
        self.mobs.sort_by(|a,b|a.wealth.cmp(&b.wealth));
    }
//consider sorting by other methods e.g by respect..
    
    pub fn run_turn(&mut self) {
        for i in 0..self.mobs.len() {
            let action = rand::thread_rng().gen_range(0..2);
    
            if let Some(target_index) = self.choose_target(i) {
                let (attacker, target) = if i < target_index {
                    let (left, right) = self.mobs.split_at_mut(target_index);
                    (&mut left[i], &mut right[0])
                } else {
                    let (left, right) = self.mobs.split_at_mut(i);
                    (&mut right[0], &mut left[target_index])
                };
    
                if action == 0 {
                    attacker.steal(target, 50);
                } else {
                    attacker.attack(target);
                }
            }
        }
    }
    
    fn choose_target(&self, idx: usize) -> Option<usize> {
        let me = &self.mobs[idx];
        //idx tells us which mob (gang) is currently taking action.
        let candidates: Vec<(usize, &Mob)> = self.mobs.iter()
            .enumerate()
            .filter(|(i, m)| *i != idx && m.wealth > 50 && !me.allies.contains(&m.name))
            .collect();
            //print candidates eligible for attack..

        if candidates.is_empty() { None }
        else {
            let richest = candidates.iter().max_by_key(|(_, m)| m.wealth).unwrap();
            Some(richest.0)
        }
    }
}

