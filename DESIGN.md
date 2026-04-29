# Design System — CAD Open Layer

The visual system for cadopenlayer.dev (marketing landing) and the WASM
Playground (browser demo). One coherent language across both surfaces.

Memorable thing: **"adults made this"**. Quiet authority before scroll.
Trust earned through typographic discipline, not visual noise.

Preview sheet: `~/.gstack/projects/cad-open-layer/designs/design-system-20260429/preview.html`

---

## Product Context

- **What this is:** open-source Rust + WASM library for programmatic
  DWG/DXF access, Apache-2.0 + future managed commercial API
- **Who it's for:** AI design startup founders, BIM/architecture SaaS
  teams, SI enterprise developers
- **Space/industry:** developer infrastructure × CAD interop. Closest
  category neighbours are Mapbox / Stripe / Linear (dev SaaS), with the
  CAD inheritance differentiating us
- **Project type:** marketing landing site + browser-based interactive
  Playground demo (one design language for both)

---

## Aesthetic Direction

- **Direction:** technical-document brutalism (drafting paper inheritance)
- **Decoration level:** minimal — structure replaces ornament. Visible 1px
  rules, numbered section markers, dimension-line callouts, drafting
  title-block frames are the decoration
- **Mood:** the cool, deliberate authority of a 1970s German engineering
  manual, retooled with the precision of a modern build system log
- **Reference inheritance:** Dieter Rams Braun manuals, Stripe API
  reference, Standardgraph/Linex architectural drafting templates,
  ISO 7200 title block conventions

---

## Typography

All three faces are FREE. No license fees, no enterprise tier.

- **Display/Hero:** **Cabinet Grotesk** (Indian Type Foundry, via Fontshare)
  — wide, mechanical confidence at 96px+. Distinctive lowercase character
  avoids the Söhne/Inter/Space Grotesk convergence
- **Body:** **Switzer** (Indian Type Foundry, via Fontshare) — Söhne-adjacent
  Swiss neutrality with warmth. Reads beautifully at 16/26 and 19/30
- **UI/Labels:** Switzer (same as body)
- **Data/Tables:** Geist Mono (tabular-nums on, ss02 enabled)
- **Code:** **Geist Mono** (Vercel, via Google Fonts) — technical authority,
  matches body weight
- **Loading:**
  ```html
  <link rel="preconnect" href="https://api.fontshare.com">
  <link href="https://api.fontshare.com/v2/css?f[]=cabinet-grotesk@700,800&f[]=switzer@400,500&display=swap" rel="stylesheet">
  <link href="https://fonts.googleapis.com/css2?family=Geist+Mono:wght@400;500&display=swap" rel="stylesheet">
  ```

### Type scale

| Role | Size | Line height | Weight | Letter spacing | Font |
|------|------|-------------|--------|----------------|------|
| display-1 (hero) | clamp(48px, 7.5vw, 96px) | 0.95 | 800 | -0.02em | Cabinet Grotesk |
| display-2 (section) | clamp(36px, 5vw, 64px) | 1.05 | 700 | -0.015em | Cabinet Grotesk |
| h1 | 36px | 1.15 | 600 | -0.01em | Cabinet Grotesk |
| h2 | 24px | 1.20 | 600 | -0.005em | Cabinet Grotesk |
| body-lg | 19px | 1.55 | 400 | 0 | Switzer |
| body | 16px | 1.60 | 400 | 0 | Switzer |
| caption | 13px | 1.40 | 500 | 0.02em | Switzer |
| section-no | 12px | 1.40 | 500 | 0.08em uppercase | Geist Mono |
| mono | 14px | 1.60 | 400 | 0 | Geist Mono |

---

## Color

- **Approach:** restrained — drafting paper background, ink for text,
  prussian blue as the single primary accent, minium red used ONLY for
  errors and dimension marks at <2% of pixels

| Token | Hex | Role |
|-------|-----|------|
| `--ink` | `#0E0E0C` | text · primary surface (dark mode) |
| `--paper` | `#F4F1EA` | primary surface (light mode) |
| `--prussian` | `#1B3A4B` | primary accent · CAD-native |
| `--minium` | `#C8412A` | errors · dim marks · used at <2% pixels |
| `--graphite` | `#6B6B66` | secondary text · rules |
| `--patina` | `#9FB8A8` | data viz · success states |
| `--onionskin` | `#E8DFC8` | hover · raised panels |
| `--surface` | `#FAF7F0` | code blocks · raised cards |
| `--rule` | `rgba(14,14,12,0.18)` | hairline divider |
| `--rule-strong` | `rgba(14,14,12,0.55)` | section divider |

### Dark mode (ink theme)

When the user toggles to ink mode, swap surfaces and reduce accent
saturation 10-20% so the prussian blue does not vibrate on near-black:

| Token | Light (paper) | Dark (ink) |
|-------|---------------|------------|
| `--bg` | `#F4F1EA` | `#0E0E0C` |
| `--fg` | `#0E0E0C` | `#F4F1EA` |
| `--accent` | `#1B3A4B` | `#6FA8C0` |
| `--warn` | `#C8412A` | `#E36F58` |
| `--surface` | `#FAF7F0` | `#141412` |
| `--onionskin` | `#E8DFC8` | `#1A1A18` |

Light is the default. Provide a `[ INK ↔ PAPER ]` toggle in the page
header (mono-styled button, no icon).

---

## Spacing

- **Base unit:** 8px
- **Density:** comfortable (not compact, not spacious — drafting-page
  appropriate)
