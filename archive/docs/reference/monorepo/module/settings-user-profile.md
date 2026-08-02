# Setup & Config: User Profile - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to modifying a single user's UI preferences, localization, and personal notification opt-ins. It strictly does NOT handle global application rules.

## 2. Input Data Required
- **Preferences:** UI Theme (Dark/Light), Timezone, Language.
- **Opt-ins:** "Mute non-critical alerts between 10PM-6AM" toggles.

## 3. Registry Sub-Component Integration
This screen is entirely UI-centric and generally bypasses registry logic:
- **Agents:** None.
- **Skills:** None.
- **Workflows:** None.
- **Protocols:** None.
- **KPIs:** None.
- **Data Inputs:** None.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Not used.
- **Goose:** Not used.
- **NemoClaw:** Anchors the UI toggle states and persists them locally.

## 5. Hybrid DB & State Storage Flow
- **State Trajectory:** User preferences are fundamentally ephemeral to the device/user session. They are saved directly into the **SQLite DB** `user_preferences` table.
- **Vault Exclusion:** These settings are NEVER committed to the Vault or Git, representing purely local configuration state.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** None.
