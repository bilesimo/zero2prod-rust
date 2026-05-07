# rust-newsletter-api

Production-style Rust newsletter API built with Actix Web, SQLx, and PostgreSQL. This project is based on the *Zero To Production in Rust* book and follows its newsletter application architecture.

## What it does

This service exposes a small HTTP API for a newsletter workflow:

- `GET /health_check` returns `200 OK`
- `POST /subscriptions` stores a subscriber as `pending_confirmation`
- `GET /subscriptions/confirm?subscription_token=...` confirms the subscriber

When a subscription is created, the application sends a confirmation email through a configurable email provider. In production, the configuration is set up for Resend.

## Background

This repository is an implementation of the newsletter project from the *Zero To Production in Rust* book. It keeps the book's core ideas, including configuration management, database migrations, integration testing, email confirmation, and production-oriented application structure.

## Stack

- Rust 2021
- Actix Web
- SQLx + PostgreSQL
- Reqwest
- Tracing
- Docker for local Postgres

## Running locally

### 1. Start PostgreSQL and run migrations

The repo includes a helper script that starts a local Postgres container and applies migrations:

```bash
./scripts/init_db.sh
```

Requirements:

- `docker`
- `psql`
- `sqlx-cli`

Install `sqlx-cli` if needed:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

The script uses these defaults:

- `POSTGRES_USER=postgres`
- `POSTGRES_PASSWORD=password`
- `POSTGRES_DB=newsletter`
- `POSTGRES_PORT=5432`

If PostgreSQL is already running locally, skip Docker:

```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```

### 2. Start the application

```bash
cargo run
```

By default the app listens on `127.0.0.1:8000`.

## Configuration

Configuration is loaded from:

- [`configuration/base.yaml`](/Users/bilesimo/Development/zero2prod/configuration/base.yaml)
- [`configuration/local.yaml`](/Users/bilesimo/Development/zero2prod/configuration/local.yaml)
- [`configuration/production.yaml`](/Users/bilesimo/Development/zero2prod/configuration/production.yaml)

The active environment is controlled by:

```bash
APP_ENVIRONMENT=local
```

Supported values:

- `local`
- `production`

Settings can also be overridden with environment variables using the `APP_` prefix. Example:

```bash
APP_APPLICATION__PORT=9000 cargo run
```

Notable settings:

- `application.host`
- `application.port`
- `application.base_url`
- `database.host`
- `database.port`
- `database.username`
- `database.password`
- `database.database_name`
- `database.require_ssl`
- `email_client.base_url`
- `email_client.sender`
- `email_client.auth_token`
- `email_client.timeout_milliseconds`

## API

### Health check

```bash
curl -i http://127.0.0.1:8000/health_check
```

### Create a subscription

```bash
curl -i \
  -X POST \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=Le%20Guin&email=ursula_le_guin%40gmail.com" \
  http://127.0.0.1:8000/subscriptions
```

On success, the subscriber is saved with `pending_confirmation` status and a confirmation email is sent.

### Confirm a subscription

The confirmation link is generated in the email body and points to:

```text
/subscriptions/confirm?subscription_token=<token>
```

Opening that link marks the subscriber as `confirmed`.

## Testing

Run the full test suite with:

```bash
cargo test
```

The integration tests:

- create an isolated database per test
- run migrations automatically
- replace the email provider with a mock HTTP server

Enable request logs during tests with:

```bash
TEST_LOG=1 cargo test
```

## Deployment

[`spec.yaml`](/Users/bilesimo/Development/zero2prod/spec.yaml) contains a DigitalOcean App Platform definition for deploying the service and its PostgreSQL database.
