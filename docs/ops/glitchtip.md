<!--
File: docs/ops/glitchtip.md
Purpose: Product-side GlitchTip DSN contract (placeholders + op:// only).
Related: .env.example, .env.op.example, sibling devao docs/governance/runtime-obs-glitchtip-setup.md
Tags: #docs #ops #glitchtip #obs #secrets
-->

# GlitchTip DSN (BITSCROW)

BITSCROW is the first live GlitchTip trial for DEVAO runtime obs. Spine work
already shipped the offline envelope stub and HITL notes; **this repo** owns
the env contract and (after human go) the live projects.

Shop standard (sibling `devao`):

- `docs/governance/runtime-obs.md`
- `docs/governance/runtime-obs-glitchtip-setup.md`
- `docs/policies/secret-injection.md`

## Env contract

| Variable | Role |
|----------|------|
| `GLITCHTIP_DSN_LOCAL` | Workstation / local dogfood |
| `GLITCHTIP_DSN_STAGING` | Staging tracker project |
| `GLITCHTIP_DSN_PROD` | Prod tracker project |

Three vendor projects only (`local`, `staging`, `prod`). No fourth "VM"
tracker project. Prefer GlitchTip; hosted `sentry.io` needs a Buzz-first
`comms_exception` before any SaaS traffic.

## What is committed

- `.env.example` — names + angle-bracket placeholders (never a live DSN).
- `.env.op.example` — `op://` vault path **shapes** only.

Copy `.env.op.example` → gitignored `.env.op`, then:

```bash
op run --env-file=.env.op -- <product command that redacts output>
```

CI and Cloud Agents stay keyless. Do not put plaintext DSN (or `op://` refs)
into chalkboard, traces, RuntimeEvent payloads, or chat.

## Stephen must supply (blocked until human go)

1. Create GlitchTip projects: `local`, `staging`, `prod` (self-host or
   compatible host — Stephen chooses).
2. Create three 1Password items under vault `Personal` (or update the path
   shapes in `.env.op` if the vault/item names differ):
   - `BITSCROW GlitchTip local` → field `credential`
   - `BITSCROW GlitchTip staging` → field `credential`
   - `BITSCROW GlitchTip prod` → field `credential`
3. Paste each project's real DSN into the matching vault field (never into
   git).
4. Record a chalkboard `decision` (human author) naming env + vault item
   path shape before treating any `op://` tracker ref as live.
5. Approve scrub-before-send rules in the eventual product client (secrets,
   tokens, cookies, Bitcoin locators, user ids). Capture-back to the shop is
   RuntimeEvent only: `title` + `fingerprint` + `count`.

## Not in this slice

- No Sentry/GlitchTip SDK crate yet (no app binary that would init a client).
- No network send from CI or agents.
- Spine `stub_send` remains offline (`sent: false`).
