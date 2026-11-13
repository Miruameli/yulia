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

## 2026-11-13 00:20 — YoungGrove (agreed by WildWolf, SageTiger)

- **Timestamp (UTC):** 2026-11-13 00:20
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, WildWolf, SageTiger
- **Decision:** Sequencing for the Phase 1b LLVM build prerequisite. Verified that `main`'s `.github/workflows/ci.yml` carried no LLVM handling while the GitHub Ubuntu 24.04 runner image ships Clang 16/17/18 only (no LLVM 20, no `llvm-config`), and that the only LLVM 20 installer (`.github/actions/install-llvm20/action.yml`) existed solely on `feat/8-driver-cli`. Consequence: `#7` (which adds `inkwell`) could neither get green checks nor land without turning `main` CI red. Agreed order: split the CI wiring out of `#16` into `#20` (`ci/llvm20-prereq`) and land it first; SageTiger then rebases `#7` onto the new `main` so its branch `ci.yml` carries the install step; then `#7`; then WildWolf rebases `#16` and drops its now-redundant CI commits; `#19` last, since it links the action file `#20` creates. `#20` merged as `3f4b619`. Durable lesson from the two failed `#20` runs: `timeout-minutes` is a workflow-JOB key and is rejected on composite-action steps (`Unexpected value` / `TemplateValidationException`), so per-step bounds belong inside the script (`timeout 600 …`).
- **Evidence:** `gh api repos/Miruameli/yulia/contents/.github/workflows/ci.yml` (no llvm refs on `main`; action 404 on `main`, 200 on `feat/8-driver-cli`); `actions/runner-images` `images/ubuntu/Ubuntu2404-Readme.md` — Clang 16.0.6 / 17.0.6 / 18.1.3, retrieved 2026-11-13; run 43338357179 (`timeout-minutes` schema rejection) and run 43340967176 (all 7 green on `f1a50fa`); `main` CI on `3f4b619` success.
- **Version marker:** —
- **team.memory entry:** "DECISION 2026-11-13 00:40 UTC (YoungGrove)" and "DECISION 2026-11-13 01:00 UTC (YoungGrove)".
- **Supersedes / superseded by:** —

## 2026-11-13 00:05 — TrueNova

- **Timestamp (UTC):** 2026-11-13 00:05
- **Author:** TrueNova
- **Type:** decision
- **Status:** agreed
- **Affected agents:** TrueNova, RedArrow, SwiftXenon, EpicKnight
- **Decision:** Docs lane catch-up complete on `main = 9d8c109`. PR #17 merged as `bf54f81` (`docs/peers/decisions.md` +91/-0, appending the 7 team.memory entries of 17:50–19:29 UTC in SOP-TEMPLATES.19 format, newest-first; author RedArrow, reviewed by TrueNova via PR comment). PR #18 merged as `9d8c109` (`CHANGELOG.md` [Unreleased] +10/-0: PR #13 `actions/checkout` 4.2.2→7.0.1 full-SHA pin; SOP-GHOP.1/.9 repository settings; PR #15 `coord_send`→`pi_messenger`). Mechanism recorded for the crew: #18's squash merge initially failed with `GraphQL: 7 of 7 required status checks are expected` because #17's merge had moved `main` and left #18 `mergeStateStatus=BEHIND` under `strict=true` while `mergeable=MERGEABLE` with all checks green; the fix is `gh pr update-branch <N>`, wait for all required checks to re-run green, then merge — no branch-protection change is needed. Reservations on `docs/peers/decisions.md` + `CHANGELOG.md` released.
- **Evidence:** `git show origin/main:docs/peers/decisions.md` (11 entries); `git show origin/main:CHANGELOG.md` lines 138–152; `git log origin/main -3`; team.memory decision note 2026-11-13 00:04:40Z.
- **Version marker:** —
- **team.memory entry:** "DECISION 2026-11-13 00:05 UTC (TrueNova). Docs lane catch-up COMPLETE on main = 9d8c109."
- **Supersedes / superseded by:** —

## 2026-11-12 23:55 — EpicKnight

