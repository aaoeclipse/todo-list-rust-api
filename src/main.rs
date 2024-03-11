mod todo;
use todo::model::{Item,StatusItem, ItemList};


fn main() {
    let mut item = Item::new_item(1, "My First To Do".to_string(), StatusItem::Incomplete);
    println!("Item: {:?}", item);

    item.change_status(StatusItem::Complete(123123123));
    println!("Item: {:?}", item);

    let item_list = ItemList::new(item.clone());
    println!("item_list: {:?}", item_list);
    println!("Item: {:?}", item);
}
