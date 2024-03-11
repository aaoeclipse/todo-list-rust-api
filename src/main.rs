#[derive(Debug, Clone)]
enum StatusItem {
    Complete(i64), // Date Unix time format (big int)
    Incomplete,
}

// implement status change.
#[derive(Debug, Clone)]
struct Item {
    id: u32,
    description: String, // should be unique.
    status: StatusItem,
}
#[derive(Debug)]
struct ItemList {
    list: Vec<Item>
}


impl Item {
    fn new_item(id: u32, description: String, status: StatusItem) -> Self {
        Self {
            id,
            description,
            status
        }
    }
    fn change_status(&mut self, status: StatusItem) {
        self.status = status;
    }
}

impl ItemList {
    fn new(item:Item ) -> Self {
        Self {
            list: vec![item]
        }
    }

}

fn main() {
    
    let mut item = Item::new_item(1, "My First To Do".to_string(), StatusItem::Incomplete);
    println!("Item: {:?}", item);

    item.change_status(StatusItem::Complete(123123123));
    println!("Item: {:?}", item);

    let mut item_list = ItemList::new(item.clone());
    println!("item_list: {:?}", item_list);
    println!("Item: {:?}", item);

}
