# Interface: PDF Viewer - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the safe binary decoding, iframe isolation, and UI rendering of `.pdf` documents within the system.

## 2. Input Data Required
- **File Locator:** The explicit `vault://` path or memory Buffer ID.
- **Access Context:** User intent to read.

## 3. Registry Sub-Component Integration
The viewer is purely a frontend read capability, constrained by:
- **Agents:** None.
- **Skills:** None.
- **Workflows:** None.
- **Protocols:** Must respect Data Classification protocols (e.g. not caching high-security Vault elements onto the OS temp folder for rendering).
- **KPIs:** None.
- **Data Inputs:** Raw binary arrays.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Execution gateway verifies the user's role has permission to access the explicit Vault path.
- **Goose:** Not used in this context.
- **NemoClaw:** The primary Canvas/iFrame anchor. It specifically binds the zoom, pan, and scroll logic.

## 5. Hybrid DB & State Storage Flow
- **Data Source:** Pulls immutable blobs directly from the **Vault**.
- **State Flow:** Renders securely into ephemeral application RAM. Never saves derived data into SQLite.
- **Caching:** If the file is >20MB, the chunked segments are held purely in memory space.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** None.
