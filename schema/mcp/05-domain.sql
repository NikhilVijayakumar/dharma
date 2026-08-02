-- mcp.db — the domains a Domain System carries (e.g. base_dev carries 16:
-- vision, philosophy, security, feature, architecture, design,
-- engineering, external-context, feature-design, feature-technical,
-- prototype, qa, implementation, build, readme, product-guide; rust_dev
-- extends it and drops three). Each domain is one document — its own
-- Section Map (06), its own Section Profile set (07), optionally a
-- relationship/tier position. `domain` rows scope every document-level
-- structure that follows: `section.domain_id`, `section_profile.section_id
-- → section.domain_id`.
--
-- `content_asset_id` traces the domain's declaration file into the capture
-- ledger — every content row must trace to a captured provider file
-- (docs/proposal/08-schema-and-crate-architecture.md, Hard Constraints).

CREATE TABLE IF NOT EXISTS domain (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    description      TEXT    NOT NULL DEFAULT '',
    tier             TEXT,  -- optional tier position
    relationship     TEXT,  -- optional cross-domain relationship
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),  -- the domain declaration file
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_domain_domain_system ON domain(domain_system_id);
