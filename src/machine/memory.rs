use std::{collections::HashMap, ops::{Index, IndexMut}};

use super::{Address, Data};

pub struct Memory {
    memory: HashMap<Address, Data>,
}

impl Memory {
    pub fn new(initial_data: Vec<Data>) -> Self {
        let mut memory = HashMap::new();

        for i in 0..initial_data.len() {
            memory.insert(i, initial_data[i]);
        }

        Memory { memory }
    }

    fn get(&self, address: Address) -> &Data {
        self.memory.get(&address).unwrap_or(&0)
    }

    fn get_mut(&mut self, address: Address) -> &mut Data {
        if self.memory.contains_key(&address) {
            self.memory.get_mut(&address).unwrap()
        } else {
            self.memory.insert(address, 0);
            self.memory.get_mut(&address).unwrap()
        }
    }
}

impl Index<Address> for Memory {
    type Output = Data;
    
    fn index(&self, index: Address) -> &Self::Output {
        &self.get(index)
    }
}

impl IndexMut<Address> for Memory {    
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        self.get_mut(index)
    }
}