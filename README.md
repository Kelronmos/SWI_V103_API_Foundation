# SWI_V103_API_Foundation

**Status: LEGACY / STUB — not the active SWI V1→V2 evidence path**

Rust-oriented **historical / experimental** API slice (older “V1.03” naming).

## Discipline

```text
Claim → Implementation → Test → Result → Limitation
TESTED ≠ SEALED ≠ PRODUCTION READY
```

| Doc | Purpose |
|-----|---------|
| [docs/STATUS.md](docs/STATUS.md) | What exists vs claims |
| [docs/LIMITATIONS.md](docs/LIMITATIONS.md) | Open gaps |
| [docs/CONNECTION_REQUIREMENTS.md](docs/CONNECTION_REQUIREMENTS.md) | Real V1/V2 link requirements |
| [docs/ARCHITECTURE_POSITION.md](docs/ARCHITECTURE_POSITION.md) | Generational map |

## In this repository

| Artifact | Reality |
|----------|---------|
| `src/api.rs` | Minimal `ApiResponse` / `ApiMeta` |
| `SWI_V103_API_Foundation.zip` | Incomplete Axum scaffold (missing path crates) |
| Active foundation | [SWI-V1-Module-1-10](https://github.com/Kelronmos/SWI-V1-Module-1-10) |
| Active V2 / M11 | [SWI-V2-Modules-11-22](https://github.com/Kelronmos/SWI-V2-Modules-11-22) |

## Non-claims

Does **not** establish: governance-ready production API, orchestration, security middleware, audit trails, Firefly/storage integration, or M11-compatible admission. Hardcoded demo fields are not trust scores.

See `LICENSE`.
