-- Your SQL goes here
CREATE TABLE notifications (
  id SERIAL PRIMARY KEY,
  price FLOAT NOT NULL
);

-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  password VARCHAR(64) NOT NULL
);