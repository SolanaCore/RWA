.PHONY: bench build clean clippy format test deploy

# Remove the first word (the target) to get positional arguments
ARG1 := $(word 2, $(MAKECMDGOALS))
ARG2 := $(word 3, $(MAKECMDGOALS))

bench:
	cargo bench --bench $(ARG1) -- $(ARG2)

# Build the program.
build:
	chio build

# Run `cargo clean`.
clean:
	cargo clean

# Run `cargo clippy`.
clippy:
	cargo clippy

# Run `cargo fmt`.
format:
	cargo fmt

# Run `chio test`
test:
	chio test

# Run `chio deploy`
deploy:
	chio deploy

# Catch-all rule to prevent errors when target is not
# match (this happens with command-line arguments)
%:
	@:
