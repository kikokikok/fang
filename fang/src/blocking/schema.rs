// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "fang_task_state"))]
    pub struct FangTaskStateEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FangTaskStateEnum;

    fang_tasks (id) {
        id -> Uuid,
        metadata -> Jsonb,
        error_message -> Nullable<Text>,
        state -> FangTaskStateEnum,
        task_type -> Varchar,
        uniq_hash -> Nullable<Bpchar>,
        retries -> Int4,
        scheduled_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}