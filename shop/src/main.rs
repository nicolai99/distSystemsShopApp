#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::{get, http::Status, response::status};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("shop")]
struct Logs(sqlx::SqlitePool);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    id: i32,
    name: String,
    quantity: i32,
}

#[get("/items/<id>")]
async fn get_item_by_id(
    mut db: Connection<Logs>,
    id: i64,
) -> Result<Json<Item>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await;

    match result {
        Ok(row) => {
            let item = Item {
                id: row.try_get("id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                quantity: row.try_get("quantity").unwrap_or_default(),
            };

            Ok(Json(item))
        }
        Err(_) => {
            let message=String::from("Server error");
            
            Err(status::Custom(Status::InternalServerError,message))
        }
    }
}

#[get("/items")]
async fn items(mut db: Connection<Logs>) -> Result<Json<Vec<Item>>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item")
        .fetch_all(&mut **db)
        .await;

    match result {
        Ok(rows) => {
            let items: Vec<Item> = rows.into_iter().map(|row| {
                Item {
                    id: row.try_get("id").unwrap_or_default(),
                    name: row.try_get("name").unwrap_or_default(),
                    quantity: row.try_get("quantity").unwrap_or_default(),
                }
            }).collect();

            Ok(Json(items))
        }
        Err(_) => {
            let message = String::from("Item not found");
            Err(status::Custom(Status::NotFound, message))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![get_item_by_id,items])
}
