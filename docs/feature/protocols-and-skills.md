# Module: Protocols and Skills

**Directory Mapping:** `src/registry/protocols/` and `src/registry/skills/`

## Dual-Format Parsers
Unlike static JSON configurations elsewhere in Dharma, Skills and Protocols typically contain significant prompt contexts. The loader aggressively supports a mix of JSON constructs or Markdown files employing strictly typed YAML-frontmatter schemas.

## Component Roles
- **Skills**: Act as functional encyclopedias of system abilities. Workflows track these explicitly inside dependency graphs.
- **Protocols**: Strict operational behavioral mandates. These evaluate linearly above standard contextual limits during agent processing blocks.
