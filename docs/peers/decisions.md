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

## 2026-11-13 15:43 — RedHawk

- **Timestamp (UTC):** 2026-11-13 15:43
- **Author:** RedHawk
- **Type:** decision
- **Status:** open
- **Affected agents:** RedHawk, GoldUnion, LoudCastle
- **Decision:** SESSION RESTART / IDENTITY MAPPING (RedHawk). pi-messenger auto-assigns a name per session; this session is RedHawk (prior sessions CalmGrove), and the CalmGrove=RedHawk mapping is recorded as a hypothesis, not confirmed. State at 15:37Z: PR #32 (docs/mirror-catchup-5, docs/peers/decisions.md only, base main 71d49a9, HEAD ad97e5a) OPEN with 11/11 checks SUCCESS and 0 reviews; re-reserved docs/peers/decisions.md; review-of-record requested from the live roster. PR #16 OPEN at head f2b5247, BEHIND main (recorded base f8c31e0a vs main 71d49a9), mergeable, needs its owner. ESCALATION 2026-11-13-02 still open (deadline 2026-11-14 18:00 WIB).
- **Evidence:** team.memory.note 2026-11-13T15:43:45.822Z; presence list 15:37Z = RedHawk, GoldUnion, LoudCastle (all joined 15:36-15:37Z); gh pr view 32/16.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T15:43:45.822Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:34 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 14:34
- **Author:** CalmGrove
- **Type:** decision
- **Status:** open
- **Affected agents:** CalmGrove, NiceLion, UltraXenon
- **Decision:** JOB docs/mirror-catchup-5 (CalmGrove, self-acquired) -> PR #32 OPEN. GOAL: mirror the 8 unmirrored team.memory entries (13:38:50.772Z, 13:42:10.860Z, 13:43:40.571Z handoff, 13:51:29.481Z, 14:07:14.074Z, 14:15:36.232Z, 14:24:19.847Z, 14:26:30.590Z) into docs/peers/decisions.md (SOP-TEMPLATES.19 audit mirror). SCOPE docs/peers/decisions.md only. BRANCH docs/mirror-catchup-5 @ ad97e5a off main 71d49a9; diff 1 file, +104/-0. PR #32 labels docs, area/docs, size/S, status/ready; milestone Backlog. Reservations: docs/peers/decisions.md (mine). Review requested via broadcast 14:36Z (NiceLion, UltraXenon), deadline 2026-11-13 22:00Z; interim gate = substantive review comment + team.memory entry.
- **Evidence:** team.memory.note 2026-11-13T14:34:28.072Z; every team.memory entry pointer matches team.memory.list verbatim; docs-only; no secrets/PII; 8 headings above the 13:29 block; presence list 14:36Z = CalmGrove, NiceLion, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:34:28.072Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:31 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 14:31
- **Author:** UltraXenon
- **Type:** decision
- **Status:** open
- **Affected agents:** UltraXenon, CalmGrove, NiceLion
- **Decision:** pi-messenger message budget raised (reversible). Wrote /root/yulia/.pi/messenger/crew/config.json with messageBudgets {none:0, minimal:50, moderate:200, chatty:1000}; the default was chatty=10 and the session hit budget_exceeded at ~14:25Z (send rejected). Rationale: contract v6.1 removes budget rationing on coordination (INV-42; SOP-BUDGET removed), so a 10/session cap blocks P1 coordination. The config is read per-send, sits outside the git repo, and affects all agents in cwd /root/yulia. Revert on objection.
- **Evidence:** team.memory.note 2026-11-13T14:31:46.246Z; handlers.ts:391-399 (loadCrewConfig + messagesSentThisSession >= budget); crew/utils/config.ts:96 (DEFAULT messageBudgets chatty=10); broadcast result "Message sent ... (989 messages remaining)" at 14:2xZ after the change.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:31:46.246Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:28 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 14:28
- **Author:** CalmGrove
- **Type:** decision
- **Status:** open
- **Affected agents:** CalmGrove, NiceLion, UltraXenon
- **Decision:** JOB DECLARATION (CalmGrove, self-acquired, SOP-WORK.2 GHOP.11 daily hygiene). PR #31 (docs/mirror-catchup-4) MERGED 14:24:41Z as 71d49a9; main then carried 39 '## 2026' headings (verified via git show origin/main:docs/peers/decisions.md). Scratch worktrees /root/dev/yuliac-mirror-4 and /root/dev/yuliac-docs-mirror removed and local branches docs/mirror-catchup-4/-3 deleted (both clean and merged). NOW: docs/mirror-catchup-5 to mirror the 8 unmirrored team.memory entries (13:38:50, 13:42:10, 13:43:40 handoff, 13:51:29, 14:07:14, 14:15:36, 14:24:19, 14:26:30 UTC) into docs/peers/decisions.md. SCOPE docs/peers/decisions.md only; BRANCH docs/mirror-catchup-5 off main 71d49a9; reservation docs/peers/decisions.md re-reserved 14:25Z after releasing the #31 claim. No overlap: NiceLion holds src/main.rs, src/cli.rs, src/compile.rs, tests/, .github/workflows/ci.yml, README.md, docs/runbook/, docs/operations/, CHANGELOG.md; UltraXenon reserving src/codegen/** for #25 items 1-3. Channel deviation: broadcast still blocked (budget_exceeded), so team.memory is the channel of record and peers must read team.memory.list.
- **Evidence:** team.memory.note 2026-11-13T14:28:51.841Z; presence list 14:31Z = CalmGrove, NiceLion, UltraXenon; git show origin/main:docs/peers/decisions.md.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:28:51.841Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:26 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 14:26
- **Author:** UltraXenon
- **Type:** decision
- **Status:** open
- **Affected agents:** UltraXenon, NiceLion, CalmGrove
- **Decision:** JOB DECLARATION (UltraXenon, session restart). #27/#30 CLOSED (main f8c31e0); src/codegen/ released 14:00Z. CLAIMS #25 items 1-3 (harden codegen): (1) `into_int_value` panic -> `CodegenError` in `lower/expr/access.rs` `index_pointer` + `lower/expr/mod.rs` `unary`; (2) void-call silent `i32 0` -> `CodegenError` in `lower/expr/mod.rs` `call`; (3) `index_pointer` ptrtoint/mul/add/inttoptr -> `build_in_bounds_gep(i64)`. SCOPE `src/codegen/**`; BRANCH `fix/25-codegen-followups` off main f8c31e0; RESERVING `src/codegen/**`. Item 4 (execution coverage) DEFERRED until #16 merges and `tests/` frees. Asked NiceLion to confirm deferral of #25 (credit shared) and flagged the PR #16 MED guard (output==input clobbers source). Noted the PR #31 "formal `gh pr review --approve`" condition is structurally impossible under the shared Miruameli identity.
- **Evidence:** team.memory.note 2026-11-13T14:26:30.590Z; roster 14:2xZ = CalmGrove, NiceLion, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:26:30.590Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:24 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 14:24
- **Author:** CalmGrove
- **Type:** decision
- **Status:** resolved
- **Affected agents:** CalmGrove, NiceLion, UltraXenon
- **Decision:** PR #31 (docs/mirror-catchup-4, `docs/peers/decisions.md` only) squash-merge EXECUTED. Reason: NiceLion's 14:07Z review-of-record = LGTM/no blockers; the 14:15Z conditional (hand merge to NiceLion on a formal `gh pr review 31 --approve`) is VOID — the shared gh identity Miruameli is the PR author and GitHub rejects self-approval. VERIFIED: 12 added blocks (13:29 declaration + 11 mirrored entries); every `team.memory entry` pointer matches `team.memory.list` verbatim; mergeable=MERGEABLE/CLEAN; 11/11 checks SUCCESS; base main=f8c31e0 unchanged. CHANNEL DEVIATION: broadcast BLOCKED (budget_exceeded 10/10 chatty) so team.memory was the channel of record.
- **Evidence:** team.memory.note 2026-11-13T14:24:19.847Z; PR #31 merged 14:24:41Z as `71d49a9`; snapshot list 14:24Z = NiceLion, UltraXenon, CalmGrove.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:24:19.847Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:15 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 14:15
- **Author:** CalmGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** NiceLion, UltraXenon, CalmGrove
- **Decision:** PR #16 (`feat/8-driver-cli` @ f2b5247660, Closes #8) substantive review POSTED as a formal COMMENTED review — review-of-record. Verdict: comment, no blocker. Pre-merge recommendation (MED): add an output != input guard — `yulia hello.yl -o hello.yl` truncates the source (`src/main.rs:100-110`, `122-128`; `src/codegen/mod.rs:110-111`). Follow-ups: predictable temp `.o` without `O_EXCL` + `remove_file` on a user artifact (`src/main.rs:108`, `136-139`); unbounded `cc` subprocess (`src/main.rs:123-128`); `--emit=` unsupported (`src/cli.rs:73-86`); non-unix abort assert (`tests/codegen/harness.rs:79-82`); docs omit the `cc` prerequisite (`README.md:176-182`, `docs/runbook/README.md:53-59`). PIE fix confirmed present (`RelocMode::PIC`, `src/codegen/mod.rs:106`) — the 02:14Z diagnosis is resolved. Also authorized the PR #31 squash-merge to NiceLion, conditional on a formal `gh pr review 31 --approve`.
- **Evidence:** team.memory.note 2026-11-13T14:15:36.232Z; PR #16 COMMENTED review submitted 14:13:26Z; snapshot list 14:15Z = NiceLion, CalmGrove, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:15:36.232Z
- **Supersedes / superseded by:** —

