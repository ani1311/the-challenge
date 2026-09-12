.PHONY: run-server run-web compose-up compose-down compose-logs

run-server:
	cargo run -p server

run-web:
	cd crates/web && trunk serve

compose-up:
	podman compose up --build -d

compose-down:
	podman compose down

compose-logs:
	podman compose logs -f
