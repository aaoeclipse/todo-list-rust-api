use serde::Serialize;
use std::ops::Index;


#[derive(Debug, Clone, Serialize)]
pub enum StatusItem {
    Complete(i64), // Date Unix time format (big int)
    Incomplete,
}

// implement status change.
#[derive(Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub description: String, // should be unique.
    pub status: StatusItem,
}
#[derive(Debug)]
pub struct ItemList {
    pub list: Vec<Item>,
}

impl Item {
    pub fn new_item(id: u32, description: String, status: StatusItem) -> Self {
        Self {id, description, status }
    }
    pub fn change_status(&mut self, status: StatusItem) {
        self.status = status;
    }
}

impl ItemList {
    pub fn new() -> Self {
        Self {list: Vec::new()}
    }

    pub fn add_new(&mut self, item: Item)  {
        self.list.push(item);
    }

    pub fn iter_tasks(&self) -> impl Iterator<Item = &Item> {
        self.list.iter()
    }

    pub fn access_todo(&self, index: usize) -> Option<&Item> {
        self.list.get(index)
        // self.list[index]
    }

    pub fn modify_status(&mut self, index: usize, status: StatusItem)  {
        if let Some(task) = self.list.get_mut(index) {
            task.change_status(status);
        } else {
            println!("No Value To Update");
        }
        
        }
}

impl Index<usize> for ItemList {
    type Output =  Item;

    // Why capital S? Has to do with the 'current' type
    // lowercase s > has to do with current isntance of the struct.
    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}