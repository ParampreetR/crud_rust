use rocket::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, DeleteResult, EntityTrait};
use sea_orm::{DatabaseConnection, Set};

use crate::structs::task::ActiveModel as TaskActiveModel;
use crate::structs::task::Entity as TaskEntity;
use crate::structs::task::Model as TaskModel;

#[async_trait]
pub trait Crud {
    async fn get_by_id(&self, id: i32) -> Option<TaskModel>;
    async fn get_all(&self) -> Vec<TaskModel>;
    async fn update_by_id(&self, id: i32, task: TaskModel) -> String;
    async fn delete_by_id(&self, id: i32) -> DeleteResult;
    async fn create(&self, task: TaskModel) -> TaskModel;
}

pub struct PostgresCrudService {
    db: DatabaseConnection,
}

impl PostgresCrudService {
    pub fn new(db: DatabaseConnection) -> impl Crud {
        PostgresCrudService { db }
    }
}

#[async_trait]
impl<'a> Crud for PostgresCrudService {
    async fn get_by_id(&self, id: i32) -> Option<TaskModel> {
        let task = TaskEntity::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Unable to get tasks from Database");

        task
    }

    async fn get_all(&self) -> Vec<TaskModel> {
        let tasks = TaskEntity::find()
            .all(&self.db)
            .await
            .expect("Unable to get tasks from Database");

        tasks
    }

    async fn update_by_id(&self, id: i32, task: TaskModel) -> String {
        let task_to_update = TaskEntity::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Unable to get task from database which needs to be updated");

        if task_to_update.is_none() {
            return "Task not found to update".to_string();
        }

        let mut task_to_update: TaskActiveModel = task_to_update.unwrap().into();

        task_to_update.info = Set(task.info);
        task_to_update.title = Set(task.title);
        task_to_update.is_done = Set(task.is_done);

        task_to_update
            .update(&self.db)
            .await
            .expect("Unable to update the task");

        return "task successfully updated".to_string();
    }

    async fn delete_by_id(&self, id: i32) -> DeleteResult {
        let result = TaskEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .expect("Unable to delete element from database");
        return result;
    }

    async fn create(&self, task: TaskModel) -> TaskModel {
        let mut task_active_model: TaskActiveModel = task.into();
        task_active_model.id = ActiveValue::not_set();
        let result = task_active_model
            .insert(&self.db)
            .await
            .expect("Unable to insert task in database");

        result
    }
}
