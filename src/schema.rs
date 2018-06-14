table! {
    chats (chat_id) {
        chat_id -> Int4,
        room_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        chat_text -> Text,
    }
}

table! {
    rooms (room_id) {
        room_id -> Int4,
        room_name -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        display_id -> Varchar,
        user_name -> Varchar,
        password_hash -> Bit,
        salt -> Varchar,
    }
}

joinable!(chats -> rooms (room_id));
joinable!(chats -> users (user_id));

allow_tables_to_appear_in_same_query!(
    chats,
    rooms,
    users,
);
