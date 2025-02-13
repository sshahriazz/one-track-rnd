use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WindowActivityData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WindowActivityData::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(
                        ColumnDef::new(WindowActivityData::ActiveWindowSs)
                            .array(ColumnType::String(StringLen::Max))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WindowActivityData::ActiveWindowData)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WindowActivityData::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WindowActivityData::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WindowActivityData::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum WindowActivityData {
    Table,
    Id,
    ActiveWindowSs,
    ActiveWindowData,
    CreatedAt,
    UpdatedAt,
}
