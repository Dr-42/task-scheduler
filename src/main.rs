use axum::{
    response::Html,
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

    let state;
    if !Path::new("data.json").exists() {
        state = app::App::new();
    } else {
        state = app::App::load().unwrap();
        state.save()?;
    }
    let routes = Router::new()
        .route("/", get(index))
        .route("/tasks", get(get_tasks))
        .route("/modifytask", post(modify_task))
        .route("/addtask", post(add_task))
        .layer(CorsLayer::permissive());
    let router_service = routes.with_state(state).into_make_service();
    axum::Server::bind(&ip.parse()?)
        .serve(router_service)
        .await?;
    Ok(())
}

async fn get_tasks(state: axum::extract::State<app::App>) -> Json<Vec<task::Task>> {
    Json(state.get_tasks().to_vec())
}

#[derive(Deserialize, Serialize)]
struct PostTask {
    id: u64,
    action: String,
}

async fn modify_task(mut state: axum::extract::State<app::App>, body: Json<PostTask>) {
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
}

#[derive(Deserialize, Serialize)]
struct AddTask {
    name: String,
    parent: Option<u64>,
}

async fn add_task(mut state: axum::extract::State<app::App>, body: Json<AddTask>) {
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
}

async fn index() -> Html<String> {
    let body = include_str!("../static/index.html").to_string();
    Html(body)
}
