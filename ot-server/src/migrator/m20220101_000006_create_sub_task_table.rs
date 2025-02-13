use super::{
    m20220101_000004_create_section_table::Section, m20220101_000005_create_task_table::Task,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubTask::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubTask::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(SubTask::Name).string().not_null())
                    .col(ColumnDef::new(SubTask::SectionId).uuid().not_null())
                    .col(ColumnDef::new(SubTask::TaskId).uuid().null())
                    .col(
                        ColumnDef::new(SubTask::Version)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubTask::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubTask::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subtask_section")
                            .from(SubTask::Table, SubTask::SectionId)
                            .to(Section::Table, Section::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subtask_task")
                            .from(SubTask::Table, SubTask::TaskId)
                            .to(Task::Table, Task::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubTask::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum SubTask {
    Table,
    Id,
    Name,
    Version,
    SectionId,
    TaskId,
    CreatedAt,
    UpdatedAt,
}
// {
//     Projects : [{
//         subTasks: [{

//         }],
//         tasks: [{
//             subTasks: [{

//             }]
//         }]
//     }]
// }

// {
//     Projects : [
// {

//         sections: [
//             {
//                 subTasks:[{}],
//                 tasks: [{
//                     subTasks: [{

//                     }]
//                 }]
//            }

//         ]
// }
//  ]
// }
