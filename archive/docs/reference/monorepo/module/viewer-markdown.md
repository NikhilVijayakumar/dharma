# Interface: Markdown Viewer - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to parsing, structuring, and visually rendering Markdown strings (such as Agent reasoning traces, Daily Briefs, and JSON abstractions).

## 2. Input Data Required
- **Raw Text:** Raw `.md` strings.

## 3. Registry Sub-Component Integration
This is the delivery mechanism for Goose logic:
- **Agents:** Renders their outputs.
- **Skills:** None.
- **Workflows:** None.
- **Protocols:** None.
- **KPIs:** None.
- **Data Inputs:** Renders them visually.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Not applicable.
- **Goose:** Goose provides the precisely structured Markdown strings (e.g., table layouts, bolding). This module *consumes* those strings.
- **NemoClaw:** The DOM rendering abstraction taking raw markdown hooks and converting them into strict CSS-scoped HTML components.

## 5. Hybrid DB & State Storage Flow
- **Data Source:** Renders dynamically from either **SQLite** (for ephemeral unapproved reports) or **Vault** (for historical data).

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** None.
