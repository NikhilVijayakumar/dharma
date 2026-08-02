---
id: openclaw-feature-extraction-parameters
title: OpenCLAW Feature Extraction Parameters
tags: [extraction, openclaw, registry, hybrid]
---
# OpenCLAW Feature Extraction Parameters

## Purpose
Defines extraction parameters required to convert OpenCLAW runtime patterns into deterministic registry metadata.

## Feature Extraction Parameters
- stage_protocol: stage_init -> stage_plan -> stage_execute -> stage_synthesis.
- model_fallback_chain: ordered providers with cooldown and retry strategy.
- deep_persona_fields: personality_traits, interaction_style, constraints, delegation_limits.
- context_budget: max_tokens, reserved_output_tokens, compact_threshold_tokens.
- subagent_controls: max_depth, timeout_ms, heartbeat_interval_ms, cycle_detection.
- approval_controls: requires_approval flags and high-risk action categories.

## Deep Persona Mapping
- Personality traits must be represented as immutable guidance unless explicitly overridden by approved workflow state.
- Constraints must be represented as hard execution guards and evaluated before stage_execute.
- Interaction style must influence synthesis verbosity but never bypass governance rules.

## Output Contract
- Registry-ready parameter map with source evidence and normalized field names.
- Compatibility notes identifying additive-only migration behavior.
- Validation checklist mapping each field to schema support.

## Quality Gates
- Parameters are executable, typed, and traceable to source pattern.
- Deep persona constraints are enforceable during plan and execute stages.
- Fallback and timeout settings include explicit defaults and escalation path.
