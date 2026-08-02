# Proposal: Bodha Section Format Reference

> Status: Draft — reference-only, no schema change. Conforms to `docs/raw/proposal.md` standard.
> **Resolves** the "Bodha's `.bodha-structure/section` and `profile-default` are cited but not vendored or linked" item in `00-overview.md`'s What Is Still Open. **Frozen as observed** — per explicit instruction, this document captures Bodha's actual format exactly as it exists today and does not propose changing it; any revision to the format itself is deferred to a later phase.

## Purpose

This document is the formal External Context reference `docs/proposal/05-domain-system-registration.md` cites but never pinned: the exact, observed shape of Bodha's Section Map (`section/map/section-map.yaml`) and Section Profile (`section/profile/*.yaml`, `section/profile-default/*.yaml`) files, read directly from `E:\Python\Bodha\.bodha-structure\` rather than assumed.

Unlike a redesign, this document changes nothing about the format — it exists so that "the Section Map/Profile shape in proposal 05 assumes Bodha's structure is stable and reusable as-is" is no longer an unverified assumption. Where Bodha's real files carry more than `schema/mcp/06-section.sql`/`07-section_profile.sql` currently capture, that gap is named explicitly and deferred, not fixed here.

## System Overview

### Overview

Bodha's format has three layers: a single **Section Map** per document type (here, a research paper) listing every section/subsection with hierarchy, ordering, and a profile reference; a **Section Profile** per section (or subsection) stating writing/knowledge/reader objectives, required inputs, per-subsection rules, and completion/review/validation checklists; and a **Section Profile Default** (e.g. `scientific-narrative`) that every profile inherits from unless it overrides a field, carrying the bulk of shared writing/quality/validation rules so individual profiles stay short.

### Structural Approach

The Map is authored once per document type and references Profiles by key (`profile: introduction`); Profiles reference a Profile Default by name (`inherits: [scientific-narrative]`). Dharma's capture flow (proposal 08) parses both file kinds into `section`/`section_profile` rows scoped by `domain_id`, keeping each YAML's full text in `content_asset` for lossless reconstruction — this document does not change that flow, only pins what it's parsing against.

### Diagram

```text
section-map.yaml (one per Domain, e.g. research-paper)
    │ sections[] each with: id, title, parent_id, level, order,
    │                       required, generated, source, profile, purpose
    │ + paper_type/supported_types (doc-type variants)
    │ + validation{} (hierarchy/ordering/structure/required rules, MAP-level)
    ▼
section/profile/<key>.yaml (one per section, e.g. introduction.yaml)
    │ section{ id, trigger, writing_objective, knowledge_goal, reader_goal,
    │          required_inputs[], expected_outputs{primary[]},
    │          subsections{ <key>: {objective, writing_guidelines[],
    │                                should_answer[], transition_to} },
    │          completion.checklist[], review.questions[], validation.rules[] }
    │ inherits: [scientific-narrative]
    ▼
section/profile-default/<name>.yaml (e.g. scientific-narrative.yaml)
    profile{ category, extends, applies_to[], writing{}, narrative{},
             scientific{}, reasoning{}, evidence{}, assets{}, constraints{},
             quality{}, completion{}, review{}, validation{},
             inheritance{ allow_override[], prohibit_override[] } }
```

## Component Model

### Section Map (observed shape)
- **Fields per entry:** `id`, `title`, `parent_id` (`root` for top-level), `level`, `order`, `required`, `generated`, `source` (optional, e.g. `metadata.paper.title`), `profile` (key into Section Profile), `purpose`.
- **Document-type metadata:** `paper_type.default` and `supported_types` (e.g. `research-paper`, `review-paper`, `survey-paper`, `short-paper`, `technical-report`, `thesis`) — the Map itself declares which document-type variants it's valid for.
- **Map-level `validation` block:** `hierarchy.max_level`, `hierarchy.require_valid_parent`, `hierarchy.root_parent_id`, `ordering.unique_order_per_parent`, `structure.duplicate_ids`/`orphan_sections`/`circular_references`, `required.enforce_required_sections`/`enforce_required_subsections` — structural rules for the tree as a whole, not any one section.
- **Currently captured by schema:** `section_id`, `title`, `parent_id`, `level`, `sort_order`, `required`, `generated`, `source`, `profile_ref`, `purpose` (`schema/mcp/06-section.sql`) — a faithful field-for-field match.
- **Not yet captured by schema:** `paper_type`/`supported_types` and the map-level `validation` block have no column anywhere in `schema/mcp/05-domain.sql` or `06-section.sql` today. The full original YAML is still preserved losslessly in `content_asset`, so nothing is lost — these two just aren't queryable as structured rows yet. Deferred, per this document's framing, to a later phase.

