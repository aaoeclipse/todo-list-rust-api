mod todo;
use serde_json::json;
use todo::model::{Item, ItemList, StatusItem};
use warp::{reply::Json, Filter};
// cargo watch -q -c -w src/ -x run



// localhost:8888/todo -> todos los todos (todos espanol y todo ingles)
// list view
async fn get_item() -> Result<String, warp::Rejection> {

    let todo_1: Item = Item::new_item(1, String::from("First item"), StatusItem::Incomplete);
    let todo_2: Item = Item::new_item(2, String::from("Second item"), StatusItem::Incomplete);
    let todo_3: Item = Item::new_item(3, String::from("Third item"), StatusItem::Incomplete);
    let todo_4: Item = Item::new_item(4, String::from("Fourth item"), StatusItem::Incomplete); // to_string() afecta como se agrega a la lista. "Fourth".to_string()
 

    let mut task_list: ItemList = ItemList::new();
    task_list.add_new(todo_1.clone());
    task_list.add_new(todo_2.clone());
    task_list.add_new(todo_3.clone());
    task_list.add_new(todo_4.clone());


    // in another function
    task_list.modify_status(0, StatusItem::Complete(1110099988));
    println!("TASK LIST:{:#?}", task_list);



    for item in task_list.iter_tasks() {
        // hace return el OK y no funciona en el for loop. 
        println!("TASK: {:?}", item); }


    Ok(format!(
        "id: {}, description: {}, status: {:?}",
        todo_3.id, todo_3.description, todo_3.status
    ))
}

// localhost:8888/todo/123 -> 123 (regresar solo el todo con 123 de id y con mas info)\
// detailed view
// i32, u32, usize, i64, u64, isize, f32, f64, char, str, String, bool, Option, Result
async fn get_item_id(id: u32) -> Result<Json, warp::Rejection> {
    // Crear item lists

    let mut todo_list: ItemList = ItemList::new();
    let task: Item = Item::new_item(1, String::from("hacer super"), StatusItem::Incomplete);

    todo_list.add_new(task);
    todo_list.add_new(Item::new_item(
        2,
        "tarea".to_string(),
        StatusItem::Complete(123123123),
    ));
    todo_list.add_new(Item::new_item(
        3,
        "mandados".to_string(),
        StatusItem::Incomplete,
    ));
    todo_list.add_new(Item::new_item(
        4,
        "algo".to_string(),
        StatusItem::Incomplete,
    ));

    // Buscar en el itemlist el id
    let searched_item = todo_list.list.iter().find(|item| item.id == id);

    match searched_item {
        Some(item) => {
            // transform to serde json
            let item_json =
                json!({"id": item.id, "description": item.description, "status": item.status});
            // transform to warp json
            let parse_json = warp::reply::json(&item_json);
            // Return json
            Ok(parse_json)
        }
        None => Err(warp::reject::not_found()),
    }
}

// cargo add serde --features derive
// serde_json
#[tokio::main]
async fn main() {



    let list_view_route = warp::path("todo")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_item);
    let detailed_view_route = warp::path("todo")
        .and(warp::get())
        .and(warp::path::param())
        .and_then(get_item_id);

    let routes = list_view_route.or(detailed_view_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await;


}


