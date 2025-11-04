use rltk::RandomNumberGenerator;

pub struct RandomEntry<T: Clone> {
    value: T,
    weight: i32,
}

#[derive(Default)]
pub struct RandomTable<T: Clone> {
    entries: Vec<RandomEntry<T>>,
    total_weight: i32,
}
impl<T> RandomTable<T>
where
    T: Clone,
{
    pub fn new() -> RandomTable<T> {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(mut self, value: T, weight: i32) -> RandomTable<T> {
        self.total_weight += weight;
        self.entries.push(RandomEntry { value, weight });
        self
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<T> {
        if self.total_weight == 0 {
            return None;
        }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return Some(self.entries[index].value.clone());
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        None
    }
}
