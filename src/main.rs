use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Define the Employee struct
#[derive(Serialize, Deserialize, Debug)]
struct Employee {
    id: u32,
    name: String,
    salary: f32,
}

// Helper function to initialize the RocksDB instance
fn init_db() -> Arc<DB> {
    let path = "./employee_db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open RocksDB");
    Arc::new(db)  // Return a thread-safe, shared instance of the database
}

// Route handler for storing an employee
async fn store_employee(db: web::Data<Arc<DB>>, employee: web::Json<Employee>) -> impl Responder {
    let employee = employee.into_inner();
    let id = employee.id.to_be_bytes();
    let value = bincode::serialize(&employee).expect("Failed to serialize employee");

    match db.put(id, value) {
        Ok(_) => HttpResponse::Ok().body("Employee stored successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to store employee: {}", e)),
    }
}

// Route handler for retrieving an employee
async fn get_employee(db: web::Data<Arc<DB>>, id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner().to_be_bytes();
    
    match db.get(id) {
        Ok(Some(value)) => {
            let employee: Employee = bincode::deserialize(&value).expect("Failed to deserialize employee");
            HttpResponse::Ok().json(employee)
        },
        Ok(None) => HttpResponse::NotFound().body("Employee not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to retrieve employee: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize RocksDB
    let db = init_db();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share the RocksDB instance across handlers
            .service(web::resource("/employee").route(web::post().to(store_employee)))
            .service(web::resource("/employee/{id}").route(web::get().to(get_employee)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

