run:
	cargo build && cargo run

install:
	go install github.com/pressly/goose/v3/cmd/goose@latest

db.migration.status:
	cd migrations && goose postgres "host=localhost user=postgres password=root dbname=postgres sslmode=disable" status

db.migration.create:
	cd migrations && goose create ${migration_name} sql

db.migration.up:
	cd migrations && goose postgres "host=localhost user=postgres password=root dbname=postgres sslmode=disable" up

db.migration.down:
	cd migrations && goose postgres "host=localhost user=postgres password=root dbname=postgres sslmode=disable" down