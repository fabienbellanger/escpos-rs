.PHONY: help \
	upgrade \
	lint \
	lint-audit \
	audit-fix \
	test \
	check \
	clean \
	build \
	build-no-audit \
	doc \
	doc-public \
	watch-doc \
	doc-deps

.DEFAULT_GOAL=help

# Parameters
APP_NAME="POS Async API"
CURRENT_PATH=$(shell pwd)
DOCKER_COMPOSE=docker-compose
DOCKER=docker
CARGO=cargo
CARGO_BIN_NAME="pos-async-api-infrastructure"
USER_LASTNAME="Admin"
USER_FIRSTNAME="Test"
USER_EMAIL="test2@testest.com"
USER_PASSWORD="00000000"

help: Makefile
	@echo
	@echo "Choose a command run in "$(APP_NAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo

## upgrade: Upgrade crates
upgrade:
	$(CARGO) upgrade
	$(CARGO) update

## lint: Run clippy and rustfmt
lint:
	$(CARGO) fmt
	$(CARGO) clippy --all-features -- -D warnings

## lint-audit: Run clippy, rustfmt and audit
lint-audit: lint
	$(CARGO) audit

## audit-fix: Fix audit
audit-fix:
	$(CARGO) audit fix

## test: Launch unit tests in a single thread
test:
	$(CARGO) test --all-features -- --nocapture

## check: Clippy, audit and test
check: lint-audit test

## clean: Remove target directory
clean:
	$(CARGO) clean

## build: Build application in release mode
build: lint-audit test
	$(CARGO) build --release

## build-no-audit: Build application in release mode
build-no-audit: lint test
	$(CARGO) build --release

## doc: Open Rust documentation without dependencies
doc:
	$(CARGO) doc --open --no-deps

## doc-public: Open Rust documentation without dependencies
doc-public:
	$(CARGO) doc --open --document-private-items --no-deps

## watch-doc: Watch Rust documentation without dependencies
watch-doc: doc
	$(CARGO) watch -x 'doc --no-deps'

## doc: Open Rust documentation with dependencies
doc-deps:
	$(CARGO) doc --open --document-private-items
