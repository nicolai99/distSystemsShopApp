#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::serde::Deserialize;
use rocket::{get, post, put, delete, http::Status, response::status};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("shop")]
struct Logs(sqlx::PgPool);

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    id: i32,
    name: String,
    quantity: i32,
}
#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ItemWithoutId {
    name: String,
    quantity: i32,
}

#[get("/test_db")]
async fn test_connection(mut db: Connection<Logs>) -> &'static str {
    // Beispiel: einfache Query (optional)
    let _rows = sqlx::query("SELECT 1")
        .fetch_all(&mut **db)
        .await
        .expect("DB Query failed");
    "Connected to Postgres!"
}
#[get("/items/<id>")]
async fn get_item_by_id(
    mut db: Connection<Logs>,
    id: i32,
) -> Result<Json<Item>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE id = $1")
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
        Err(_) => Err(status::Custom(Status::InternalServerError, "Server error".into())),
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
        Err(_) => Err(status::Custom(Status::NotFound, "Items not found".into())),
    }
}

#[post("/items", data = "<item>")]
async fn create_or_update_item(
    mut db: Connection<Logs>,
    item: Json<ItemWithoutId>,
) -> Result<status::Custom<Json<Item>>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE name = $1")
        .bind(&item.name)
        .fetch_optional(&mut **db)
        .await;

    match result {
        Ok(Some(row)) => {
            let new_quantity: i32 = row.try_get("quantity").unwrap_or(0) + item.quantity;
            let id: i32 = row.try_get("id").unwrap_or(0);
            let _ = sqlx::query("UPDATE item SET quantity = $1 WHERE name = $2")
                .bind(new_quantity)
                .bind(&item.name)
                .execute(&mut **db)
                .await
                .map_err(|_| status::Custom(Status::InternalServerError, "Update failed".into()))?;

            let response_item = Item {
                id,
                name: item.name.clone(),
                quantity: new_quantity,
            };

            Ok(status::Custom(Status::Ok, Json(response_item)))
        }
        Ok(None) => {
            let insert_result = sqlx::query("INSERT INTO item (name, quantity) VALUES ($1, $2) RETURNING id")
                .bind(&item.name)
                .bind(item.quantity)
                .fetch_one(&mut **db)
                .await;

            match insert_result {
                Ok(row) => {
                    let new_id = row.try_get("id").unwrap_or(0);
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

#[put("/items/<id>", data = "<item>")]
async fn update_item(
    mut db: Connection<Logs>,
    id: i32,
    item: Json<ItemWithoutId>,
) -> Result<status::Custom<Json<Item>>, status::Custom<String>> {
    let result = sqlx::query("SELECT * FROM item WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    match result {
        Ok(Some(_)) => {
            let update_result = sqlx::query("UPDATE item SET name = $1, quantity = $2 WHERE id = $3")
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
    let result = sqlx::query("DELETE FROM item WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(status::NoContent),
        Ok(_) => Err(status::Custom(Status::NotFound, "Item not found".into())),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Delete failed".into())),
    }
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_items() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::items)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create_item(){
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let item=r#"{"name":"Apfel","quantity":5}"#;
        let response = client.post(uri!(super::create_or_update_item)).body(item).dispatch();
        let status=response.status();
        assert!(status==Status::Created || status==Status::Ok,"Status should be 200 or 201");

    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![get_item_by_id,items, create_or_update_item, update_item, delete_item,test_connection])
}
