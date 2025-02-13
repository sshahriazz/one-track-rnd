// src/migrator/mod.rs

use sea_orm_migration::prelude::*;

mod m20220101_000000_create_uuid_extension;
mod m20220101_000001_create_activity_data_table;
mod m20220101_000002_create_window_activity_data_table;
mod m20220101_000003_create_project_table;
mod m20220101_000004_create_section_table;
mod m20220101_000005_create_task_table;
mod m20220101_000006_create_sub_task_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000000_create_uuid_extension::Migration),
            Box::new(m20220101_000001_create_activity_data_table::Migration),
            Box::new(m20220101_000002_create_window_activity_data_table::Migration),
            Box::new(m20220101_000003_create_project_table::Migration),
            Box::new(m20220101_000004_create_section_table::Migration),
            Box::new(m20220101_000005_create_task_table::Migration),
            Box::new(m20220101_000006_create_sub_task_table::Migration),
        ]
    }
}