- **Timestamp (UTC):** 2026-11-12 23:55
- **Author:** EpicKnight
- **Type:** interface
- **Status:** in-progress
- **Affected agents:** EpicKnight (author), NiceArrow (#16 consumer), SageBear, TrueNova
- **Decision:** Issue #7 Phase 1b LLVM codegen declared complete and re-verified against `origin/main 1bee3de`. Branch `feat/7-codegen-llvm` (worktree `/root/dev/yuliac-7-codegen`). Files: `src/codegen/{mod.rs,ty.rs,lower/{mod.rs,scope.rs,function.rs,expr/{mod.rs,binary.rs,checked.rs,access.rs,aggregate.rs},stmt/{mod.rs,control.rs,for_stmt.rs}}}` (recorded as “all ≤142 SLOC, SOP-MOD compliant”; re-measured 2026-11-13 counting non-blank non-comment lines: `lower/expr/access.rs` 142 SLOC / 164 total, `lower/expr/mod.rs` 141 SLOC / 162 total, `src/typeck/mod.rs` 105 SLOC / 143 total — all within the 150 cap. An earlier total-lines-only reading of the same files was a false alarm and is retracted; issue #21 closed as a false alarm) and `src/typeck/{mod.rs,expr/mod.rs}` (`TypeMap`); `Cargo.toml`+`Cargo.lock` add `inkwell 0.11` with feature `llvm20-1`. Interface contract for PR #16: `Checker::check` / `Checker::types`; `codegen::generate`, `write_ir`, `write_object`. Reservations held: `src/codegen/`, `src/typeck/`, `Cargo.toml`, `Cargo.lock`.
- **Evidence:** `cargo fmt --check` OK; `cargo clippy --lib --all-targets -- -D warnings` CLEAN; `cargo test --lib` 1 passed; E2E SUM exit 2560 (=10<<8), R0101/R0102/R0103 stderr `runtime error[<code>]` + SIGABRT; team.memory decision note 2026-11-12 23:55:52Z.
- **Version marker:** —
- **team.memory entry:** "DECISION 2026-11-12 23:55 UTC (EpicKnight). Issue #7 Phase 1b LLVM codegen is COMPLETE…"
- **Supersedes / superseded by:** —

## 2026-11-12 19:29 — SageBear

- **Timestamp (UTC):** 2026-11-12 19:29
- **Author:** SageBear
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, DarkStorm, HappyHawk
- **Decision:** Correction of the 19:21 "FALLBACK=HOLD" call on PR #14/#15. That call read `gh api repos/Miruameli/yulia/pulls/{14,15}/reviews`, which is empty because APPROVE is impossible on the shared Miruameli account. This crew's review channel is PR COMMENTS (recorded 17:50). NiceArrow posted substantive LGTM review comments on both PRs at 19:13 UTC, before the 19:20 deadline, so SOP-GHOP.2's "at least one review" was satisfied and the hold was unnecessary. PR #14 merged as 8d217b1.
- **Evidence:** `gh api repos/Miruameli/yulia/issues/14/comments` and `/15/comments` ("Reviewed 2026-11-12 (NiceArrow) — LGTM", with SOP-TEMPLATES.19 format check and `git grep coord_send origin/main` verification); `gh pr view 14` (MERGED, mergeCommit 8d217b1); team.memory correction note 19:29 UTC.
- **Version marker:** —
- **team.memory entry:** "CORRECTION 2026-11-12 19:29 UTC (SageBear)"
- **Supersedes / superseded by:** Supersedes the 19:21 UTC hold decision.

## 2026-11-12 19:27 — SageBear

- **Timestamp (UTC):** 2026-11-12 19:27
- **Author:** SageBear
- **Type:** interface
- **Status:** open
- **Affected agents:** NiceArrow (owner), SageBear, DarkStorm
- **Decision:** SOP-GHOP.8 audit of `origin/main:.github/workflows/ci.yml` (126 lines). Compliant: `permissions: contents: read`; per-job and per-step `timeout-minutes` on all 7 jobs; `actions/checkout` pinned to full SHA `3d3c42e5…` # v7.0.1; no `pull_request_target`; no secrets; env at workflow level; `CARGO_BUILD_JOBS=2` and `--test-threads=2`; cargo-deny 0.20.2, typos-cli 1.52.1, @colbymchenry/codegraph 1.6.2 pinned. Two deviations routed to NiceArrow for PR #8, which holds the `ci.yml` reservation (INV-41): (1) `runs-on: ubuntu-latest` on all 6 jobs vs the SOP-GHOP.8 pinned `ubuntu-24.04`; (2) `concurrency.cancel-in-progress: true` vs the conditional `${{ github.event_name == 'pull_request' }}` form.
- **Evidence:** `gh api repos/Miruameli/yulia/contents/.github/workflows/ci.yml`.
- **Version marker:** —
- **team.memory entry:** "GHOP.8 CI audit (SageBear, 2026-11-12 19:27 UTC)"
- **Supersedes / superseded by:** —

## 2026-11-12 19:25 — SageBear

- **Timestamp (UTC):** 2026-11-12 19:25
- **Author:** SageBear
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, DarkStorm, HappyHawk
- **Decision:** SOP-GHOP.9 compliance: private vulnerability reporting on Miruameli/yulia was disabled and is now enabled (verified `enabled: true`; reversible with `gh api -X DELETE repos/Miruameli/yulia/private-vulnerability-reporting`). `.env.example` is deliberately absent and stays absent: `gh api search/code?q=env::var+repo:Miruameli/yulia` returns `total_count: 0`, so the project reads no environment configuration. Also re-verified clean: 0 Dependabot / 0 code-scanning / 0 secret-scanning alerts; no orphan branches; `.github` holds CODEOWNERS, four issue templates, the PR template, `dependabot.yml`, `workflows/ci.yml`.
- **Evidence:** `gh api repos/Miruameli/yulia/private-vulnerability-reporting` before/after; `gh api search/code`; `gh api repos/Miruameli/yulia/branches`; the three alert endpoints.
- **Version marker:** —
- **team.memory entry:** "GHOP compliance 2026-11-12 19:25 UTC (SageBear)"
- **Supersedes / superseded by:** —

## 2026-11-12 19:08 — SageBear

- **Timestamp (UTC):** 2026-11-12 19:08
- **Author:** SageBear
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, DarkStorm, HappyHawk
- **Decision:** Repository setting `has_wiki` changed `true -> false` per SOP-GHOP.1 ("Wiki disabled, use `docs/`"). Evidence that nothing was lost: the wiki git remote `https://github.com/Miruameli/yulia.wiki.git` returned HTTP 404 (uninitialized, no content). Reversible with `gh api -X PATCH -F has_wiki=true`. This closes the "Remaining: wiki is ON" item in the 18:05 entry above; per NiceArrow's review of PR #14 the append-only mirror records it as a new entry rather than editing that one.
- **Evidence:** `gh api repos/Miruameli/yulia` before/after (`has_wiki`); the wiki remote 404.
- **Version marker:** —
- **team.memory entry:** "decision (19:07 UTC): repo setting has_wiki true->false"
- **Supersedes / superseded by:** —

## 2026-11-12 18:48 — SageBear

- **Timestamp (UTC):** 2026-11-12 18:48
- **Author:** SageBear
- **Type:** interface
- **Status:** open
- **Affected agents:** NiceArrow, SageBear, DarkStorm
- **Decision:** Cross-agent contract on the shared `.github/workflows/ci.yml`. PR #13 (checkout pin bump) is merged as 6a16aaa; all 7 jobs pin `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1` (full SHA, no mutable tag). NiceArrow's #8 adds an `install-llvm20` composite step; rebase onto current main and keep the v7.0.1 SHA — do not re-introduce 4.2.2. SageBear holds no reservation on `ci.yml`.
- **Evidence:** `gh pr view 13`; `gh api repos/Miruameli/yulia/contents/.github/workflows/ci.yml`.
- **Version marker:** —
- **team.memory entry:** "ci.yml coordination (SageBear, 2026-11-12 18:47 UTC)"
- **Supersedes / superseded by:** —

## 2026-11-12 18:22 — SageBear / HappyHawk

- **Timestamp (UTC):** 2026-11-12 18:22
- **Author:** SageBear / HappyHawk
- **Type:** risk
- **Status:** open
- **Affected agents:** SageBear, NiceArrow, DarkStorm, HappyHawk
- **Decision:** v6.0 SOP-GHOP.2 says main requires 1 approval, but the repo runs `required_approving_review_count=0` and `require_code_owner_reviews=false`, because every crew agent shares the single Miruameli GitHub account (self-approval impossible) and `.github/CODEOWNERS` routes every path to `@Miruameli`. Options: (a) keep 0 as a recorded deviation, with the 7 bare-name checks as the gate and peer review done via `pi_messenger` + PR comments — recommended; (b) require the human's review on every PR, which serialises all merges on the human. Owner: `@Miruameli`.
- **Evidence:** `gh pr review 9 --approve` failed with the GraphQL self-review error; `gh api repos/Miruameli/yulia/branches/main/protection`.
- **Version marker:** —
- **team.memory entry:** "OPEN ESCALATION (SageBear/HappyHawk, 2026-11-12 18:00 UTC)"
- **Supersedes / superseded by:** —

## 2026-11-12 17:50 — HappyHawk

- **Timestamp (UTC):** 2026-11-12 17:50
- **Author:** HappyHawk
- **Type:** decision
- **Status:** agreed
- **Affected agents:** SageBear, NiceArrow, DarkStorm, HappyHawk
- **Decision:** Contract v6.0 adoption is complete on main: PR #11 merged (fff8300) and PR #9 merged (3fe9e00); `origin/main:KONTRAK.md` sha256 `46e4c4e4e105b23245be3d385157386c24a002b547418767bef640fe31f6176c`, 1755 lines, 61 sections; `origin/main:docs/peers/decisions.md` sha256 `60c761089d38072fe9bbdab6c4504b888ca10797c2e6e59986823ead8e041c97`, 60 lines, SOP-TEMPLATES.19 format. DURABLE MECHANISM FACT: all crew agents share one GitHub account (Miruameli), so GitHub rejects APPROVE on any crew PR; peer review in this crew is done via PR COMMENTS, not approvals.
- **Evidence:** `gh api` hashes above; `gh pr review 9 --approve` self-review error; `gh api repos/Miruameli/yulia/branches/main/protection`.
- **Version marker:** v6.0 (contract)
- **team.memory entry:** "DECISION 2026-11-12 17:50 UTC (HappyHawk)"
- **Supersedes / superseded by:** —

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
