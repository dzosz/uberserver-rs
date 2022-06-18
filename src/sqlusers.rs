use diesel::connection::Connection;
use diesel::sql_types::Timestamp;
use diesel::sqlite::SqliteConnection;
//use diesel::sql_types::VarChar;
//use diesel::sql_query;
//use diesel::RunQueryDsl;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use crate::schema::users;
use chrono::Utc;
use chrono::NaiveDateTime;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting sqlite to {}", database_url))
}

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub register_date: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub last_ip: String,
    pub last_agent: String,
    pub last_sys_id: String,
    pub last_mac_id: String,
    pub ingame_time: i32,
    pub access: String, // user, moderator, admin, bot, agreement, fresh
    pub email: String,
    pub bot: i32,
}

impl User {
    fn new(username: String, password: String, last_ip: String, email: String) -> Self {
        Self {
            id: None,
            username: username,
            password: password,
            register_date: Utc::now().naive_utc(),
            last_login: Utc::now().naive_utc(),
            last_ip: last_ip,
            last_agent: "".into(),
            last_sys_id: "".into(),
            last_mac_id: "".into(),
            ingame_time: 0,
            access: "agreement".into(),
            email: email,
            bot: 0,
        }
    }
}

pub struct Verification {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub code: i32,
    pub expiry: NaiveDateTime,
    pub attempts: i32,
    pub resends: i32,
    pub reason: String,
}
pub struct Login {
    pub id: i32,
    pub user_id: i32,
    pub ip_address: String,
    pub time: NaiveDateTime,
    pub agent: String,
    pub last_sys_id: String,
    pub last_mac_id: String,
    pub local_ip: String,
    pub country: String,
    pub end: NaiveDateTime,
}
pub struct Bridged {
    pub id: i32,
    pub external_id: i32,
    pub location: String,
    pub external_username: String,
    pub last_bridged: NaiveDateTime,
}
pub struct Rename {
    pub id: i32,
    pub user_id: i32,
    pub original: String,
    pub time: NaiveDateTime,
}
pub struct Ignore {
    pub id: i32,
    pub user_id: i32,
    pub ignored_user_id: i32,
    pub reason: String,
    pub time: NaiveDateTime,
}
pub struct Friend {
    pub id: i32,
    pub first_user_id: i32,
    pub second_user_id: i32,
    pub time: NaiveDateTime,
}
pub struct FriendRequest {
    pub id: i32,
    pub user_id: i32,
    pub friend_user_id: i32,
    pub msg: String,
    pub time: NaiveDateTime,
}
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub owner_user_id: i32,
    pub topic: String,
    pub topic_user_id: i32,
    pub antispam: bool,
    pub censor: bool,
    pub store_history: bool,
    pub last_used: NaiveDateTime,
}
pub struct ChannelHistory {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub bridged_id: i32,
    pub time: NaiveDateTime,
    pub msg: String,
    pub ex_msg: bool,
}
struct UsersHandler {
    conn : SqliteConnection,
}
impl UsersHandler {
    fn clientFromUsername(&self, name : &str) -> Option<User> {
        use crate::schema::users::dsl::*;
        users.filter(username.eq(name)).first(&self.conn).ok()
    }
}

#[cfg(test)]
mod tests {

    // embed_migrations!("./migrations/sqlite");

    use super::*;

    #[test]
    fn test_sqlite_connection() {
        get_db();
    }

    fn get_db() -> SqliteConnection {
        let database_url = ":memory:";
        let database_url = "example.db";
        let db = establish_connection(database_url);
        // embedded_migrations::run(&db); TODO enable auto-migrations for tests?
        db.batch_execute("PRAGMA journal_mode = MEMORY; PRAGMA synchronous = OFF;")
            .unwrap();
        db
    }

    #[test]
    fn test_users() {
        let db = get_db();

        {
            let user = User::new(
                "test".into(),
                "pass".into(),
                "192.168.1.1".into(),
                "blackhole@blackhole.io".into(),
            );

            diesel::insert_into(users::table)
                .values(&user)
                .execute(&db)
                .expect("Could not add new user");
            }

        {
            let user = User::new(
                "test2".into(),
                "pass".into(),
                "192.168.1.2".into(),
                "blackhole2@blackhole.io".into(),
            );

            diesel::insert_into(users::table)
                .values(&user)
                .execute(&db)
                .expect("Could not add new user");
        }

        let handler = UsersHandler { conn : db };
        assert!(handler.clientFromUsername("test").is_some());
        assert!(handler.clientFromUsername("test2").is_some());
    }
}
