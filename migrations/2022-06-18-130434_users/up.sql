-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username VARCHAR(40) NOT NULL UNIQUE,
  password VARCHAR(64) NOT NULL,
  register_date DATETIME NOT NULL,
  last_login DATETIME NOT NULL,
  last_ip VARCHAR(15) NOT NULL,
  last_agent VARCHAR(254) NOT NULL,
  last_syc_id VARCHAR(16) NOT NULL,
  last_mac_id VARCHAR(16) NOT NULL,
  ingame_time INTEGER NOT NULL,
  access VARCHAR(32) NOT NULL,
  email VARCHAR(254) UNIQUE,
  bot INTEGER NOT NULL
)

--  body TEXT NOT NULL,
--  published BOOLEAN NOT NULL DEFAULT FALSE
