use app::App;
use axum::{
    body::{self, Full},
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{env, path::Path};
use tower_http::cors::CorsLayer;

mod app;
mod duration;
mod task;
mod time;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let mut port = 8080;
    if args.len() == 2 {
        port = args[1].parse::<u16>()?;
    } else {
        println!("Usage: {} <port>", args[0]);
    }

    let ip = format!("0.0.0.0:{}", port);

    let mut state;
    if !Path::new("data.json").exists() {
        state = app::App::new();
        let t1 = state.add_task("Task 1".to_string()).unwrap();
        let t2 = state.add_task("Task 2".to_string()).unwrap();
        let t3 = state.add_task("Task 3".to_string()).unwrap();
        let t4 = state.add_subtask(t1, "Task 1.1".to_string()).unwrap();
        let t5 = state.add_subtask(t1, "Task 1.2".to_string()).unwrap();
        let t6 = state.add_subtask(t2, "Task 2.1".to_string()).unwrap();
        let t7 = state.add_subtask(t4, "Task 1.1.1".to_string()).unwrap();
        let _ = state.start_task(t3);
        let _ = state.start_task(t5);
        let _ = state.stop_task(t5);
        state.save()?;
    } else {
        state = app::App::load().unwrap();
        state.save()?;
    }
    let routes = Router::new()
        .route("/", get(index))
        .route("/index.js", get(get_js))
        .route("/index.css", get(get_css))
        .route("/favicon.ico", get(get_favicon))
        .route("/tasks", get(get_tasks))
        .route("/modifytask", post(modify_task))
        .route("/addtask", post(add_task))
        .layer(CorsLayer::permissive());
    let router_service = routes.into_make_service();
    axum::Server::bind(&ip.parse()?)
        .serve(router_service)
        .await?;
    Ok(())
}

async fn get_tasks() -> Json<Vec<task::Task>> {
    let state = App::load().unwrap();
    Json(state.get_tasks().to_vec())
}

#[derive(Deserialize, Serialize)]
struct PostTask {
    id: u64,
    action: String,
}

async fn modify_task(body: Json<PostTask>) -> impl IntoResponse {
    let mut state = App::load().unwrap();
    let tasks = state.get_tasks();
    let task = tasks
        .iter()
        .find(|t| t.get_id() == body.id)
        .unwrap()
        .get_id();
    match body.action.as_str() {
        "start" => {
            state.start_task(task).unwrap();
        }
        "stop" => {
            state.stop_task(task).unwrap();
        }
        _ => {}
    }
    state.save().unwrap();
    println!("{} {}ed", body.id, body.action);
    Response::builder()
        .status(StatusCode::OK)
        .body("".to_string())
        .unwrap()
}

#[derive(Deserialize, Serialize, Debug)]
struct AddTask {
    name: String,
    parent: Option<u64>,
}

async fn add_task(body: Json<AddTask>) -> impl IntoResponse {
    let mut state = App::load().unwrap();
    println!("{:?}", body);
    let parent = body.parent;
    let name = &body.name;
    if parent.is_some() {
        let parent = parent.unwrap();
        state.add_subtask(parent, name.to_string()).unwrap();
    } else {
        state.add_task(name.to_string()).unwrap();
    }
    state.save().unwrap();
    println!("Added task {}", name);
    Response::builder()
        .status(StatusCode::OK)
        .body("".to_string())
        .unwrap()
}

async fn index() -> Html<String> {
    //let body = include_str!("../static/index.html").to_string();
    let file = std::fs::read_to_string("static/index.html").unwrap();
    Html(file)
}

async fn get_js() -> impl IntoResponse {
    let m = "text/javascript";
    let content = std::fs::read_to_string("static/index.js").unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(&m).unwrap(),
        )
        .body(content)
        .unwrap()
}

async fn get_css() -> impl IntoResponse {
    let m = "text/css";
    let content = std::fs::read_to_string("static/index.css").unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(&m).unwrap(),
        )
        .body(content)
        .unwrap()
}

async fn get_favicon() -> impl IntoResponse {
    let m = "image/x-icon";
    let body = include_bytes!("../static/favicon.ico").to_vec();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_str(&m).unwrap(),
        )
        .body(body::boxed(Full::from(body)))
        .unwrap()
}
