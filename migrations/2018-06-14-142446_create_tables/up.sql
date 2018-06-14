-- Your SQL goes here
CREATE TABLE rooms (
  room_id SERIAL PRIMARY KEY,
  room_name VARCHAR NOT NULL
);

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  display_id VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL,
  password_hash BIT(256) NOT NULL,
  salt VARCHAR NOT NULL
);

CREATE TABLE chats (
  chat_id SERIAL PRIMARY KEY,
  room_id INTEGER REFERENCES rooms(room_id),
  user_id INTEGER REFERENCES users(user_id),
  chat_text TEXT NOT NULL
);