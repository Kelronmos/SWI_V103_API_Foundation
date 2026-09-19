# Connecting V103 to V1 / V2 — Requirements (not met)

**Status:** REQUIREMENTS ONLY — NOT AUTHORIZED for implementation until contracts are approved.

## Principle

A README link is not a connection. Connection requires contract, code, tests, and CI without weakening V1/V2 seals.

## Possible roles (if authorized later)

| Role | Must not |
|------|----------|
| HTTP transport of admitted results | Create authority |
| Health / version | Fake security scores |
| Serialized evidence for V2 admit | Skip M11 |

## Forbidden shortcuts

Treat V103 JSON as authorized · use security_standing as trust · assume Firefly from Cargo names · bypass M11 · feed V103 into require_admitted without a new contract.

## Minimum evidence for CONNECTED

1. Versioned contract  
2. Buildable crate (no missing deps)  
3. Unit + negative tests  
4. Authority non-expansion tests  
5. Optional V1/V2 evidence compatibility  
6. CI on pinned toolchain  
7. Status docs; SEALED only via sealing manual  

```text
CONNECTION_STATE = NOT CONNECTED
```

```text
Client → V103 (transport only) → V1/V2 boundary → existing admission
```