- **Scale:** `2xs(2) xs(4) sm(8) md(16) lg(24) xl(32) 2xl(48) 3xl(64) 4xl(96)`
- **Section vertical rhythm:** 80px top padding + 64px bottom padding,
  separated by `--rule-strong`

---

## Layout

- **Approach:** strict 12-column grid with editorial overrides for hero
  and Playground
- **Grid:** 12 columns, 24px gutter, 32px page padding (16px on mobile)
- **Max content width:** `1280px`
- **Border radius:** mostly 0. Use radius only for `border-radius: 0`
  drafting frames OR small radius (4px) on form inputs and code blocks.
  No bubble pill UI.
- **Section markers:** every section gets a `§NN / SECTION-LABEL` marker
  in Geist Mono caps at 12px, color `--graphite`
- **Visible 1px rules:** between sections (`--rule-strong`), inside
  capability tables (`--rule`), inside drafting title blocks
- **Title blocks:** the Playground UI sits inside a 1.5px ink border with
  a 4-cell metadata header (SHEET / SCALE / REV / DATE) and a footer
  showing file name + compile status. Looks like an actual drawing sheet.

---

## Hero composition (the deliberate departure)

NO illustration. NO 3D render. NO product screenshot. NO icon grid.

The above-the-fold viewport is split:

- **Left (1.1fr):** typeset prose
  - 4-line display headline ("Programmable / DWG/DXF for / the people who / build software.")
  - 2-line body paragraph stating capability + license
  - Two buttons (`Read the docs →` primary, `Open Playground ↗` secondary)
  - Metadata line at the bottom: RUNTIME / LICENSE / STATUS in 3 columns,
    separated by hairline above

- **Right (1.0fr):** drafting title-block code frame
  - 4-cell header: SHEET 1/1 · SCALE 1:1 · REV 0.1.0 · DATE
  - Body: 8 lines of syntax-highlighted Rust (parse_dxf_file usage)
  - Footer: file path + `● compiled · 0.42s` indicator

**Why:** category convention says "show the product." Our category neighbors
(Mapbox, Stripe, Linear, Vercel) all do. We say: the product IS text
(library output, code, data). Respect it. The hero earns trust by being
the first sample of the documentation, not a marketing layer over it.

---

## Components

### Buttons

| Variant | Background | Border | Text | Hover |
|---------|-----------|--------|------|-------|
| Primary | `--ink` | `--ink` | `--paper` | `--prussian` bg |
| Secondary | transparent | `--ink` 1.5px | `--ink` | inverts |
| Ghost | transparent | `--rule-strong` 1px | `--fg` | `--hover` bg |

Padding `12px 22px`, font 14px Switzer 500. Arrow uses Geist Mono 13px.

### Form fields

Border `1px --rule-strong`. Background `--bg`. Focus border `--accent`.
Labels in Geist Mono 11px caps `--muted`. No border-radius.

### Alerts

Left-border 3px in semantic color. Background `--surface`. Icon in
`[ LABEL ]` mono-bracket style, not iconography. Four variants: info
(prussian), success (patina), warn (#C8923A — sand), error (minium).

### Code blocks

Background `--surface`. Border-radius 4px. Padding 24px. Font Geist Mono
13.5px / 1.6. Token colors:
- keyword: `--prussian`
- function name: `--ink`
- string: `--minium`
- number: `--prussian`
- comment: `--muted` italic

### Capability table (numbered, dimension-line)

```
001    Title           Description with inline metric         module-name
002    Title           Description with inline metric         module-name
003    Title           Description with inline metric         module-name
```

Grid `80px 200px 1fr auto`, 24px gap, 18px vertical padding, top/bottom
strong rule, internal hairlines. Number in Geist Mono `--muted`. Title
in Cabinet Grotesk 22/600. Description in body Switzer.

---

## Motion

- **Approach:** minimal-functional. NO scroll-driven animation. NO entrance
  fades. NO parallax.
- **Easing:** enter `ease-out`, exit `ease-in`, transitions `ease-in-out`
- **Duration:** micro (50-100ms) for hovers, short (150-250ms) for theme
  switches, medium (250-400ms) for nothing on a landing page
- **One allowed flourish:** code blocks may show a subtle blinking cursor
  at `compiled · 0.42s` to suggest live execution. Even this should be
  soft (12px line, opacity 0.5)

---

## Voice for landing copy

- Sentence-cased, not Title Cased
- Short paragraphs. Period at the end. No marketing exclamations.
- Concrete numbers in body copy ("23k entities / 178ms"), not adjectives
  ("blazingly fast")
- Code samples are the documentation, not decoration
- Tone reference: Stripe API ref + Linear changelog + a 1970s technical
  manual

---

## What NOT to do (anti-slop guardrails)

- No purple/violet gradient anywhere
- No 3-column feature grid with icons in colored circles
- No centered-everything layout
- No bubble border-radius on cards or buttons
- No gradient CTA buttons
- No Inter / Roboto / Arial / Helvetica / Space Grotesk as body or display
  (we have Switzer + Cabinet Grotesk for a reason)
- No hero stock photo or 3D render
- No "Built for X" / "Designed for Y" marketing patterns
- No dark mode by default (we differentiate with light); dark is a toggle
- No system-ui as primary font (the "I gave up on typography" signal)

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-04-29 | Initial design system created via /design-consultation | Drafting-paper brutalism direction. Three free typefaces (Cabinet Grotesk + Switzer + Geist Mono). Light-mode default with dark toggle. Numbered section markers + visible 1px rules + drafting title-block hero. Memorable thing: "adults made this". Independent design subagent's drafting-paper proposal preferred over Claude main's dark-mode split-canvas; subagent's font choices substituted with free equivalents. |
