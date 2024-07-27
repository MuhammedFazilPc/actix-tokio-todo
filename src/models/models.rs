//src/models/models.rs
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub checked:bool
}
pub async fn add_item(client: &Client, title: &str, description: &str) -> Result<TodoItem, Error> {
    let row: tokio_postgres::Row=client
            .query_one(
            "INSERT INTO todo_items (title, description) VALUES ($1, $2) RETURNING id, title, description,checked",
            &[&title, &description])
            .await ?;

    Ok(TodoItem {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        checked: row.get(3),
    })
}
pub async fn get_all_items(client: &tokio_postgres::Client) -> Result<Vec<TodoItem>, Error> {
    match client
        .query("SELECT id, title, description,checked FROM todo_items", &[])
        .await
    {
        Ok(rows) => {
            info!("Successfully retrieved rows: {:?}", rows);
            let items: Vec<TodoItem> = rows
                .iter()
                .map(|row| TodoItem {
                    id: row.get(0),
                    title: row.get(1),
                    description: row.get(2),
                    checked: row.get(3),
                })
                .collect();
            Ok(items)
        }
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}
pub async fn update_item(
    client: &tokio_postgres::Client,
    id: i32,
) -> Result<(), Error> {
    client
        .execute(
            "UPDATE todo_items SET checked=true WHERE id = $1",
            &[ &id]
        )
        .await?;
    Ok(())
}
pub async fn delete_item(
    client: &tokio_postgres::Client,
    id: i32
)->Result<(),Error>{
    client.execute(
        "DELETE FROM todo_items WHERE id=$1", &[&id]
    ).await ?;
      Ok(())
}