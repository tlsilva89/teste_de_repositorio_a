use axum::{routing::{get, post, put, delete}, Router, Json, extract::{Path, State}};
use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Item {
    id: i32,
    nome: String,
}

#[derive(Deserialize)]
struct CreateItem {
    nome: String,
}

#[tokio::main]
async fn main() {
    let db_url = "mysql://root:root@localhost:3306/teste_db";
    let pool = MySqlPool::connect(db_url).await.expect("Falha ao conectar no MySQL");

    sqlx::query("CREATE TABLE IF NOT EXISTS itens (id INT AUTO_INCREMENT PRIMARY KEY, nome VARCHAR(255) NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/itens", post(create_item).get(list_items))
        .route("/itens/:id", put(update_item).delete(delete_item))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_item(State(pool): State<MySqlPool>, Json(payload): Json<CreateItem>) -> Json<Item> {
    let res = sqlx::query("INSERT INTO itens (nome) VALUES (?)")
        .bind(&payload.nome)
        .execute(&pool)
        .await
        .unwrap();

    Json(Item { id: res.last_insert_id() as i32, nome: payload.nome })
}

async fn list_items(State(pool): State<MySqlPool>) -> Json<Vec<Item>> {
    let itens = sqlx::query_as::<_, Item>("SELECT * FROM itens")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(itens)
}

async fn update_item(State(pool): State<MySqlPool>, Path(id): Path<i32>, Json(payload): Json<CreateItem>) -> &'static str {
    sqlx::query("UPDATE itens SET nome = ? WHERE id = ?")
        .bind(&payload.nome)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    "Item atualizado"
}

async fn delete_item(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> &'static str {
    sqlx::query("DELETE FROM itens WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    "Item removido"
}