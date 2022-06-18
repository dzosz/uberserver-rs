table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        register_date -> Timestamp,
        last_login -> Timestamp,
        last_ip -> Text,
        last_agent -> Text,
        last_sys_id -> Text,
        last_mac_id -> Text,
        ingame_time -> Integer,
        access -> Text,
        email -> Text,
        bot -> Integer,
    }
}
