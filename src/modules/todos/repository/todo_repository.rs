use super::super::model::todo_model::*;
use crate::db::db_connection::establish_connection;
use crate::diesel::prelude::*;

pub fn find_all() -> Vec<TodoList> {
    use crate::schema::todos:: dsl::*;

    let connection = establish_connection();
    todos.limit(50)
        .order(id.asc())
        .load::<TodoList>(&connection)
        .expect("Error loading posts")
}

pub fn find(target_id: i32) -> TodoList {
    use crate::schema::todos:: dsl::*;  

    let connection = establish_connection();
    todos.find(target_id).get_result(&connection).expect("Todos not found")
}

pub fn save(todo: PutTodo) -> TodoList {
    use crate::schema::todos;

    let conn = establish_connection();

    diesel::insert_into(todos::table)
        .values(&todo)
        .get_result(&conn)
        .expect("Error saving new todo")
}

pub fn update(todo: TodoList) -> TodoList {
    use crate::schema::todos:: dsl::*;

    let conn = establish_connection();

    diesel::update(todos.find(todo.id))
        .set(&todo)
        .get_result(&conn)
        .expect("Error updating Todos")
}

pub fn delete(target_id: i32) -> usize {
    use crate::schema::todos:: dsl::*;

    let conn = establish_connection();

    diesel::delete(todos.filter(id.eq(target_id))).execute(&conn).expect("Error deleting")
}