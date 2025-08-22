use super::Mob;

impl Mob {
    pub fn recruit(&mut self, member: super::Member) {
        self.members.push(member);
    }

    pub fn mark_enemy(&mut self, enemy_name: String) {
        if !self.enemies.contains(&enemy_name) {
            self.enemies.push(enemy_name);
        }
    }

    pub fn ally_with(&mut self, other: &mut Mob) {
        if !self.enemies.contains(&other.name) && !other.enemies.contains(&self.name) {
            self.allies.push(other.name.clone());
            other.allies.push(self.name.clone());
        }
    }

    pub fn steal(&mut self, other: &mut Mob, amount: u32) {
        other.mark_enemy(self.name.clone());
        if other.wealth >= amount {
            other.wealth -= amount;
            self.wealth += amount;
        }
    }

    pub fn attack(&self, other: &mut Mob) -> bool {
        let my_power = self.boss.power + self.members.len() as u32;
        let other_power = other.boss.power + other.members.len() as u32;
        other.mark_enemy(self.name.clone());
        my_power > other_power
    }
}