use std::borrow::Borrow;

use rocket::{http::Status, response::status, serde::json::Json, Request, State};

use crate::{
    services::crud_service::{Crud, PostgresCrudService},
    structs::task::Model,
};

#[rocket::get("/<id>")]
pub async fn get_task_by_id(
    id: i32,
    crud_service: &State<PostgresCrudService>,
) -> Option<Json<Model>> {
    let task = crud_service.borrow().get_by_id(id).await;

    if task.is_none() {
        return None;
    } else {
        return Some(Json(task.unwrap()));
    }
}

#[rocket::get("/")]
pub async fn get_all_tasks(crud_service: &State<PostgresCrudService>) -> Json<Vec<Model>> {
    let tasks = crud_service.borrow().get_all().await;

    return Json(tasks);
}

#[rocket::post("/", data = "<task>")]
pub async fn add_task(
    mut task: Json<Model>,
    crud_service: &State<PostgresCrudService>,
) -> Json<Model> {
    task.id = 0;
    let created_task = crud_service.borrow().create(task.into_inner()).await;
    return Json(created_task);
}

#[rocket::delete("/<id>")]
pub async fn delete_by_id(
    id: i32,
    crud_service: &State<PostgresCrudService>,
) -> status::Custom<Json<String>> {
    let result = crud_service.borrow().delete_by_id(id).await;

    if result.rows_affected > 0 {
        status::Custom(Status::Accepted, Json("Deleted successfully".to_string()))
    } else {
        status::Custom(
            Status::NoContent,
            Json("Task not found to delete".to_string()),
        )
    }
}

#[rocket::put("/<id>", data = "<task>")]
pub async fn update_by_id(
    id: i32,
    task: Json<Model>,
    crud_service: &State<PostgresCrudService>,
) -> String {
    let result = crud_service
        .borrow()
        .update_by_id(id, task.into_inner())
        .await;

    return result;
}

#[rocket::catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
