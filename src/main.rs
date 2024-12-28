use std::path::PathBuf;

use crud_rust::{handlers::crud_handler::*, services::crud_service::PostgresCrudService, utils};
use rocket::{catchers, routes};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().unwrap_or_else(|err| {
        println!("{:?}", &err);
        if err.not_found() {}

        PathBuf::new()
    });
    let db = utils::db::set_up_db()
        .await
        .expect("Unable to connect to database");
    rocket::build()
        .configure(rocket::Config {
            port: str::parse(
                std::env::var("SERVER_PORT")
                    .expect("Unable to get SERVER_PORT from env")
                    .as_str(),
            )
            .unwrap(),

            ..rocket::Config::debug_default()
        })
        .manage(PostgresCrudService::new(db))
        .mount(
            "/v1/task",
            routes![
                add_task,
                update_by_id,
                delete_by_id,
                get_all_tasks,
                get_task_by_id
            ],
        )
        .register("/", catchers![default])
        .launch()
        .await?;
    Ok(())
}
