// @generated automatically by Diesel CLI.

diesel::table! {
    sys_user (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 150]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        nickname -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        #[max_length = 11]
        phone -> Nullable<Varchar>,
        is_static -> Nullable<Bool>,
        create_at -> Timestamptz,
        update_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}
