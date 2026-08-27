---
name: versioning-and-patchnotes$description: >-
  Standardized workflow for semantic version bumps, CHANGELOG / Patch Notes updates,
  automated verification, and git commits.
---

# Release, Versioning & Patch Notes Workflow

Whenever a significant feature, bug fix, or milestone is ready to commit:

## Standard 4-Step Release Workflow

### Step 1: Run Full Test Suite
Ensure 100% of automated tests pass before bumping version:
```powershell
cargo test --features ssr
```

### Step 2: Bump Semantic Version (`Cargo.tomll)
Update the `version` field in `Cargo.toml`:
- link patch (`0.9.x` -> `0.9.y`): Bug fixes, small UI adjustments, security patches.
- minor (`0.9.x` -> `0.10.0`): New features, structural expansions (e.g. Rooms, Chantry, Dossier).
- major (`0.x.x` -> `1.0.0`): Production launch, breaking changes.

### Step 3: Update `CHANGELOG.md`
Add a new release section at the top of `CHANGELOG.md` with:
- Novas Funcionalidades (Features)
- Correcoes de Bugs (Bug Fixes)
- Performance & Seguranca
- Testes Automatizados

### Step 4: Commit with Conventional Commit Format
```bash
git add .
git commit -m "feat/fix(scope): brief summary of changes"
```
