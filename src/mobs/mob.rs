use super::{Boss, Member};

#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,// only one boss, so no need for name.
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
    pub allies: Vec<String>,
    pub enemies: Vec<String>,
}
// 👉 “The struct defines the mob’s shape. The `new` method is a convenience constructor that helps us build mobs with sensible defaults.”

impl Mob {
    pub fn new(name: &str, boss: Boss, wealth: u32, city: &str) -> Self {
        Self {
            name: name.to_string(),
            boss,
            members: vec![],
            cities: vec![],
            wealth,
            allies: vec![],
            enemies: vec![],
        }
    }
}

 //CITIES CREATION
cities: vec![(city.to_string(), 5)],
* Initialize with `vec![]` (no cities).
* Provide a method like `add_city(name: &str, influence: u8)`.
