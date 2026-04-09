# Flowforge

Flowforge is a production-style Rust backend for managing partner-bound order workflows.

It models a realistic business backend where orders are created internally, submitted for partner processing, tracked through lifecycle changes, and recorded with audit logs. The long-term goal of the project is to demonstrate strong backend engineering fundamentals in Rust: HTTP APIs, PostgreSQL, transactions, async workers, external API integration, tracing, and clean architecture.

## One-line pitch

A Rust backend service for managing partner-bound order workflows, with PostgreSQL persistence, async job processing, mocked external integrations, audit logging, and HTTP/gRPC interfaces.

## Project idea

An internal client or operator can:

- create an order
- view orders
- submit an order for processing
- cancel an order before processing starts

The system then:

- stores orders in PostgreSQL
- creates async jobs for submitted orders
- runs background workers
- calls a mocked external partner API
- updates order status based on the partner response
- writes audit logs for every important event

## Why this project

This project is designed to feel like a real backend system.

It showcases:

- HTTP API design
- PostgreSQL usage
- transactions
- async workers
- external API integration
- retries
- tracing and logging
- clean architecture
- gRPC
- Dockerized local development

## Current status

The repository currently implements the first vertical slice of the system:

- create an order
- persist it in PostgreSQL
- create the initial audit log
- expose health and readiness endpoints
- run integration tests against a real PostgreSQL container
- emit structured tracing logs for the create-order flow

This is an early slice of a larger workflow system. The full order lifecycle and async job processing are planned next.

## Architecture

The codebase is organized as a Rust workspace with explicit layering:

- `crates/domain` — core business entities and types
- `crates/app` — application use cases, validation, and traits
- `crates/infra` — PostgreSQL-backed implementations and infrastructure concerns
- `crates/api` — Axum routes, handlers, DTOs, and HTTP state

### Dependency direction

The intended dependency flow is:

```text
api -> app -> domain
      |
      -> infra implements app traits
```