### Section Profile (observed shape)
- **Top-level:** `schema.id`/`name`/`version`, `inherits` (list — Bodha's file uses a one-element list; `schema/mcp/07-section_profile.sql`'s `inherits` column is a single TEXT value, a simplification).
- **`section` block:** `id`, `trigger` (e.g. `after_research_complete` — not currently captured anywhere), `writing_objective`, `knowledge_goal`, `reader_goal`, `required_inputs[]`, `expected_outputs.primary[]`, `subsections{<key>: {objective, writing_guidelines[], should_answer[], transition_to}}`, `completion.checklist[]`, `review.questions[]`, `validation.rules[]`.
- **Currently captured by schema:** `writing_objective`, `knowledge_goal`, `reader_goal`, `required_inputs_json`, `expected_outputs_json`, `subsection_rules_json` (the `subsections` map, flattened to JSON), `completion_checklist_json`, `review_questions_json`, `validation_rules_json`, `inherits` — a faithful match, with `subsections` and `expected_outputs.primary` stored as JSON rather than further-normalized columns.
- **Not yet captured by schema:** `section.trigger` has no column. Deferred.

### Section Profile Default (observed shape)
- **`profile` block:** `category`, `extends` (nullable — a default profile may itself extend another), `applies_to[]` (which section keys use this default), then nine rule groups — `writing` (tone/voice/perspective/language.style/preferred/avoid), `narrative` (paragraphs/transitions/progression), `scientific` (requirements/terminology), `reasoning` (requirements), `evidence` (citations/quantitative_results/comparisons/assumptions), `assets` (managed_by_section), `constraints` (prohibited/required), `quality` (readability/scientific/writing), `completion.checklist`, `review` (scientific/writing/reader questions), `validation` (required + rules.scientific/writing/structure) — plus `inheritance.allow_override[]`/`prohibit_override[]` naming which of the nine groups (and `schema`/`profile.category`/`profile.extends`) a profile may or may not override.
- **Currently captured by schema:** Nothing — `schema/mcp/07-section_profile.sql`'s `inherits` column names a Profile Default by string; the Default's own content is not parsed into rows anywhere in the current schema. The full YAML is preserved in `content_asset` via the Default's own capture (as a `yaml_template`-eligible asset), so a reconstruction is possible, just not queryable per-field today.
- **Not yet captured by schema:** The entire Profile Default rule-group structure, and `inheritance.allow_override`/`prohibit_override`. This is the largest of the three deferred gaps — noted, not fixed, per this document's scope.

## Communication

### Communication Paths

**Bodha (provider) → Capture flow**
- **Pattern:** Synchronous, at capture time (unchanged from proposal 08).
- **Contract:** All three file kinds (Map, Profile, Profile Default) are walked from `dharma-domain.toml`'s declared content root, hashed, and stored in `content_asset`; Map and Profile rows are additionally parsed into `section`/`section_profile`. Profile Default files are captured (`content_asset`) but not currently parsed into structured rows (see Component Model gaps above).

### Communication Diagram

```text
Bodha files → Capture : hash + store(content_asset)
Bodha section-map.yaml → Capture : parse → section rows (per Component Model's "currently captured" list)
Bodha profile/*.yaml → Capture : parse → section_profile rows (per Component Model's "currently captured" list)
Bodha profile-default/*.yaml → Capture : store only (content_asset); no structured parse yet
```

## Data Flow

### Data Paths

**Reference-Verification Path**
- **Entry point:** This document's own authoring — reading Bodha's real files directly rather than trusting a remembered description.
- **Transformations:** None to Bodha's files. This document only records what was found.
- **Ownership boundary:** Bodha owns its files; this document owns only the observation of their shape at the time of writing.
- **Exit point:** A named, versioned reference (`schema.id`/`version` fields observed: `bodha.paper.section-map` v1.0.0, `bodha.section.profile.introduction` v1.0.0, `bodha.section.profile.defaults.scientific-narrative` v1.0.0) that proposal 05 and `schema/mcp/06-`/`07-` can be checked against.

