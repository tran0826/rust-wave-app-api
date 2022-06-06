table! {
    score (uuid) {
        uuid -> Text,
        stage_id -> Int4,
        clear_time -> Float4,
        user_name -> Text,
    }
}

table! {
    stage (id) {
        id -> Int4,
        difficulty -> Int4,
    }
}

joinable!(score -> stage (stage_id));

allow_tables_to_appear_in_same_query!(
    score,
    stage,
);
