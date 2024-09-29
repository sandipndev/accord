next-watch:
	cargo watch -s 'cargo nextest run'

clean-deps:
	rm -rf .accorde
	docker compose down

start-deps:
	docker compose up -d deps

setup-db:
	cd server && cargo sqlx migrate run --ignore-missing

reset-deps: clean-deps start-deps setup-db

run-server:
	SQLX_OFFLINE=true cargo run --bin accorde-server

run-frontend:
	cd frontend && pnpm run dev
