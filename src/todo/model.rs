#[derive(Debug, Clone)]
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
        Self {
            id,
            description,
            status,
        }
    }
    pub fn change_status(&mut self, status: StatusItem) {
        self.status = status;
    }
}

impl ItemList {
    pub fn new(item: Item) -> Self {
        Self { list: vec![item] }
    }
}
