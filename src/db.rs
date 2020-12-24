use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> anyhow::Result<Vec<TodoList>> {
    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await?;
    let lists = client
        .query(&statement, &[])
        .await?
        .iter()
        // TODO: don't love the error handling here (or lack thereof)
        .map(|row| TodoList::from_row_ref(&row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(lists)
}

pub async fn get_items(client: &Client, list_id: i32) -> anyhow::Result<Vec<TodoItem>> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id asc")
        .await?;
    let items = client
        .query(&statement, &[&list_id])
        .await?
        .iter()
        // TODO: don't love the error handling here (or lack thereof)
        .map(|row| TodoItem::from_row_ref(&row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> anyhow::Result<TodoList> {
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await?;
    Ok(client
        .query(&statement, &[&title])
        .await?
        .iter()
        // TODO: don't love the error handling here (or lack thereof)
        .map(|row| TodoList::from_row_ref(&row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo list",
        ))?)
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> anyhow::Result<()> {
    let statement = client
        .prepare("update todo_item set checked=true where list_id = $1 and id = $2 and checked=false")
        .await?;
    let updated = client
        .execute(&statement, &[&list_id, &item_id])
        .await?;
    
    match updated {
        ref count if *count == 1 => Ok(()),
        _ => Err(anyhow::Error::new(io::Error::new(
            io::ErrorKind::Other,
            "Error marking item as checked",
        )))
    }
}
