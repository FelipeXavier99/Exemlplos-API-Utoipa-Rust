use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{Crud, Utoipa};
use utoipa_swagger_ui::swagger_ui;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct User {
    id: usize,
    username: String,
    password: String,
}

#[derive(Clone)]
struct UserStore {
    data: Mutex<Vec<User>>,
}

impl UserStore {
    fn new() -> Self {
        UserStore {
            data: Mutex::new(vec![]),
        }
    }
}

#[derive(Clone)]
struct AppState {
    user_store: UserStore,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_store = UserStore::new();
    let state = web::Data::new(AppState { user_store });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(|cfg| {
                Utoipa::new()
                    .with_prefix("/api/users")
                    .with_readiness("/readiness")
                    .into_config(cfg);
            })
            .wrap(swagger_ui(&include_str!("openapi.yaml"), None))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