### Data Flow Diagram

```text
E:\Python\Bodha\.bodha-structure\section\{map,profile,profile-default}\*.yaml
        │ read directly (this document)
        ▼
Observed shape recorded here ──compared against──▶ schema/mcp/06-section.sql, 07-section_profile.sql
                                                          │
                                                          ▼
                                          gaps named (paper_type/supported_types,
                                          map-level validation, profile trigger,
                                          Profile Default structured content) —
                                          deferred, not fixed, this revision
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Bodha's actual `.yaml` files | Bodha (the provider repository) |
| This document's observation of their shape | This document, versioned against the `schema.id`/`version` values observed above |
| The parsed `section`/`section_profile` rows | `mcp.db`, per proposal 08 — currently a partial capture, per Component Model |

## Security

### Trust Boundaries

- **Bodha's files → this document:** Read-only observation; nothing here writes back to Bodha or to `mcp.db`.
- **This document → future implementation:** Advisory — it names gaps but does not mandate closing them in this revision; a later phase decides what, if anything, changes.

### Threat Model

- **Format drift after this document is written:** Bodha's files change and this reference goes stale. Mitigation: this document cites the exact `schema.id`/`version` triples observed; a version bump on Bodha's side is the signal to re-verify, not a silent assumption of continued compatibility.
- **Treating "not yet captured" as "never needed":** A future implementer reads the deferred gaps as permanently out of scope. Mitigation: each gap is stated as deferred to "a later phase," not rejected — see Constraints.

## Rationale

### Freeze the Reference, Defer the Redesign
- **Context:** The open item this document resolves was about verifying an assumption, not about whether Bodha's format is optimal.
- **Decision:** Record the format exactly as observed; explicitly do not propose changes to it in this revision.
- **Alternatives Considered:** Redesign the Section Map/Profile shape now, while already looking closely at it.
- **Rejection Reason:** Explicit instruction: keep the same structure and format for now, modify only in later phases if needed. Redesigning now would also block on decisions (e.g., should `trigger` become a first-class Task Runtime concept?) that don't need to be made to unblock implementation.
- **Architectural Goal:** Close the open verification item without opening a new, larger design question prematurely.

### Name Every Gap Instead of Silently Dropping Fields
- **Context:** `content_asset` already preserves every field losslessly as text, so no data is actually lost by an incomplete structured parse.
- **Decision:** Explicitly list every field the current schema doesn't parse into rows (`paper_type`/`supported_types`, map-level `validation`, profile `trigger`, the entire Profile Default rule-group structure and its `inheritance` overrides), rather than letting the gap be discovered later by surprise.
- **Alternatives Considered:** Only document what's captured, treat the rest as an implementation detail to discover later.
- **Rejection Reason:** A gap discovered during implementation costs a mid-implementation schema change; a gap named now costs one sentence.
- **Architectural Goal:** No implementation-time surprise about what Bodha's format actually contains.

## Constraints

### Hard Constraints
- **No format change in this revision** (source: Rationale above) — this document must not be read as authorizing a Section Map/Profile redesign.
- **Every named gap stays queryable via `content_asset`** (source: Threat Model above) — even unparsed fields remain reconstructable from the captured YAML text.

### Soft Constraints
- Prefer closing the named gaps (Profile Default structured parse, in particular — the largest) in the same later phase, rather than piecemeal, since they share one underlying cause (Profile Default was captured but never modeled).

## Traceability

### Derivation Chain

```text
05-domain-system-registration (cites Bodha's format, previously unverified)
    │
    ▼
Bodha Section Format Reference (this document) — verifies against the real files
    │
    ▼
(a later phase, if the named gaps are worth closing — not gated by this document)
```

### Non-Contradiction Rule

No downstream document may claim Bodha's Section Map/Profile format was redesigned by this revision, or treat a gap named here as silently resolved without a schema change that actually parses the field into rows.

## Related

- [Proposal 05 — Domain System Registration](05-domain-system-registration.md) — the proposal this reference resolves the open item for
- [Proposal 08 — Schema & Crate Architecture](08-schema-and-crate-architecture.md) — the Domain System Content Model and capture flow this document checks against
- `E:\Python\Bodha\.bodha-structure\section\` — the actual files observed
