.PHONY: format lint test deploy
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build-docker:
	docker build -t actix-microservice:latest .

all: format lint test build-docker