use super::{Boss, Member};

#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,// #debug by turning  boss to a vec
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
    pub allies: Vec<String>,
    pub enemies: Vec<String>,
}
//understanding  vecs, and how tey are differently used in the examples above,  why  is it that only these variants are of Vec type
impl Mob {
    pub fn new(name: &str, boss: Boss, wealth: u32, city: &str) -> Self {
        Self {
            name: name.to_string(),
            boss,
            members: vec![],
            cities: vec![(city.to_string(), 5)],
            wealth,
            allies: vec![],
            enemies: vec![],
        }
    }
}
