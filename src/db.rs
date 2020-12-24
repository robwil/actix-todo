use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> anyhow::Result<Vec<TodoList>> {
    let statement = client.prepare("select * from todo_list order by id desc").await?;
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
    let statement = client.prepare("select * from todo_item where list_id = $1 order by id asc").await?;
    let items = client
        .query(&statement, &[&list_id])
        .await?
        .iter()
        // TODO: don't love the error handling here (or lack thereof)
        .map(|row| TodoItem::from_row_ref(&row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}