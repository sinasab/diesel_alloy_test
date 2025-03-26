-- Your SQL goes here
CREATE TABLE "users"(
	"addr" BYTEA NOT NULL PRIMARY KEY,
	"sig" BYTEA NOT NULL,
	"hash" BYTEA NOT NULL,
	"maybe_hash" BYTEA
);