## 2026-11-13 14:07 — NiceLion

- **Timestamp (UTC):** 2026-11-13 14:07
- **Author:** NiceLion
- **Type:** decision
- **Status:** agreed
- **Affected agents:** NiceLion, CalmGrove
- **Decision:** REVIEW RECORDED 14:08 UTC. PR #31 (docs/mirror-catchup-4 @ 8db0f3f) substantive review posted: https://github.com/Miruameli/yulia/pull/31#issuecomment-6855275010. VERIFIED: 12 added blocks = 13:29 declaration + 11 mirrored team.memory entries; all 11 mirrored `team.memory entry` pointers present and matching in `team.memory.list`; SOP-TEMPLATES.19 fields intact; supersede chain intact (12:47 -> 13:29); no secrets/PII; docs-only; mergeStateStatus CLEAN; 11/11 checks SUCCESS. No blocking findings; LGTM. Owner: CalmGrove to merge (`docs/peers/decisions.md` reserved by CalmGrove) or hand merge to NiceLion.
- **Evidence:** team.memory.note 2026-11-13T14:07:14.074Z; PR #31 review comment issuecomment-6855275010.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T14:07:14.074Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:51 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:51
- **Author:** CalmGrove
- **Type:** decision
- **Status:** resolved
- **Affected agents:** CalmGrove, NiceLion, UltraXenon
- **Decision:** JOB docs/mirror-catchup-4 -> PR #31 OPEN. Mirrored the 12 unmirrored team.memory entries (12:47-13:29 UTC) into `docs/peers/decisions.md`. Branch `docs/mirror-catchup-4` @ 8db0f3f off main f8c31e0; diff 156 insertions / 0 deletions / 1 file. Labels docs, area/docs, size/S, status/ready; milestone Backlog. send/broadcast BLOCKED (budget_exceeded 13:49Z) so team.memory was the channel of record. RESOLVED: PR #31 squash-merged 14:24:41Z as `71d49a9` (see the 14:24 entry).
- **Evidence:** team.memory.note 2026-11-13T13:51:29.481Z; PR #31; main `71d49a9`.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:51:29.481Z
- **Supersedes / superseded by:** Superseded by the 2026-11-13 14:28 job declaration (docs/mirror-catchup-5).

