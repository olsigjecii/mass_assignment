use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//-----------------------------------------------------------------------------
// 1. Data Models & In-Memory "Database"
//-----------------------------------------------------------------------------

/// Represents our in-memory storage.
type Db = Arc<DashMap<String, User>>;

/// The full User model, including sensitive fields.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    password: String, // In a real app, this should be a hash!
    email: String,
    role: String,
    organization: String,
}

/// The Data Transfer Object (DTO) for creating a user securely.
#[derive(Deserialize, Debug)]
struct CreateUserDto {
    username: String,
    password: String,
    email: String,
}

//-----------------------------------------------------------------------------
// 2. Route Handlers
//-----------------------------------------------------------------------------

/// VULNERABLE user creation handler.
async fn create_user_vulnerable(db: web::Data<Db>, user_data: web::Json<User>) -> impl Responder {
    let new_user = user_data.into_inner();

    if db.contains_key(&new_user.email) {
        return HttpResponse::Conflict().body("User with this email already exists.");
    }
    db.insert(new_user.email.clone(), new_user.clone());
    println!("[VULNERABLE] Created user: {:?}", new_user);
    HttpResponse::Created().json(new_user)
}

/// SECURE user creation handler.
async fn create_user_secure(
    db: web::Data<Db>,
    user_dto: web::Json<CreateUserDto>,
) -> impl Responder {
    if db.contains_key(&user_dto.email) {
        return HttpResponse::Conflict().body("User with this email already exists.");
    }
    let new_user = User {
        username: user_dto.username.clone(),
        password: user_dto.password.clone(),
        email: user_dto.email.clone(),
        role: "user".to_string(),
        organization: "default_org".to_string(),
    };
    db.insert(new_user.email.clone(), new_user.clone());
    println!("[SECURE] Created user: {:?}", new_user);
    HttpResponse::Created().json(new_user)
}

//-----------------------------------------------------------------------------
// 3. Main Application Setup
//-----------------------------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // **FIXED**: Explicitly define the key and value types for the DashMap.
    let db = web::Data::new(Arc::new(DashMap::<String, User>::new()));

    println!("🚀 Server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                web::scope("/vulnerable")
                    .route("/user/create", web::post().to(create_user_vulnerable)),
            )
            .service(
                web::scope("/secure").route("/user/create", web::post().to(create_user_secure)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
