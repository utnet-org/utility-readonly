// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        id -> Int4,
        teacher_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        time -> Nullable<Date>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    course,
    posts,
);
