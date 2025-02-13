use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter,
};

use crate::entities::{prelude::*, *};

pub struct SubTaskCreateData {
    pub name: String,
}

pub async fn create_sub_task_for_task_and_section(
    db: &DatabaseConnection,
    data: SubTaskCreateData,
    section_id: Uuid,
    task_id: Option<Uuid>,
) -> Result<sub_task::Model, DbErr> {
    let sub_task = sub_task::ActiveModel {
        name: ActiveValue::set(data.name),
        task_id: ActiveValue::set(task_id),
        section_id: ActiveValue::set(section_id),
        ..Default::default()
    };

    let result = SubTask::insert(sub_task).exec_with_returning(db).await?;
    Ok(result)
}
