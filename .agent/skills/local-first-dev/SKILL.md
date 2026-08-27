---
name: local-first-dev
description: >-
  Enforces the local-first development workflow in Rust/Leptos.
  Always prefer local compilation (cargo test, cargo leptos watch, cargo check)
  for development and testing, reserving Docker builds ONLY as the final production packaging step.
---

# Local-First Development Workflow

In this project, development and testing MUST always happen directly on the host machine using native Rust tooling. Docker is strictly reserved as the final production packaging artifact.

## Core Rules

1. **Never use Docker for development iterations:**
   - Do NOT run docker compose up -d --build to test code changes, UI adjustments, or bug fixes during development.
   - Compiling in Docker takes 6-9 minutes because it lacks local caching and runs in a virtualized container.

2. **Use Local Tooling for 2-second feedback loops:**
   - **Local Dev Server & Hot-Reload:** cargo leptos watch (or .\scripts\dev.ps1).
   - **Unit & Integration Tests:** cargo test --features ssr (or .\scripts\test.ps1).
   - **Fast Compilation Check:** cargo check --features ssr (or .\scripts\check.ps1).

3. **When to use Docker:**
   - ONLY when explicitly preparing a release for the VPS or validating the final production container (.\scripts\build-prod.ps1 or .\scripts\package-vps.ps1).
