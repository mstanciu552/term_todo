table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        created_at -> Nullable<Date>,
        until_at -> Nullable<Date>,
        in_progress -> Bool,
    }
}
