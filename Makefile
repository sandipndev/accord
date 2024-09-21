next-watch:
	cargo watch -s 'cargo nextest run'

clean-deps:
	docker compose down

start-deps:
	docker compose up -d integration-deps

setup-db:
	cd accorde-server && cargo sqlx migrate run --ignore-missing

reset-deps: clean-deps start-deps setup-db

run-server:
	SQLX_OFFLINE=true cargo run --bin accorde-server
