# Theming & UI

Astra leverages Material UI (MUI) for its component library and provides a robust `ThemeProvider` wrapper to handle dynamic theme switching (Light/Dark mode) and persistence.

## Architecture

### 1. ThemeProvider (`src/common/theme/ThemeProvider.tsx`)

A custom wrapper around MUI's native `ThemeProvider`. It adds logic for:
-   **State**: Managing `darkMode` boolean.
-   **Persistence**: reading/writing `darkMode` preference to `localStorage`.
-   **Storybook Support**: Accepts a `forceTheme` prop to lock the theme for UI testing.

#### Props
```typescript
type ThemeProviderProps = {
  lightTheme: Theme;  // MUI Theme object
  darkTheme: Theme;   // MUI Theme object
  forceTheme?: 'light' | 'dark'; // Optional overrides
};
```

### 2. Theme Context (`src/common/theme/ThemeContext.ts`)

#### `useTheme()` Hook
This is **not** the MUI standard hook (which is `useTheme` from `@mui/material`). This is Astra's custom hook to access the *toggle functionality*.

```typescript
const { 
  darkMode,       // boolean
  toggleDarkMode  // function() => void
} = useTheme();
```

> **Note**: To access theme variables (colors, spacing), continue using MUI's `useTheme` or `makeStyles`. Use Astra's `useTheme` only for switching modes.

---

## Implementation Details

### Persistence
The provider automatically checks `localStorage` on initialization:
```typescript
const [internalDarkMode, setInternalDarkMode] = useState(() => {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('darkMode') === 'true';
  }
  return false;
});
```
This ensures the user's preference is remembered across sessions.

### Storybook Integration
The `forceTheme` prop allows you to create stories that specifically render in Dark Mode without manual interaction.
```tsx
// MyComponent.stories.tsx
export const DarkModeView = () => (
    <ThemeProvider lightTheme={l} darkTheme={d} forceTheme="dark">
        <MyComponent />
    </ThemeProvider>
);
```

## Setup Example

```tsx
import { createTheme } from '@mui/material/styles';
import { ThemeProvider } from 'astra';

const light = createTheme({ palette: { mode: 'light', primary: { main: '#007bff' } } });
const dark = createTheme({ palette: { mode: 'dark', primary: { main: '#90caf9' } } });

export const App = () => (
  <ThemeProvider lightTheme={light} darkTheme={dark}>
    <MainLayout />
  </ThemeProvider>
);
```