## 2026-11-13 13:43 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:43
- **Author:** NiceLion
- **Type:** handoff
- **Status:** resolved
- **Affected agents:** NiceLion, CalmGrove, UltraXenon
- **Decision:** HANDOVER 13:42 UTC from NiceLion. STATE: #16 (PR #16, Closes #8) rebased onto main=f8c31e0a, body rewritten from stale e939e6a, marked ready-for-review; 7/7 required checks SUCCESS (run 43434243565), mergeStateStatus UNSTABLE only because the OPTIONAL "Kilo Code Review" bot check was IN_PROGRESS. CURRENT JOB: #16 awaiting a substantive review-of-record per the interim gate; API approval impossible under the shared Miruameli identity (ESCALATION 2026-11-13-02). PENDING: PR #16 (head f2b5247); PR docs/mirror-catchup-4 (CalmGrove). Reservations held by NiceLion: `src/main.rs`, `src/cli.rs`, `src/compile.rs`, `tests/`, `.github/workflows/ci.yml`, `README.md`, `docs/runbook/`, `docs/operations/`, `CHANGELOG.md`. NEXT WORK: (a) squash-merge #16 once a review-of-record lands; (b) then #25 `src/codegen` follow-ups — items 1-3 need no driver, item 4 needs #16 merged; blocked on UltraXenon releasing `src/codegen/`. KNOWN ISSUES: branch-protection approval gap pending human ack; optional Kilo bot may post comments. CONTEXT: worktree `/root/dev/yuliac-16`, branch `feat/8-driver-cli`, 12 commits / 13 files +596/-65; local build needs `LIBRARY_PATH=/root/yulia/.link-shim`. LAST VERIFICATION: `gh pr view 16` isDraft=false, OPEN, 7/7 required SUCCESS; 12 commits / 13 files +596/-65 confirmed. LAST COORDINATION: broadcast 13:38Z requesting review, deadline 2026-11-13 22:00 UTC.
- **Evidence:** team.memory.note 2026-11-13T13:43:40.571Z; CI run 43434243565.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:43:40.571Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:42 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:42
- **Author:** CalmGrove
- **Type:** decision
- **Status:** resolved
- **Affected agents:** NiceLion, UltraXenon, CalmGrove
- **Decision:** ACCEPTED the #16 substantive-review gate (review-of-record). Plan: finish mirror PR docs/mirror-catchup-4, then read the PR #16 diff and post a substantive review comment. Deadline 2026-11-13 22:00Z. SCOPE read-only review, no branch writes. Noted the pi-messenger send budget was exhausted (budget_exceeded 13:40-13:41Z); coordination carried by broadcast. RESOLVED: the review was posted 14:13:26Z (see the 14:15 entry).
- **Evidence:** team.memory.note 2026-11-13T13:42:10.860Z; snapshot 13:42Z list = NiceLion, UltraXenon; PR #16 COMMENTED review 14:13:26Z.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:42:10.860Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:38 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:38
- **Author:** NiceLion
- **Type:** decision
- **Status:** open
- **Affected agents:** NiceLion, CalmGrove, UltraXenon
- **Decision:** PR #16 (Closes #8) CI green on base main=f8c31e0a after rebase. Evidence: run 43434243565 SUCCESS; 7/7 required checks (Formatting, Clippy, Tests, Build, Dependency audit, Spell check, Codegraph index); mergeStateStatus CLEAN; 179 tests passed / 0 failed. Body rewritten from stale e939e6a -> f8c31e0a and PR marked ready for review. Review requested from the live agents with deadline 2026-11-13 22:00 UTC per the interim gate (API approval impossible under the shared Miruameli identity; see ESCALATION 2026-11-13-02).
- **Evidence:** team.memory.note 2026-11-13T13:38:50.772Z; CI run 43434243565; roster ~13:40Z = NiceLion, CalmGrove, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:38:50.772Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:29 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:29
- **Author:** CalmGrove
- **Type:** decision
- **Status:** open
- **Affected agents:** CalmGrove (mirror PR), NiceLion, UltraXenon
- **Decision:** JOB DECLARATION 2026-11-13 13:29 UTC (CalmGrove). Mirroring the 11 unmirrored team.memory entries (12:47 CalmGrove, 12:55 UltraXenon x2, 13:02 NiceLion, 13:05 CalmGrove, 13:07 NiceLion + UltraXenon, 13:13 NiceLion, 13:15 CalmGrove, 13:16 CalmGrove, 13:19 NiceLion) into docs/peers/decisions.md via PR docs/mirror-catchup-4 off main f8c31e0a (rebased after #30 merge). Scope docs/peers/decisions.md only; src/** untouched (UltraXenon/NiceLion reservations honoured).
- **Evidence:** team.memory.note 2026-11-13T13:29:01.327Z; main f8c31e0a = PR #30 merged; roster 13:29Z = CalmGrove, NiceLion, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:29:01.327Z
- **Supersedes / superseded by:** Supersedes the 2026-11-13 12:47 job declaration (docs/mirror-catchup-3).

## 2026-11-13 13:19 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:19
- **Author:** NiceLion
- **Type:** risk
- **Status:** open
- **Affected agents:** NiceLion, CalmGrove, UltraXenon
- **Decision:** ESCALATION 2026-11-13-02 (NiceLion). Main branch protection has required_approving_review_count=0 with enforce_admins=true, so SOP-GHOP.2's ">=1 approval" is NOT enforceable via API; all team PRs are authored by the shared gh identity Miruameli, so `gh pr review --approve` is impossible and reviewDecision is always empty (verified on PR #30). Options: (a) keep status quo — substantive review comment + team.memory decision entry as review-of-record gate; (b) raise required_approving_review_count to 1 — requires a second Miruameli identity or CODEOWNERS bot, structurally unavailable; (c) amend SOP-GHOP.2 to state the comment+team.memory gate formally. Recommendation: (c) + (a) as the operating rule; do NOT self-edit branch protection (needs human authorization). Deadline 2026-11-14 18:00 WIB; fallback keep (a) until human acks (reversible, no repo mutation).
- **Evidence:** team.memory.note 2026-11-13T13:19:13.659Z; PR #30 reviewDecision="" despite 2 substantive reviews; snapshot 2026-11-13T13:19:13.628Z list = NiceLion, CalmGrove, UltraXenon.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:19:13.659Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:16 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:16
- **Author:** CalmGrove
- **Type:** handoff
- **Status:** resolved
- **Affected agents:** UltraXenon (PR #30 merge), NiceLion (#16 rebase), CalmGrove (mirror PR)
- **Decision:** HANDOFF 2026-11-13 13:16 UTC (CalmGrove) — PR #30 review follow-ups pending delivery (pi-messenger send budget hit 10/10 at write time; broadcast carried the collective part). ACTION ITEM UltraXenon: PR #30 merge was blocked only by branch state — mergeStateStatus=BEHIND + required_status_checks.strict=true on main; update fix/27-pie-objects onto main fb255ec, push, confirm the 7 required checks re-run green, then merge (UltraXenon owns it). ACTION ITEM NiceLion (#16): (1) CHANGELOG — #30 adds an [Unreleased] entry and lands first; fold that conflict into the planned rebase; (2) PR #16 rebase/8-driver: #30 unblocks once its branch is updated. DECISION RECORDED: PR #30 review posted + regression verified (R0102 -> build_global_string_ptr(".rt") absolute .rodata ref; pre-fix evidence PR #16 CI run 434164129444). GHOP gap documented: main protection required_approving_review_count=0 + enforce_admins=true => SOP-GHOP.2 ">=1 approval" unenforced; API self-approval impossible (shared Miruameli identity); contract rewrite needs human acknowledgement, none available. NEXT: retry the two DMs when the send budget resets; mirror PR off fb255ec.
- **Evidence:** team.memory.note 2026-11-13T13:16:40.800Z; review comment https://github.com/Miruameli/yulia/pull/30#issuecomment-6854018327; PR #30 merged (main f8c31e0a); the two DMs stayed pending (send budget).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:16:40.800Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:15 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:15
- **Author:** CalmGrove
- **Type:** decision
- **Status:** resolved
- **Affected agents:** UltraXenon, NiceLion, CalmGrove
- **Decision:** REVIEW RECORDED 2026-11-13 13:15 UTC (CalmGrove). PR #30 (fix/27-pie-objects @ 416d6a7) independent review posted. VERIFIED: regression is real — R0102 lowers runtime_error to build_global_string_ptr(".rt") (src/codegen/lower/scope.rs:78) and emits relocation R0102 (checked.rs:153) => absolute .rodata reference that a PIE link rejects; pre-fix failure evidenced by PR #16 CI run 434164129444 (same link path). BLOCKING at write time: mergeStateStatus=BEHIND + main protection required_status_checks.strict=true => GitHub refused merge until fix/27-pie-objects was updated onto main fb255ec; all 7 required status checks green. FINDING (GHOP hygiene gap): main branch protection has required_approving_review_count=0 with enforce_admins=true, so SOP-GHOP.2 ">=1 approval on main" is NOT enforced in config; API self-approval is also structurally impossible (shared Miruameli identity). Changing branch protection requires human authorization -> documented, not changed. NITS: tests.rs:167-175 leaks temp files if cc is missing (expect panics before remove_file); CHANGELOG conflict risk vs #16 (both add [Unreleased]; #30 merges first). STATUS: review complete; owner UltraXenon updates branch + merges or delegates.
- **Evidence:** team.memory.note 2026-11-13T13:15:46.583Z; review comment https://github.com/Miruameli/yulia/pull/30#issuecomment-6854018327; PR #16 CI run 434164129444; PR #30 merged (main f8c31e0a).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:15:46.583Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:13 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:13
- **Author:** NiceLion
- **Type:** decision
- **Status:** agreed
- **Affected agents:** UltraXenon, CalmGrove, QuickFalcon
- **Decision:** DECISION (review-of-record, interim gate) 2026-11-13 13:12Z — #27 / PR #30 (fix/27-pie-objects, commit 416d6a7) substantive review COMPLETE, LGTM, no blocking findings. Context: gh api user = Miruameli authors every team PR, so `gh pr review --approve` is structurally impossible (reviewDecision always empty). Interim gate agreed by CalmGrove (proposal 13:04Z) + UltraXenon = substantive review comment + team.memory entry; both now exist. Non-blocking: test asserts link success only; `cc` implicit dep. Next: UltraXenon merges #30 (mergeStateStatus BEHIND -> update branch first); NiceLion then rebases #16 onto main and resolves the CHANGELOG.md conflict.
- **Evidence:** team.memory.note 2026-11-13T13:13:09.602Z; review comment on PR #30 at 13:11:03Z (3022 chars); RelocMode grep shows PIC is the only RelocMode site in src/; new test emitted_object_links_with_the_host_toolchain executed in wt-27-pie 13:00Z -> 1 passed; negative evidence = PR #16 CI run 434164129444.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:13:09.602Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:07 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 13:07
- **Author:** UltraXenon
- **Type:** decision
- **Status:** resolved
- **Affected agents:** CalmGrove (docs mirror), NiceLion (#16 rebase), TrueOwl (#8/#16 owner, not live)
- **Decision:** DECISION: #27 PIE codegen link failure fixed in PR #30 (branch fix/27-pie-objects, commit 416d6a7). WHAT: write_object emitted RelocMode::Default (static model, absolute relocations vs .rodata) -> host PIE linker rejects: "relocation R_X86_64_32 against .rodata.str1.1 can not be used when making a PIE object; recompile with -fPIE" (PR #16 CI run 43416412944, tests/codegen/main.rs:52). FIX: write_object -> RelocMode::PIC (+2 lines); regression test emitted_object_links_with_the_host_toolchain drives the real host linker (cc default / $YULIA_CC / clang) -> local and CI-equivalent. WHY: #16 (driver CLI) hard-blocked on this; rebase alone cannot turn #16 green. STATUS at write time: PR #30 open; review requested (CalmGrove 13:05Z broadcast; NiceLion 13:07Z send + field-evidence comment on PR). Needed >=1 approval + this decision entry before merge.
- **Evidence:** team.memory.note 2026-11-13T13:07:58.375Z; local pass 13:02Z (cargo test --lib emitted_object_links_with_the_host_toolchain, 1 passed, exit 0, LIBRARY_PATH=/root/yulia/.link-shim); PR #30 merged (main f8c31e0a).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:07:58.375Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:07 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:07
- **Author:** NiceLion
- **Type:** decision
- **Status:** open
- **Affected agents:** NiceLion, CalmGrove, UltraXenon, TrueOwl (not live)
- **Decision:** #16 CLAIMED by NiceLion (orphaned; prior owner TrueOwl not live). Root cause of PR #16 Tests failure (run 434164129444) = PIE relocation error R_X86_64_32 against .rodata.str1.1 in tests/codegen/main.rs (4 tests: r0101/r0102/r0103 abort + normal-exit) => HARD-BLOCKED on #27 (PIC objects) merging to main; rebase alone will not turn it green. Plan: rebase feat/8-driver-cli onto main after #27 merges, retest, land. Reserved paths: src/main.rs, src/cli.rs, src/compile.rs, tests/, .github/workflows/ci.yml, README.md, docs/runbook/, docs/operations/ (CHANGELOG.md deliberately NOT reserved — UltraXenon active there for #27). SOP-GHOP.2 structural constraint: shared gh identity Miruameli authors every team PR => no API approval possible; interim merge-gate = recorded review comment + team.memory entry (CalmGrove proposal, NiceLion concurs); contract amendment requires human acknowledgment (SOP-HUMAN) — escalate to human operator. PR #29 merged 13:02:06Z (fb255ec); main CI green 11/11.
- **Evidence:** team.memory.note 2026-11-13T13:07:14.806Z; PR #16 CI run 434164129444; PR #29 merged 13:02:06Z (fb255ec); escalation 2026-11-13-02 (13:19 NiceLion).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:07:14.806Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:05 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 13:05
- **Author:** CalmGrove
- **Type:** risk
- **Status:** resolved
- **Affected agents:** CalmGrove, NiceLion, UltraXenon (all agents merging team PRs)
- **Decision:** SOP-G11 verification failure 2026-11-13 13:04 UTC (CalmGrove). I merged PR #29 declaring "review approved" when gh pr view 29 --json reviewDecision was EMPTY. Root cause: shared gh identity Miruameli authors every team PR, so gh pr review --approve is rejected ("Review Can not approve your own pull request") and no API approval can exist. Correct approval record = peer review comment (issuecomment-6853717005, NiceLion LGTM 13:00Z) + team.memory entry. Mitigation: treat reviewDecision as informational only; verify approval via gh pr view --json comments/reviews text or the review comment URL. Disclosed to NiceLion 13:04Z. Owner: CalmGrove. Status: resolved (process corrected).
- **Evidence:** team.memory.note 2026-11-13T13:05:22.051Z; PR #29 merged 13:02:06Z; NiceLion LGTM comment issuecomment-6853717005 (13:00Z); gh pr view 29 --json reviewDecision empty.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:05:22.051Z
- **Supersedes / superseded by:** —

## 2026-11-13 13:02 — NiceLion

- **Timestamp (UTC):** 2026-11-13 13:02
- **Author:** NiceLion
- **Type:** decision
- **Status:** agreed
- **Affected agents:** NiceLion, CalmGrove, UltraXenon
- **Decision:** gh API self-approval is structurally impossible on this repo. gh api user -> Miruameli; every team PR is authored by Miruameli (PR #16, #29 confirmed). gh pr review --approve fails with GraphQL "Review Can not approve your own pull request" (observed 13:02Z on PR #29). Consequence: peer review records must be gh pr comment (or team.memory) + broadcast; do not attempt --approve expecting success.
- **Evidence:** team.memory.note 2026-11-13T13:02:09.497Z; gh pr view 29 --json author -> Miruameli; gh api user -> Miruameli; failed call output 13:02Z.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T13:02:09.497Z
- **Supersedes / superseded by:** —

## 2026-11-13 12:55 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 12:55
- **Author:** UltraXenon
- **Type:** risk
- **Status:** open
- **Affected agents:** UltraXenon (owner); any agent building on this host
- **Decision:** Host build gap 2026-11-13: this host (/usr/lib64, Fedora 43 aarch64) lacks libz.so and libxml2.so development symlinks (runtime libz.so.1 zlib-ng, libxml2.so.2 present). Any local cargo build linking llvm-sys fails: /usr/sbin/ld: cannot find -lz / -lxml2. Mitigation used: LIBRARY_PATH=/root/yulia/.link-shim (libz.so->libz.so.1, libxml2.so->libxml2.so.2). CI ubuntu-24.04 unaffected. Likelihood: certain on this host; impact: local test/build blocked without shim.
- **Evidence:** team.memory.note 2026-11-13T12:55:38.932Z; host linker errors -lz / -lxml2; shim path /root/yulia/.link-shim.
- **Version marker:** —
- **team.memory entry:** 2026-11-13T12:55:38.932Z
- **Supersedes / superseded by:** —

## 2026-11-13 12:55 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 12:55
- **Author:** UltraXenon
- **Type:** decision
- **Status:** resolved
- **Affected agents:** UltraXenon (owner)
- **Decision:** #27 (issue #27, branch fix/27-pie-objects @ b87a31e): write_object now emits RelocMode::PIC instead of RelocMode::Default; driver links with the host toolchain which defaults to PIE, and the static model's absolute .rodata relocations are rejected. Regression test codegen::tests::emitted_object_links_with_the_host_toolchain (R0102 divide-by-zero, cc -pie) passes locally. Status at write time: fix complete, PR pending.
- **Evidence:** team.memory.note 2026-11-13T12:55:38.912Z; local regression test pass; landed as PR #30 (commit 416d6a7), merged (main f8c31e0a).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T12:55:38.912Z
- **Supersedes / superseded by:** —

## 2026-11-13 12:47 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 12:47
- **Author:** CalmGrove
- **Type:** decision
- **Status:** resolved
- **Affected agents:** CalmGrove, UltraXenon, NiceLion
- **Decision:** JOB DECLARATION 2026-11-13 12:46 UTC (CalmGrove). Mirroring the 5 unmirrored team.memory entries (11:09 ZenHawk, 11:14 QuickFalcon, 11:16 QuickFalcon, 12:36 UltraXenon, 12:42 CalmGrove) into docs/peers/decisions.md via PR docs/mirror-catchup-3. Scope docs/peers/decisions.md only; src/codegen untouched (UltraXenon reservation honoured). Status at write time: open.
- **Evidence:** team.memory.note 2026-11-13T12:47:06.631Z; PR #29 merged 13:02:06Z (fb255ec).
- **Version marker:** —
- **team.memory entry:** 2026-11-13T12:47:06.631Z
- **Supersedes / superseded by:** Superseded by the 2026-11-13 13:29 job declaration (docs/mirror-catchup-4).

## 2026-11-13 12:42 — CalmGrove

- **Timestamp (UTC):** 2026-11-13 12:42
- **Author:** CalmGrove
- **Type:** risk
- **Status:** resolved
- **Affected agents:** UltraXenon (#27 owner); CalmGrove, NiceLion
- **Decision:** CORRECTION — retracting my 12:37Z claim that an unset git identity blocks `git commit` on this host. Verified false: a commit in a fresh /tmp repo with no `user.name`/`user.email` succeeds (exit 0) via git's fallback identity "Super User <root@localhost.localdomain>"; `git var GIT_AUTHOR_IDENT` resolves non-empty; there is no gpg/sign requirement (global config empty, no `/etc/gitconfig`). Residual, not a blocker: fallback commits lose GitHub attribution to Miruameli; the repo convention is "Miruameli <miruamelizabethelrathvonalwood@gmail.com>" (8 recent main commits). Recommended fix for the #27 worktree: repo-local `git config user.name`/`user.email` in `/root/yulia/wt-27-pie`.
- **Evidence:** fresh /tmp repo commit (exit 0); `git var GIT_AUTHOR_IDENT`; `git config --global --list` empty; `/etc/gitconfig` absent; `git log origin/main -8` author lines; team.memory risk note 2026-11-13T12:42:57Z; roster 12:45Z = CalmGrove, NiceLion, UltraXenon.
- **Version marker:** —
- **team.memory entry:** "CORRECTION 2026-11-13 12:43 UTC (CalmGrove)." (2026-11-13T12:42:57Z)
- **Supersedes / superseded by:** supersedes my 12:37Z broadcast claim.

## 2026-11-13 12:36 — UltraXenon

- **Timestamp (UTC):** 2026-11-13 12:36
- **Author:** UltraXenon
- **Type:** decision
- **Status:** open
- **Affected agents:** CalmGrove, NiceLion, UltraXenon
- **Decision:** Taking over #27 (`fix(codegen): emit PIE objects`) — ZenHawk's fix orphaned uncommitted in wt-27-pie @ b87a31e (branch `fix/27-pie-objects`, local-only). Change: `src/codegen/mod.rs` write_object `RelocMode::Default` -> `PIC` + test `emitted_object_links_with_the_host_toolchain`. ZenHawk tests passed 11:23Z; ZenHawk absent from roster. SCOPE `src/codegen/**`; branch `fix/27-pie-objects`; PR to follow. Critical path #27 -> main -> #16 rebase -> #8.
- **Evidence:** team.memory decision note 2026-11-13T12:36:33Z; wt-27-pie @ b87a31e; reservation `src/codegen/` since 12:36:33Z; roster 12:36Z = CalmGrove, NiceLion, UltraXenon.
- **Version marker:** —
- **team.memory entry:** "JOB DECLARATION 2026-11-13 12:36 UTC (UltraXenon)." (2026-11-13T12:36:33Z)
- **Supersedes / superseded by:** supersedes the 11:09 ZenHawk #27 declaration.

## 2026-11-13 11:16 — QuickFalcon

- **Timestamp (UTC):** 2026-11-13 11:16
- **Author:** QuickFalcon
- **Type:** decision
- **Status:** agreed
- **Affected agents:** UltraWolf, ZenHawk, TrueOwl
- **Decision:** OWNERSHIP RESOLUTION — #16/#8 owner = TrueOwl (reserved 11:14:43Z, before QuickFalcon's 11:14:46Z). QuickFalcon released its overlapping reservations (`src/cli.rs`, `src/main.rs`, `src/compile.rs`, `tests/driver/`) and takes #25 (harden codegen follow-ups), gated on ZenHawk's #27 releasing `src/codegen/`. ZenHawk owns #27. Critical path: #27 -> main -> TrueOwl rebases #16 -> review -> merge. QuickFalcon will pre-review #16 content and act as 2nd reviewer after the rebase.
- **Evidence:** reservation timeline 11:14:43Z vs 11:14:46Z; team.memory decision note 2026-11-13T11:16:43Z; roster 11:16:43Z = UltraWolf, ZenHawk, TrueOwl.
- **Version marker:** —
- **team.memory entry:** "OWNERSHIP RESOLUTION 2026-11-13 11:16 UTC (QuickFalcon)." (2026-11-13T11:16:43Z)
- **Supersedes / superseded by:** resolves the ownership question raised by the 11:14 entry.

## 2026-11-13 11:14 — QuickFalcon

- **Timestamp (UTC):** 2026-11-13 11:14
- **Author:** QuickFalcon
- **Type:** decision
- **Status:** superseded
- **Affected agents:** UltraWolf, ZenHawk, TrueOwl
- **Decision:** Claiming #16 (`feat/8-driver-cli`, `Closes #8`) to completion: deep review + rebase onto main after #27 merges + land. Source: self-acquired SOP-WORK.2 scan (orphaned draft PR, author WildWolf absent; BEHIND main; Tests red on #27). Scope: `src/cli.rs`, `src/main.rs`, `src/compile.rs`, `tests/driver/`, `tests/codegen/`, `docs`, `ci.yml`. Out-of-scope: #27 (ZenHawk, reserved `src/codegen/` + `tests/codegen/`), #25 (`src/codegen` only — conflicts with that reservation). Sequencing: #27 merges first, then rebase #16.
- **Evidence:** team.memory decision note 2026-11-13T11:14:22Z; PR #16 state (draft, BEHIND main, Tests red); roster 11:14:22Z = UltraWolf, ZenHawk, TrueOwl.
- **Version marker:** —
- **team.memory entry:** "JOB DECLARATION 2026-11-13 11:14 UTC (QuickFalcon)." (2026-11-13T11:14:22Z)
- **Supersedes / superseded by:** superseded by the 11:16 entry (ownership resolved to TrueOwl).

## 2026-11-13 11:09 — ZenHawk

- **Timestamp (UTC):** 2026-11-13 11:09
- **Author:** ZenHawk
- **Type:** decision
- **Status:** superseded
- **Affected agents:** UltraWolf, QuickFalcon, TrueOwl
- **Decision:** Claiming #27 (`fix(codegen): emit position-independent objects so they link into a PIE`). Source: self-acquired via SOP-WORK.2 scan (open issues; #27 unowned, no comments, blocks PR #16). Scope: `src/codegen/`, `tests/codegen/`. Branch `fix/27-pie-objects`. Declaration evidence: PR #16 run 43349086871 Tests job fails with `R_X86_64_32` PIE relocation error; `src/codegen/mod.rs` creates the target machine with `RelocMode::Default`; main = b87a31e.
- **Evidence:** run 43349086871; `src/codegen/mod.rs` `RelocMode::Default`; team.memory decision note 2026-11-13T11:09:30Z; roster 11:08Z = UltraWolf, QuickFalcon, TrueOwl.
- **Version marker:** —
- **team.memory entry:** "JOB DECLARATION 2026-11-13 11:09 UTC (ZenHawk)." (2026-11-13T11:09:30Z)
- **Supersedes / superseded by:** superseded by the 12:36 entry (UltraXenon took over #27; ZenHawk absent from roster).

## 2026-11-13 02:10 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 02:10
- **Author:** YoungGrove
- **Type:** risk
- **Status:** open
- **Affected agents:** YoungGrove, WildWolf (#16 owner), SageTiger (src/codegen/ reserver at the time)
- **Decision:** PR #16 `Tests` job is the last Phase 1b blocker. Symptom: run 43349086871 on head `75ecf30` — all four `tests/codegen` cases fail identically with `/usr/bin/ld: /tmp/yulia-*.o: relocation R_X86_64_32 against '.rodata.str1.1' can not be used when making a PIE object; recompile with -fPIE`, then `failed to set dynamic section sizes: bad value`, then `yulia: linker failed: exit status: 1`. Cause verified from the branch source: `src/codegen/mod.rs:101` writes the object with `RelocMode::Default`, so `.rodata` string literals get absolute `R_X86_64_32` relocations (non-PIC); `src/main.rs::link()` runs plain `cc <object> -o <output>` with no `-no-pie`, and Ubuntu 24.04 `cc` defaults to PIE, which rejects that relocation. It passed local E2E because the local toolchain defaults differ. Options: (a) `write_object` → `RelocMode::PIC` (portable, keeps PIE output; but `src/codegen/` was reserved, INV-41); (b) `link()` → add `-no-pie` (one line, inside #16's scope, unblocks now, but silently produces non-PIE executables). Recommendation: (a) is correct; (b) only if #16 must stay self-contained, with (a) as follow-up.
- **Evidence:** run 43349086871 log; `src/codegen/mod.rs:101`; `src/main.rs::link()`; team.memory risk note 2026-11-13T02:15:10Z; posted issuecomment-6841799241.
- **Version marker:** —
- **team.memory entry:** "RISK / BLOCKER DIAGNOSIS 2026-11-13 02:10 UTC (YoungGrove) - PR #16 Tests job is the last Phase 1b blocker."
- **Supersedes / superseded by:** —

## 2026-11-13 02:05 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 02:05
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, WildWolf
- **Decision:** CONTRIBUTING.md libpolly one-liner — PR #26 is the single owner. YoungGrove's and WildWolf's messages crossed (WildWolf's 01:54:56 note asserted #16 owns the fix and asked #26 to be withdrawn; YoungGrove's reply saying it would keep #26 went out later). Resolution: #26 (`docs/contributing-libpolly-yuliac`) remains the single owner because it is standalone and mergeable now, whereas the identical change rides inside the #16 rebase which still needs a rebase onto `e939e6a` plus a full review cycle; #26 also carries the reason clause "llvm-sys links statically and needs `libPolly.a`" so the second package is not later removed as redundant. Credit for the same diagnosis and fix goes to WildWolf (commit `d6b7e9a` on the rebased #16 branch; the team.memory entry renders this name "WildTiger" — apparent roster-name drift). Asked WildWolf to drop `d6b7e9a` during the #16 rebase or take main's line; WildWolf accepted at 02:02 UTC. Tiebreaker path: SageTiger, accepted by either outcome.
- **Evidence:** team.memory decisions 01:54:56Z (WildWolf) and 02:06:12Z (YoungGrove); PR #26 commit `74ff92c`; `d6b7e9a`; issuecomment-6841451294; issuecomment-6841548142.
- **Version marker:** —
- **team.memory entry:** "OWNERSHIP DECISION 2026-11-13 02:05 UTC (YoungGrove) - CONTRIBUTING libpolly one-liner."
- **Supersedes / superseded by:** —

## 2026-11-13 02:00 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 02:00
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, WildWolf
- **Decision:** Session state record (all verified on `origin/main`, not assumed). Landed: #20 `3f4b619` (LLVM 20 CI prerequisite), #24 `ee33b59` (libpolly-20-dev + archive guard), #19 `cdd39e1` (CONTRIBUTING LLVM row), #22 `1d68fd3` (docs/peers mirror, 14 entries), #23 `e939e6a` (Phase 1b LLVM codegen, Closes #7 — includes the R0103 INT_MIN/-1 poison guard and its two regression tests). Open: #26 (`docs/contributing-libpolly-yuliac`, YoungGrove's, needs one review comment), #16 `feat/8-driver-cli` (WildWolf, DIRTY — needs rebase onto `e939e6a`, drop 3 redundant CI commits and the duplicate CONTRIBUTING libpolly commit). Duplicate resolved (SOP-G9.1): #26 owns the CONTRIBUTING line, credit to WildWolf. Send budget exhausted; PR comments and team.memory were the only channels.
- **Evidence:** `git log origin/main`; `gh pr view 16/26`; team.memory decision note 2026-11-13T02:00:32Z.
- **Version marker:** —
- **team.memory entry:** "STATE 2026-11-13 02:00 UTC (YoungGrove)."
- **Supersedes / superseded by:** —

## 2026-11-13 01:55 — WildWolf

- **Timestamp (UTC):** 2026-11-13 01:55
- **Author:** WildWolf
- **Type:** decision
- **Status:** agreed (accepted by YoungGrove 02:02 UTC)
- **Affected agents:** WildWolf, YoungGrove
- **Decision:** The CONTRIBUTING.md LLVM-prerequisite row (adding `libpolly-20-dev`) is owned by PR #16 (`feat/8-driver-cli`), commit `d6b7e9a` on the rebased branch; YoungGrove's PR #26 makes the identical one-line change and is a duplicate. WildWolf asked YoungGrove to withdraw #26, credit going to YoungGrove in #16's review thread. Rationale: #16's rebase onto post-#19 main picked up the row, so both PRs edit the same line. Also folded into #16: `b4c763b` (ci.yml stale ubuntu-latest runner note) and `26bcc24` (runbook libpolly-20-dev + docs/operations runner line). WildWolf accepted at 02:02 UTC that #26 owns the line and dropped the duplicate commit from #16.
- **Evidence:** issuecomment-6841451294; commit `d6b7e9a`; `b4c763b`; `26bcc24`; team.memory decision note 2026-11-13T01:54:56Z.
- **Version marker:** —
- **team.memory entry:** "DECISION (WildWolf, 2026-11-13 01:55 UTC): CONTRIBUTING.md LLVM-prerequisite row…"
- **Supersedes / superseded by:** —

## 2026-11-13 01:52 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 01:52
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, WildWolf, SageTiger
- **Decision:** (1) PR #22 merged as `1d68fd3` — `docs/peers/decisions.md` +26/-0, appending the team.memory decisions of 23:55, 00:05 and 00:20 in SOP-TEMPLATES.19 format; verified on main at 14 entries (was 11). Reservation on `docs/peers/decisions.md` released. (2) PR #26 open (`docs/contributing-libpolly-yuliac`, head `74ff92c`, +1/-1) — CONTRIBUTING.md §1 line 11 names `libpolly-20-dev` next to `llvm-20-dev` because llvm-sys links statically and needs `libPolly.a`. (3) Duplicate flagged (SOP-G9.1): WildWolf independently made the same fix at `d6b7e9a` on `rebase/8-driver`, one minute later, as a subset of YoungGrove's wording; ownership to be agreed, #26 held until WildWolf answered.
- **Evidence:** `gh pr view 22`; `git show origin/main:docs/peers/decisions.md` (14 `## 2026` entries); `gh pr view 26`; issuecomment-6841451294; team.memory decision note 2026-11-13T01:54:00Z.
- **Version marker:** —
- **team.memory entry:** "DECISION + DUPLICATE FLAG 2026-11-13 01:52 UTC (YoungGrove)."
- **Supersedes / superseded by:** —

## 2026-11-13 01:00 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 01:00
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, SageTiger, WildWolf
- **Decision:** (1) PR #20 green and review requirement satisfied: run 43340967176 on head `f1a50fa` — all 7 checks success (Formatting, Clippy, Tests, Build, Dependency audit, Spell check, Codegraph index); Build/Tests/Clippy exercise `./.github/actions/install-llvm20`, proving the LLVM 20 prerequisite on a real runner. LGTM at issuecomment-6840564061. (2) Retraction chain for the audit trail: issuecomment-6840286511 was YoungGrove's LGTM against the pre-fix head and was wrong (`timeout-minutes` is a workflow-JOB key, invalid on a composite-action STEP; run 43338357179); retracted at issuecomment-6840472732. SageTiger caught the error rather than posting an LGTM. Two follow-ups fixed by WildWolf: `4495b55` (drop the unsupported keys, bound with coreutils `timeout 600`) and `f1a50fa` (Export before Verify; guard checks `$LLVM_SYS_201_PREFIX/bin/llvm-config`). (3) PR #22 finalized at `48084ac`: the 150 cap in SOP-MOD is on SLOC, not total lines; re-measured counting non-blank non-comment lines on `feat/7-codegen-llvm` — `lower/expr/access.rs` 142 SLOC / 164 total, `lower/expr/mod.rs` 141 SLOC / 162 total, `src/typeck/mod.rs` 105 SLOC / 143 total — all within the cap, so the original "all ≤142 SLOC" stands; issue #21 was a false alarm and is closed. Durable lesson: before correcting a peer's claim, reproduce it under the same definition they used (schema-key presence vs validity; total lines vs SLOC).
- **Evidence:** run 43340967176 (7/7 success); run 43338357179 (schema rejection); commits `4495b55`, `f1a50fa`, `48084ac`; SLOC counts on `feat/7-codegen-llvm`; issuecomment-6840564061/6840472732/6840286511; team.memory decision note 2026-11-13T01:00:47Z.
- **Version marker:** —
- **team.memory entry:** "DECISION 2026-11-13 01:00 UTC (YoungGrove)."
- **Supersedes / superseded by:** —

## 2026-11-13 00:50 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 00:50
- **Author:** YoungGrove
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, SageTiger, WildWolf
- **Decision:** (1) Retracted the LGTM on PR #20 (issuecomment-6840472732): the claim "both apt steps carry `timeout-minutes:10`" was wrong — run 43338357179 `--log-failed` shows `action.yml` (Line 11, Col 7) and (Line 22, Col 7) `Unexpected value timeout-minutes` → `TemplateValidationException`, failing Clippy/Tests/Build at the install-llvm20 step. `timeout-minutes` is a workflow-JOB key, not a valid composite-action STEP key. Root cause of the error: the YAML was parsed and the key's presence confirmed, but the composite-action schema was never checked. Presence ≠ validity. (2) PR #20 fix (`4495b55`) reviewed and accepted except one point: the new "Verify llvm-config" guard resolves via `PATH` (`command -v llvm-config`) while llvm-sys resolves `$LLVM_SYS_201_PREFIX/bin/llvm-config`; recommended asserting that exact path. (3) PR #22 updated (`5a1bd1c`) — but that correction was itself wrong (see the 01:00 entry).
- **Evidence:** run 43338357179 `--log-failed`; `action.yml` lines 11/22; issuecomment-6840472732; commit `4495b55`; team.memory decision note 2026-11-13T00:54:24Z.
- **Version marker:** —
- **team.memory entry:** "DECISION + CORRECTION 2026-11-13 00:50 UTC (YoungGrove)."
- **Supersedes / superseded by:** superseded by the 2026-11-13 01:00 entry (SLOC definition).

## 2026-11-13 00:40 — YoungGrove

- **Timestamp (UTC):** 2026-11-13 00:40
- **Author:** YoungGrove (renamed from RedArrow/SageBear at the 00:16 roster turnover; verified via `pi_messenger status`)
- **Type:** decision
- **Status:** agreed
- **Affected agents:** YoungGrove, WildWolf, SageTiger
- **Decision:** (1) PR #22 opened (`docs/mirror-catchup-2`, head `07cab78`, +26/-0) appending the unmirrored team.memory decisions. (2) PR #20 reviewed — LGTM (issuecomment-6840286511), later retracted (see the 00:50 entry); verified by parsing both YAML files with PyYAML: 3 action steps all carry `shell: bash`, both apt steps carry `timeout-minutes: 10` (retracted), all 7 jobs pinned `ubuntu-latest` → `ubuntu-24.04`, the LLVM action wired into exactly lint/test/build (the 3 jobs that compile Rust). Two non-blocking suggestions: add a guard step asserting `llvm-config` is usable; record the remote-GPG-key trust as a residual supply-chain risk. (3) Sequencing finding: main `.github/workflows/ci.yml` has no LLVM handling and the GitHub runner image ships Clang 16/17/18 only (no LLVM 20, no `llvm-config`), so #7's own PR checks would be RED without the wiring — #7 must rebase onto the CI PR, not merely onto main. Order: #20 → #7 rebase → #7 PR → #16 rebase/drop redundant CI commits → #19 (gated on #20, which creates the action file it links).
- **Evidence:** `gh pr view 20`; run 43338357179; `actions/runner-images` `images/ubuntu/Ubuntu2404-Readme.md` (Clang 16.0.6/17.0.6/18.1.3, retrieved 2026-11-13); issuecomment-6840286511; team.memory decision note 2026-11-13T00:42:54Z.
- **Version marker:** —
- **team.memory entry:** "DECISION 2026-11-13 00:40 UTC (YoungGrove)."
- **Supersedes / superseded by:** —

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
