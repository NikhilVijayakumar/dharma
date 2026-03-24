---
id: command-decomposition-logic
title: Command Decomposition Logic
tags: [generated, agentic]
---
# Command Decomposition Logic

## Purpose
Defines specialized capability for Command Decomposition Logic workflows and decisions.

## Activation Triggers
- Trigger when tasks match the Command Decomposition Logic responsibility domain.
- Trigger for planning, diagnosis, optimization, or remediation requests tied to this capability.
- Trigger when cross-team handoffs need deterministic decision framing.

## Inputs
- Current objective, constraints, and operating context.
- Relevant KPI signals, telemetry, or evidence artifacts.
- Applicable protocols, policies, and dependency boundaries.

## Operating Workflow
1. Scope the problem and clarify success conditions.
2. Validate data quality and identify missing dependencies.
3. Build an execution strategy with prioritized steps and checkpoints.
4. Evaluate risk, compliance, and downstream impact before action.
5. Produce deterministic recommendations with explicit assumptions.
6. Reassess outcomes and iterate if confidence or quality gates fail.

## Output Contract
- Actionable summary with rationale and traceable evidence.
- Ordered execution plan with expected outcomes and fallback path.
- Decision log entries suitable for audit and post-mortem review.

## Quality Gates
- Recommendations are testable, measurable, and policy-aligned.
- Terminology and decisions remain consistent across handoffs.
- Escalation is invoked when uncertainty, risk, or ambiguity exceeds tolerance.

## Escalation Conditions
- Conflicting requirements or policy interpretation disputes.
- Insufficient data quality for reliable recommendation.
- High-impact actions that require human approval prior to execution.

## Feature Extraction Parameters (OpenCLAW Sync)
- stage_protocol_sequence: enforce stage_init -> stage_plan -> stage_execute -> stage_synthesis in decomposition outputs.
- execution_constraints: include hard constraints and approval flags as explicit decomposition checkpoints.
- delegation_controls: include max_depth and timeout_ms when decomposition requires subagent branches.
- context_budget_hints: annotate token-sensitive branches with compact_threshold guidance.
