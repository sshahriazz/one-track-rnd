use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ActivityData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ActivityData::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(
                        ColumnDef::new(ActivityData::Screenshots)
                            .array(ColumnType::String(StringLen::Max))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::KeyboardActivityPercent)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::MouseActivityPercent)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::TotalPercent)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::TrackInterval)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::StartTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::EndTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivityData::UpdatedAt)
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
            .drop_table(Table::drop().table(ActivityData::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ActivityData {
    Table,
    Id,
    Screenshots,
    KeyboardActivityPercent,
    MouseActivityPercent,
    TotalPercent,
    TrackInterval,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
