# Platform Compatibility

This document outlines the compatibility of the Astra library with various React environments, specifically **React Web**, **Electron**, and **React Native**.

## Summary Table

| Platform | Status | Notes |
| :--- | :--- | :--- |
| **React Web** | ✅ Fully Supported | Native environment for this library. |
| **Electron** | ✅ Fully Supported | Electron uses a Chromium renderer, so all web technologies work out of the box. |
| **React Native** | ❌ Not Compatible | Dependencies on `react-dom`, Material UI, and browser APIs (`localStorage`) prevent usage. |

---

## Detailed Analysis

### 1. Electron Support
**Verdict: Compatible**

Electron applications essentially run a web page inside a Chromium window. Since Astra is built for standard React Web (using `react-dom`), it functions perfectly within an Electron `BrowserWindow`.

*   **DOM Access**: Electron has a full DOM implementation, supporting the Material UI components that Astra uses.
*   **Storage**: The `localStorage` API used in Astra's `ThemeProvider` works natively in Electron's renderer process to persist user preferences (like Dark Mode).
*   **Networking**: The `ApiService` (Axios) works seamlessly in Electron.

**Integration Tip**: If you are strict about security (e.g., `contextIsolation: true`), this library still works because it operates entirely within the "Renderer" process, just like a normal website.

### 2. React Native Support
**Verdict: Incompatible**

Astra cannot be used directly in a React Native project in its current state.

#### Reasons:
1.  **DOM Dependencies**:
    *   Astra relies on `@mui/material` (Material UI), which renders HTML elements like `<div>`, `<span>`, and `<button>`. React Native requires platform-specific primitives like `<View>`, `<Text>`, and `<TouchableOpacity>`.
    *   `react-dom` is a peer dependency, which does not exist in React Native.

2.  **Browser APIs**:
    *   **LocalStorage**: Astra's `ThemeProvider` directly calls `window.localStorage` to save theme settings. React Native does not have a global `window` object or `localStorage`; it requires `AsyncStorage` or similar libraries.

3.  **Styling Engine**:
    *   MUI uses Emotion/CSS-in-JS that generates CSS classes. React Native uses a style object system compliant with Yoga Layout, not CSS classes.

#### Migration Path (If needed)
To make this library compatible with React Native, you would need to:
1.  **Abstraction**: Refactor `ThemeProvider` to accept a generic "Persistence Adapter" (so you can swap `localStorage` for `AsyncStorage`).
2.  **UI Separation**: Remove `@mui/material` and `react-dom` dependencies. The logic hooks (`useDataState`, `LanguageProvider`) are logic-only and *could* potentially be reused if stripped of DOM references, but the UI components would need to be rewritten.
