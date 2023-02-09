-- Your SQL goes here
CREATE TABLE users (
	id VARCHAR NOT NULL PRIMARY KEY,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	session_id VARCHAR NOT NULL DEFAULT ''
);
