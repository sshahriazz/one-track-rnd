use super::m20220101_000003_create_project_table::Project;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Section::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Section::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Section::Name).string().not_null())
                    .col(
                        ColumnDef::new(Section::Version)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Section::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(Section::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Section::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_section_project")
                            .from(Section::Table, Section::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Section::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Section {
    Table,
    Id,
    Name,
    Version,
    ProjectId,
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
//     {
//         id: "proj1",
//         name: "One Track App",
//         sections: [
//             {
//                 id: "sec1",
//                 name: "Frontend",
//                 subTasks:[{
//                     id: "subtask1",
//                 }],
//                 tasks: [{
//                     subTasks: [{
//                         id: "subtask1",
//                     }]
//                 }]
//            }

//         ]
//     }
//  ]
// }
