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
                if action == 0 {
                    self.mobs[i].steal(&mut self.mobs[target_index], 50);
                } else {
                    self.mobs[i].attack(&mut self.mobs[target_index]);
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
