.PHONY: setup build run test e2e clean

# ============================================
# Setup
# ============================================
setup: setup-be setup-fe

setup-be:
	cargo build --all

setup-fe:
	cd frontend && npm install

# ============================================
# Database
# ============================================
db-up:
	docker compose up -d postgres minio

db-down:
	docker compose down

db-reset:
	docker compose down -v
	docker compose up -d postgres minio
	sleep 3
	$(MAKE) migrate
	$(MAKE) seed

migrate:
	cargo run -p {{project_name}}-cli -- migrate

seed:
	cargo run -p {{project_name}}-cli -- seed

# ============================================
# Development
# ============================================
run-be:
	cargo watch -x 'run -p {{project_name}}-cli -- serve'

run-fe:
	cd frontend && npm run dev

run: db-up
	$(MAKE) -j2 run-be run-fe

# ============================================
# Testing
# ============================================
test: test-be test-fe

test-be:
	cargo test --all

test-fe:
	cd frontend && npm test

e2e: db-up
	TEST_MODE=true cargo run -p {{project_name}}-cli -- serve &
	sleep 5
	cd frontend && npm run build && npm run preview &
	sleep 3
	cd e2e && npx playwright test
	pkill -f "{{project_name}}-cli" || true

e2e-ui:
	cd e2e && npx playwright test --ui

test-all: test e2e

# ============================================
# Docker
# ============================================
docker-build:
	docker compose build

docker-up:
	docker compose up -d

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

# ============================================
# Code Quality
# ============================================
fmt:
	cargo fmt --all
	cd frontend && npm run format || true

lint:
	cargo clippy --all -- -D warnings
	cd frontend && npm run lint || true

check: fmt lint test

# ============================================
# Build
# ============================================
build:
	cargo build --all
	cd frontend && npm run build

# ============================================
# Clean
# ============================================
clean:
	cargo clean
	cd frontend && rm -rf node_modules dist || true
	cd e2e && rm -rf node_modules test-results playwright-report .auth || true
