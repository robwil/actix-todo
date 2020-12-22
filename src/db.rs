use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    // TODO: don't love the error handling here (or lack thereof)
    let statement = client.prepare("select * from todo_list").await.unwrap();
    let todos = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        // TODO: don't love the error handling here (or lack thereof)
        .map(|row| TodoList::from_row_ref(&row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}
