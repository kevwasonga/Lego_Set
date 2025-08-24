use crate::mobs::Mob;
use rand::Rng;

pub struct World {
    pub mobs: Vec<Mob>,
}

impl World {
    pub fn new(mobs: Vec<Mob>) -> Self {
        Self { mobs }
    }

    
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
        let candidates: Vec<(usize, &Mob)> = self.mobs.iter()
            .enumerate()
            .filter(|(i, m)| *i != idx && m.wealth > 50 && !me.allies.contains(&m.name))
            .collect();

        if candidates.is_empty() { None }
        else {
            let richest = candidates.iter().max_by_key(|(_, m)| m.wealth).unwrap();
            Some(richest.0)
        }
    }
}





// pub fn run_turn(&mut self) {
//     for i in 0..self.mobs.len() {
//         let action = rand::thread_rng().gen_range(0..2);

//         if let Some(target_index) = self.choose_target(i) {
//             if action == 0 {
//                 self.mobs[i].steal(&mut self.mobs[target_index], 50);


// //                     //self.mobs[i].steal(&mut self.mobs[target_index], 50);
// //    |                     ---------    -----      ^^^^^^^^^ second mutable borrow occurs here
// //    |                     |            |
// //    |                     |            first borrow later used by call
// //    |                     first mutable borrow occurs here
// //   = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
//             } else {
//                 self.mobs[i].attack(&mut self.mobs[target_index]);
//             }
//         }
//     }
// }
