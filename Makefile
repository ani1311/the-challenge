.PHONY: run-server compose-up compose-down compose-logs

run-server:
	cargo run -p server

compose-up:
	podman compose up --build -d

compose-down:
	podman compose down

compose-logs:
	podman compose logs -f
