-- mcp.db — a domain document's Section Map (shape per Bodha's
-- `section-map.yaml`): an ordered, self-referencing tree of sections and
-- subsections. Scoped by `domain_id` into domain (05) — each domain has
-- its own Section Map. Subsections nest under sections via `parent_id`;
-- `required: 0` marks optional sections and optional sub-headings. So "what
-- sections does domain X require, and which are optional" is a direct
-- indexed lookup.
--
-- `section_id` is the YAML-declared id within the map (distinct from this
-- table's pk `id`). `generated` marks sections the author flags as
-- machine-generated, `source` records which file the entry came from, and
-- `profile_ref` names the Section Profile key the entry uses (the
-- structured profile itself lives in section_profile, 07). `content_asset_
-- id` traces the parsed section-map file into the capture ledger.

CREATE TABLE IF NOT EXISTS section (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_id        INTEGER NOT NULL REFERENCES domain(id) ON DELETE CASCADE,
    section_id       TEXT    NOT NULL,  -- YAML-declared id in the Section Map
    title            TEXT    NOT NULL,
    parent_id        INTEGER REFERENCES section(id),  -- subsections nest under sections
    level            INTEGER NOT NULL DEFAULT 0,
    sort_order       INTEGER NOT NULL DEFAULT 0,
    required         INTEGER NOT NULL DEFAULT 1,
    generated        INTEGER NOT NULL DEFAULT 0,
    source           TEXT    NOT NULL DEFAULT '',
    profile_ref      TEXT,  -- named Section Profile key; structured profile in 07
    purpose          TEXT    NOT NULL DEFAULT '',
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),  -- the section-map file
    UNIQUE(domain_id, section_id)
);
CREATE INDEX IF NOT EXISTS idx_section_domain ON section(domain_id);
CREATE INDEX IF NOT EXISTS idx_section_parent ON section(parent_id);
