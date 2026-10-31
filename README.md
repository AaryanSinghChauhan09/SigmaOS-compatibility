# SigmaOS-compatibility

Compatibility and translation layer for the SigmaOS project.

This repository owns the focused compatibility and translation layer component in SigmaOS's multi-repository architecture. The crate manifest and src/ tree are the source of truth for its current API and implementation status.

## Development

Use the Rust toolchain declared by the repository. Run cargo check and the available crate tests before proposing implementation changes. Keep component boundaries explicit, document upstream behavior that informs changes, and include rollback considerations for user-visible or system-level behavior.

## Status

This overview describes repository scope; it does not claim that every planned feature is implemented or production-ready.

The IMA signature API rejects non-empty signatures because no cryptographic
verifier or trusted public key is configured. Empty inputs are accepted only
when strict mode is disabled. Do not treat this crate as an executable
signature verification boundary until a real verifier is integrated and
validated.
