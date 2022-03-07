use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Stats {
    amount: usize,
    percentage: f64,
}

pub struct Rarity {
    pub total: usize,
    pub data: HashMap<String, HashMap<String, Stats>>,
}

impl Rarity {
    pub fn new(total: usize) -> Self {
        let data = HashMap::new();

        Self { total, data }
    }

    pub fn count_trait(&mut self, layer: &str, name: &str) {
        match self.data.get_mut(layer) {
            Some(traits) => {
                match traits.get_mut(name) {
                    Some(stats) => {
                        stats.amount += 1;
                        stats.percentage = stats.amount as f64 / self.total as f64;
                    }
                    None => {
                        traits.insert(
                            name.to_string(),
                            Stats {
                                amount: 1,
                                percentage: 1.0 / self.total as f64,
                            },
                        );
                    }
                };
            }
            None => {
                let mut traits = HashMap::new();

                traits.insert(
                    name.to_string(),
                    Stats {
                        amount: 1,
                        percentage: 1.0 / self.total as f64,
                    },
                );

                self.data.insert(layer.to_string(), traits);
            }
        }
    }
}
