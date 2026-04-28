# Contributing to CAD Open Layer

Thanks for considering a contribution. CAD Open Layer is governed by a strict clean-room implementation policy because of the IP-sensitive nature of the CAD file format domain. This document tells you how to contribute.

## Before your first contribution

1. **Read the legal package.** Especially `legal-package/01-clean-room-policy.md` and `legal-package/03-no-gpl-contamination-policy.md`.
2. **Sign a contributor declaration.** Copy `legal-package/04-contributor-declaration-template.md` into `legal-package/contributors/{your-github-username}.md`, fill it in, and submit it as your **first PR** (separate from any code change).
3. **Wait for declaration merge.** Code PRs will be blocked by CI until your declaration is registered.

## Critical IP rules

- **No GPL code.** No copy, translation, structural mimicry, or algorithm porting from LibreDWG or any GPLv2/GPLv3/AGPL project.
- **No proprietary SDK.** Do not access Autodesk RealDWG, ODA Teigha, or any NDA-protected DWG material.
- **Reference materials must be logged.** Add to `legal-package/02-reference-material-log.md` before using new external sources.
- **Trademark compliance.** "DWG", "AutoCAD", "Autodesk" are referential only. Never product/domain/repo branding. See `legal-package/05-trademark-usage-guideline.md`.

CI will block PRs that fail clean-room checks. False positives can be overridden with `GPL-OVERRIDE: <reason>` in the commit message after maintainer review.

## Development workflow

```bash
# Toolchain (rust-toolchain.toml will pin automatically)
rustup show

# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --check
cargo fmt   # auto-fix
```

## Pull request expectations

- One logical change per PR. Keep diffs small and reviewable.
- Tests for new behavior. New algorithms come with unit tests + a property test where relevant.
- Reference material citations in commit messages where applicable.
- All CI checks pass before requesting review.

## Test corpus

- New test DXF files must be registered in `legal-package/06-test-corpus-license-log.md` in the same PR.
- Synthetic files preferred. License-restricted files (NDA, copyrighted) must not be committed to the public repo.

## Communication

- Bug reports / feature requests: GitHub Issues.
- Security issues: see `SECURITY.md` (private disclosure).
- Architecture questions: GitHub Discussions or design RFC PRs in `docs/rfcs/`.

## Code of conduct

Be direct, technical, and respectful. Personal attacks, harassment, or off-topic conflicts will not be tolerated.
