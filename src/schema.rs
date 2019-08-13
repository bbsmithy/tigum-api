table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        note_content -> Nullable<Array<Int4>>,
    }
}

table! {
    resources (id) {
        id -> Int4,
        content_type -> Nullable<Text>,
        content -> Nullable<Text>,
        generated_by -> Nullable<Text>,
    }
}

table! {
    topics (id) {
        id -> Int4,
        title -> Varchar,
        date_created -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    notes,
    resources,
    topics,
);
