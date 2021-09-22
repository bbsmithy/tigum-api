table! {
    article_snippets (id) {
        id -> Int4,
        content -> Text,
        origin -> Text,
        date_created -> Timestamp,
        topic_id -> Int4,
        user_id -> Int4,
        title -> Nullable<Varchar>,
        date_updated -> Nullable<Timestamp>,
        published -> Nullable<Bool>,
    }
}

table! {
    code (id) {
        id -> Int4,
        topic_id -> Int4,
        language -> Nullable<Text>,
        content -> Text,
        user_id -> Int4,
        date_created -> Nullable<Timestamp>,
        origin -> Nullable<Text>,
    }
}

table! {
    images (id) {
        id -> Int4,
        topic_id -> Int4,
        user_id -> Int4,
        src -> Text,
        origin -> Text,
        date_created -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Int4,
        title -> Text,
        user_id -> Int4,
        topic_id -> Int4,
        date_created -> Timestamp,
        source -> Text,
        date_updated -> Nullable<Timestamp>,
        published -> Nullable<Bool>,
    }
}

table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        date_created -> Nullable<Timestamp>,
        topic_id -> Int4,
        user_id -> Int4,
        date_updated -> Nullable<Timestamp>,
        published -> Nullable<Bool>,
    }
}

table! {
    resources (id) {
        id -> Int4,
        content_type -> Nullable<Text>,
        content -> Nullable<Text>,
        generated_by -> Nullable<Text>,
        date_created -> Nullable<Timestamp>,
        title -> Nullable<Text>,
        thumbnail_img -> Nullable<Text>,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    topics (id) {
        id -> Int4,
        title -> Text,
        date_created -> Timestamp,
        notes -> Array<Int4>,
        videos -> Array<Int4>,
        code -> Array<Int4>,
        article_snippets -> Array<Int4>,
        links -> Array<Int4>,
        excercises -> Array<Int4>,
        user_id -> Int4,
        images -> Array<Int4>,
        date_updated -> Nullable<Timestamp>,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password_hash -> Nullable<Text>,
        email_hash -> Nullable<Int8>,
        verify_hash -> Nullable<Varchar>,
        verified -> Nullable<Bool>,
    }
}

table! {
    videos (id) {
        id -> Int4,
        title -> Text,
        iframe -> Text,
        origin -> Text,
        date_created -> Timestamp,
        thumbnail_img -> Text,
        topic_id -> Int4,
        user_id -> Int4,
        date_updated -> Nullable<Timestamp>,
        published -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    article_snippets,
    code,
    images,
    links,
    notes,
    resources,
    spatial_ref_sys,
    topics,
    users,
    videos,
);
