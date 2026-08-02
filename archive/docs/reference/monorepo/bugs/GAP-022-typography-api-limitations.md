# Gap Analysis: Google Workspace APIs vs Advanced CSS Typography

This document outlines the limitations encountered when mapping a web-based CSS design system (HTML/CSS) to Google Workspace APIs (Google Docs and Google Slides), specifically concerning advanced typography and spacing.

## 1. Typography and Kerning Limitations

**Web/CSS Capability:**
Modern CSS allows fine-grained control over typography using properties like `letter-spacing` (kerning/tracking) and `word-spacing`. 
Example: `letter-spacing: -0.02em;` to tighten heading display fonts.

**Google Docs/Slides API Limitations:**
- **Google Docs API**: The `TextStyle` object allows setting `fontSize`, `weightedFontFamily`, `bold`, `italic`, `foregroundColor`, and `backgroundColor`. However, it **does not support** tracking or letter-spacing adjustments programmatically.
- **Google Slides API**: Similarly, the `TextStyle` object in Google Slides lacks any attribute equivalent to `letter-spacing`.

**Impact:** 
If the Director's Office visual identity mandates specific tracking for display typography (e.g., tighter H1s or looser all-caps subheadings), this cannot be enforced via the automated templating engine in Google Docs or Slides. Texts will fall back to the font's default tracking.

## 2. Line Height (Leading) Caveats

**Web/CSS Capability:**
CSS `line-height` can be set unitless (e.g., `1.5`), in pixels, or as percentages.

**Google Docs/Slides API Status:**
- **Google Docs API**: Supported via `ParagraphStyle.lineSpacing` (which uses a percentage representation of normal line height, usually between 100-200). 
- **Google Slides API**: Supported via `ParagraphStyle.lineSpacing` (also a percentage mapping).

**Impact:**
While line height is supported, translating rem/px values from CSS to the percentage-based system in Google APIs requires a normalization formula to ensure the visual rhythm remains accurate.

## 3. Margin and Padding Collisions

**Web/CSS Capability:**
CSS uses the box model, allowing precise `margin` and `padding` handling, margin collapsing, and distinct block/inline layout behaviors.

**Google Workspace APIs:**
- **Google Docs**: `ParagraphStyle` supports `spaceAbove`, `spaceBelow`, `indentFirstLine`, and `indentStart`. There is no direct representation of `padding` inside a paragraph unless it is within a table cell. Margin collapsing does not work organically; space above and below must be calculated explicitly by the Renderer.
- **Google Slides**: Relies on absolute positioning of elements (`PageElement`). White space is achieved not by CSS margins but by computing explicit X/Y coordinates for the `Transform` affine matrix.

## Mitigation Strategy

1. **Typographic Fallbacks**: Accept that Google Fonts will be rendered with default kerning in Workspace. We will select Google Fonts that inherently look good without manual tracking adjustments (e.g., *Inter*, *Roboto*, *Lora*).
2. **Deterministic Spacing Engine**: The `WorkspaceRenderer` must compute the absolute position logic for Slides manually. For Docs, it must calculate explicit `spaceAbove` / `spaceBelow` values translating CSS rem/px variables.
3. **Table-based Padding**: For elements that require visual "padding" (like callout boxes or styled blockquotes) in Google Docs, the Renderer will map these to single-cell tables instead of standard paragraphs.
