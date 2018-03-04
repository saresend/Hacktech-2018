table! {
    CurrEvent (id) {
        id -> Integer,
        name -> Text,
        lat -> Float,
        lng -> Float,
        startTime -> Text,
        endTime -> Text,
    }
}

table! {
    NewPost (id) {
        id -> Integer,
        url -> Text,
        userId -> Text,
        eventId -> Integer,
        lat -> Float,
        lng -> Float,
    }
}

joinable!(NewPost -> CurrEvent (eventId));

allow_tables_to_appear_in_same_query!(
    CurrEvent,
    NewPost,
);
