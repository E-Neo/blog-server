table! {
    api_user (username) {
        username -> Varchar,
        salt -> Varchar,
        hash -> Varchar,
    }
}

table! {
    blog (id) {
        id -> Uuid,
        title -> Text,
        markdown -> Text,
        created_time -> Timestamp,
    }
}

table! {
    page (pagename) {
        pagename -> Varchar,
        markdown -> Text,
    }
}

table! {
    tweet (id) {
        id -> Uuid,
        markdown -> Text,
        created_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    api_user,
    blog,
    page,
    tweet,
);
