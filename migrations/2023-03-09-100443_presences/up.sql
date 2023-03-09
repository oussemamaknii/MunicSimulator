-- Your SQL goes here
CREATE TABLE presences (
  id SERIAL PRIMARY KEY,
  id_str VARCHAR NOT NULL,
  msg_type VARCHAR NOT NULL,
  reason TEXT NOT NULL,
  asset TEXT NOT NULL,
  time TEXT NOT NULL
)