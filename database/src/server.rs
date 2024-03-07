use crate::controllers;

use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use tower_http::trace::TraceLayer;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{delete, get, patch, post},
    Extension, Router,
};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn db() {
    // Create DB if not exists
    let db_url: String = db_file_path();
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool: sqlx::Pool<Sqlite> = SqlitePool::connect(&db_url)
        .await
        .expect("Connect to Sqlite");

    _ = controllers::user::init::init(pool.clone()).await;
    _ = controllers::conversation::init::init(pool.clone()).await;
    _ = controllers::engagement::init::init(pool.clone()).await;

    // allow CORS from any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::PATCH,
            Method::POST,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        // User
        .route("/users", get(controllers::user::get::get_users))
        .route("/user", post(controllers::user::post::add_user))
        .route("/user/id/:id", get(controllers::user::get::get_user_by_id))
        .route(
            "/user/name/:name",
            get(controllers::user::get::get_user_by_name),
        )
        .route(
            "/users/active/:activity",
            get(controllers::user::get::get_users_by_activity),
        )
        .route("/user/id/:id", patch(controllers::user::patch::update_user))
        .route(
            "/user/id/:id",
            delete(controllers::user::delete::delete_user_by_id),
        )
        .route(
            "/user/name/:name",
            delete(controllers::user::delete::delete_user_by_name),
        )
        // Conversation
        .route(
            "/conversation/:user_id",
            post(controllers::conversation::post::add_conversation),
        )
        .route(
            "/conversations",
            get(controllers::conversation::get::get_conversations),
        )
        .route(
            "/conversation/id/:id",
            get(controllers::conversation::get::get_conversation_by_id),
        )
        .route(
            "/conversations/user_id/:user_id",
            get(controllers::conversation::get::get_conversations_by_user_id),
        )
        .route(
            "/conversation/update/:id",
            patch(controllers::conversation::patch::update_conversation),
        )
        .route(
            "/conversation/id/:id",
            delete(controllers::conversation::delete::delete_conversation_by_id),
        )
        // Engagement
        .route(
            "/engagement",
            post(controllers::engagement::post::add_engagement),
        )
        .route(
            "/engagements",
            get(controllers::engagement::get::get_engagements),
        )
        .route(
            "/engagement/id/:id",
            get(controllers::engagement::get::get_engagement_by_id),
        )
        .route(
            "/engagements/conversation/:conversation_id",
            get(controllers::engagement::get::get_engagements_by_conversation_id),
        )
        .route(
            "/engagement/update/:id",
            patch(controllers::engagement::patch::update_engagement),
        )
        .route(
            "/engagement/id/:id",
            delete(controllers::engagement::delete::delete_engagement_by_id),
        )
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Serve
    let localhost = "127.0.0.1";
    let database_url = std::option_env!("FIRESIDE_DATABASE_URL")
        .unwrap_or(localhost)
        .to_string();
    let port = 16980;
    let tcp_string = format!("{}:{}", database_url, port);

    let listener = TcpListener::bind(tcp_string).await.unwrap();
    println!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}

use tauri::{api::path::app_config_dir, Config};

pub fn db_file_path() -> String {
    let config_dir = app_config_dir(&Config::default()).expect("load tauri config");
    let fireside = "fireside-chat".to_string();
    let config_dir_path = config_dir.join(fireside);
    _ = std::fs::create_dir_all(&config_dir_path);
    format!("sqlite://{}/sqlite.db", config_dir_path.display())
}
