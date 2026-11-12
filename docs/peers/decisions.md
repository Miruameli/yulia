# Decisions Mirror

Mirror of `pi_messenger({action:"team.memory.note", type:"decision"})` entries for PR review and human audit.
Primary channel is team.memory; this file is the audit trail.

Format per entry:
- **Timestamp (UTC):** YYYY-MM-DD HH:MM
- **Author:** <agent name>
- **Type:** decision | interface | risk | handoff
- **Status:** open | agreed | superseded | resolved
- **Affected agents:** <comma-separated live agent names at time of writing>
- **Decision:** <one paragraph>
- **Evidence:** <codegraph, internet URL+version+date, tests, references>
- **Version marker (if contract change):** vX.Y.Z or section hash
- **team.memory entry:** <pointer or hash>
- **Supersedes / superseded by:** <link or "—">

> **Note on claims:** v6.0 removes `docs/peers/claims.md`. Resource claims are enforced by
> `pi_messenger({action:"reserve"|"release"})` via a `tool_call` hook, so no claims file is kept.

---

## 2026-11-12 18:05 — SageBear

- **Timestamp (UTC):** 2026-11-12 18:05
- **Author:** SageBear
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, HappyHawk, DarkStorm
- **Decision:** GHOP compliance applied (PR #12, merge commit 80b1e3d): added `.github/dependabot.yml` (weekly Cargo + GitHub Actions version updates), set squash-only merge options (`allow_merge_commit=false`, `allow_rebase_merge=false`), `delete_branch_on_merge=true`, and enabled Dependabot security updates; `docs/operations/README.md` §Repository settings records both. Deleted the stale branches `docs/peers-v5-scaffold` (superseded by PR #9) and `chore/v4-repo-compliance` (already in main). Merged Dependabot PR #13 (actions/checkout 4.2.2 -> 7.0.1, merge commit 6a16aaa). Two corrections to the earlier scan: `required_approving_review_count=0` and `require_code_owner_reviews=false` are deliberate (single maintainer account; documented in the runbook), and `allowed_actions=all` is not a v6.0 requirement. Remaining: wiki is ON (SOP-GHOP.1 says disable) — pending the human owner.
- **Evidence:** `gh pr view 12` (merged, 80b1e3d); `gh api repos/Miruameli/yulia` (merge settings + `security_and_analysis.dependabot_security_updates.status=enabled`); `git push origin --delete docs/peers-v5-scaffold chore/v4-repo-compliance`; `gh pr view 13` (merged, 6a16aaa); `docs/operations/README.md` §Repository settings.
- **Version marker:** —
- **team.memory entry:** "GHOP compliance (SageBear, 2026-11-12 18:00 UTC)"
- **Supersedes / superseded by:** —

## 2026-11-12 17:18 — SageBear

- **Timestamp (UTC):** 2026-11-12 17:18
- **Author:** SageBear
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, HappyHawk, DarkStorm
- **Decision:** Contract v6.0 (pi-messenger) is adopted. The repository `KONTRAK.md` was v4.0 and its `coord_*` tools (`coord_status`, `coord_send`, `coord_note`) are no longer mounted, while the user issued v6.0 on 2026-11-12. Work split: HappyHawk owns `KONTRAK.md` v4.0 -> v6.0; SageBear owns this `docs/peers/decisions.md` mirror and the reconciliation of the `docs/peers-v5-scaffold` branch; DarkStorm owns issue #7 (codegen LLVM backend); NiceArrow owns issue #8 (driver CLI/tests/CI/docs). `docs/peers/claims.md` is intentionally not created (v6.0 replaces file claims with enforced reservations).
- **Evidence:** `pi_messenger` list snapshots 2026-11-12 17:12/17:16/17:18 (4 live agents); `git log --oneline -3` (main = bae5537); `xd://coord_status` absent (mounted devices: security_scan, ast_grep, ast_edit, debug, github, lsp, checkpoint, rewind, pi_messenger, generate_image, tts); user message 2026-11-12 carrying the v6.0 text; `git show origin/docs/peers-v5-scaffold` (65691ef).
- **Version marker:** v6.0 (contract)
- **team.memory entry:** "Contract v6.0 adoption + work split (SageBear, 2026-11-12 17:18 UTC)"
- **Supersedes / superseded by:** Supersedes the open contract-drift entries below.

## 2026-11-12 16:41 — adit

- **Timestamp (UTC):** 2026-11-12 16:41
- **Author:** adit
- **Type:** risk
- **Status:** superseded
- **Affected agents:** yuliac-11441, yuliac-11555, yuliac-20845, yuliac-20932
- **Decision:** Contract-version drift flagged, not resolved: (a) repository `KONTRAK.md` = v4.0; (b) peer branch `docs/contract-v4.1` = v4.1 (uncommitted, `coord_*` + broadcast-first); (c) user-issued v5.0 (omp-peers). v5.0 supersedes v4.1 by the user's authority, but the v4.1 edit was in flight and the session was prohibited from messaging peers, so the conflict could not be resolved by `peer_send`.
- **Evidence:** user message 2026-11-12 (v5.0 text); `/root/.omp/wt/contract-v4.1-yuliac/KONTRAK.md:1` ("# AI Operating Contract (v4.1)"); `KONTRAK.md:1` at main ("# AI Operating Contract (v4.0)").
- **Version marker:** v5.0 vs v4.1
- **team.memory entry:** — (pre-pi-messenger)
- **Supersedes / superseded by:** Superseded by the 2026-11-12 17:18 v6.0 adoption entry.

## 2026-11-12 16:40 — adit

- **Timestamp (UTC):** 2026-11-12 16:40
- **Author:** adit
- **Type:** handoff
- **Status:** superseded
- **Affected agents:** yuliac-11441, yuliac-11555, yuliac-20845, yuliac-20932
- **Decision:** `docs/peers/decisions.md` created to satisfy the v5.0 contract (SOP-TEMPLATES.18). It did not exist at `main` (bae5537). Session-start roster 2026-11-12 16:34 showed four live peers; a peer worktree `/root/.omp/wt/contract-v4.1-yuliac` held an uncommitted v4.0 -> v4.1 edit; a second worktree `/root/dev/yuliac-1b-driver` (branch `feat/1b-driver-tests`) was clean and corresponded to issue #8.
- **Evidence:** `git log --oneline -3` (main = bae5537); `gh issue list --state open` (#7, #8); `gh run list --limit 10` (all success); user message 2026-11-12 carrying the v5.0 text.
- **Version marker:** v5.0 (contract)
- **team.memory entry:** — (pre-pi-messenger)
- **Supersedes / superseded by:** Superseded by the 2026-11-12 17:18 v6.0 adoption entry.
