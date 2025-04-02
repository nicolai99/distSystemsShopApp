#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::serde::Deserialize;
use rocket::{get, post, put, delete, http::Status, response::status};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("shop")]
struct Logs(sqlx::SqlitePool);

#[derive(Serialize)]
#[derive(Deserialize)]
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

#[post("/items", format = "json", data = "<item>")]
async fn create_or_update_item(
    mut db: Connection<Logs>,
    item: Json<Item>,
) -> Result<status::Custom<Json<Item>>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE name = ?")
        .bind(&item.name)
        .fetch_optional(&mut **db)
        .await;

    match result {
        Ok(Some(row)) => {
            let new_quantity: i32 = row.try_get("quantity").unwrap_or(0) + item.quantity;
            
            sqlx::query("UPDATE item SET quantity = ? WHERE name = ?")
                .bind(new_quantity)
                .bind(&item.name)
                .execute(&mut **db)
                .await
                .map_err(|_| status::Custom(Status::InternalServerError, "Update failed".into()))?;

            let updated_item = Item {
                id: row.try_get("id").unwrap(),
                name: item.name.clone(),
                quantity: new_quantity,
            };

            Ok(status::Custom(Status::Ok, Json(updated_item)))
        }
        Ok(None) => {
            let insert_result = sqlx::query("INSERT INTO item (name, quantity) VALUES (?, ?)")
                .bind(&item.name)
                .bind(item.quantity)
                .execute(&mut **db)
                .await;

            match insert_result {
                Ok(_) => {
                    let new_id = sqlx::query("SELECT last_insert_rowid() AS id")
                        .fetch_one(&mut **db)
                        .await
                        .map(|row| row.try_get::<i32, _>("id").unwrap_or(0))
                        .unwrap_or(0);

                    let new_item = Item {
                        id: new_id,
                        name: item.name.clone(),
                        quantity: item.quantity,
                    };

                    Ok(status::Custom(Status::Created, Json(new_item)))
                }
                Err(_) => Err(status::Custom(Status::InternalServerError, "Insert failed".into())),
            }
        }
        Err(_) => Err(status::Custom(Status::InternalServerError, "Database error".into())),
    }
}

#[put("/items/<id>", format = "json", data = "<item>")]
async fn update_item(
    mut db: Connection<Logs>,
    id: i32,
    item: Json<Item>,
) -> Result<status::Custom<Json<Item>>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    match result {
        Ok(Some(_)) => {
            let update_result = sqlx::query("UPDATE item SET name = ?, quantity = ? WHERE id = ?")
                .bind(&item.name)
                .bind(item.quantity)
                .bind(id)
                .execute(&mut **db)
                .await;

            match update_result {
                Ok(_) => Ok(status::Custom(
                    Status::Ok,
                    Json(Item {
                        id,
                        name: item.name.clone(),
                        quantity: item.quantity,
                    }),
                )),
                Err(_) => Err(status::Custom(Status::InternalServerError, "Update failed".into())),
            }
        }
        Ok(None) => Err(status::Custom(Status::NotFound, "Item not found".into())),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Database error".into())),
    }
}

#[delete("/items/<id>")]
async fn delete_item(
    mut db: Connection<Logs>,
    id: i32,
) -> Result<status::NoContent, status::Custom<String>> {
    let result = sqlx::query("DELETE FROM item WHERE id = ?")
        .bind(id)
        .execute(&mut **db)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(status::NoContent),
        Ok(_) => Err(status::Custom(Status::NotFound, "Item not found".into())),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Delete failed".into())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![get_item_by_id,items, create_or_update_item, update_item, delete_item])
}
