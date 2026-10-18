# Craft defaults

Read when writing tokens, building the core set, or adding motion. Values are starting points; the owner tunes them, and the tuned values live in the tokens.

## Tokens

- **Color**: role pairs, each fill with its foreground. Example names: `ground`, `surface`, `raised`, `ink`, `muted`, `line`, `accent` and `on-accent`, `danger` and `on-danger`, `focus`. Each color does one job, written in design.md.
- **Type**: roles `display`, `body`, `mono`, each with family, a small size scale, weights and line heights.
- **Space**: one scale, used for padding and gaps. A parent owns the gap between its children.
- **Radius**: tiers keyed to hierarchy, not one value everywhere.
- **Shadow**: tiers keyed to elevation.
- **Motion**: durations, easings, the press scale.
- **Control height**: shared by button, input and select, so they change together.
- **Focus ring**: width, offset and color, used by every control.

## Color

- OKLCH ramps: one hue variable per ramp, scale steps of even perceived lightness, chroma tapered toward both ends. Neutrals carry a little of the hue.
- Contrast is measured, not eyeballed, to WCAG AA: normal text 4.5:1; large text (24 CSS px, or 18.67px bold) 3:1; 3:1 for a control's boundary where it identifies the control or its state, and for the focus indicator.
- Color is never the only signal of a state.

## Theme

- Light and dark are each designed; neither is the other inverted.
- `color-scheme: light dark` on `:root` follows the system; under a stored choice it is `light` or `dark`. `light-dark()` resolves from it but cannot select whole shadow values, so shadow tokens switch through the theme selector or a `prefers-color-scheme` media query.
- A stored choice is applied by an inline script in `<head>` that sets the theme attribute before first paint. Paint `html` with the `ground` token and keep the `theme-color` meta in step.
- A state that starts folded or collapsed renders that way on first paint, with no animation.

## Depth

Containers, the things that hold other things, get a layered shadow instead of a border. In light mode no layer goes above about 6%; a shadow you consciously see is too strong. Higher elevations add layers and keep the first two. Light:

```css
--shadow-surface:
  0 0 0 1px oklch(0 0 0 / 0.06), 0 1px 2px -1px oklch(0 0 0 / 0.06), 0 2px 4px oklch(0 0 0 / 0.04);
```

In dark mode the edge comes from light: a faint inset ring and top highlight, with outer shadows only for separation.

```css
--shadow-surface:
  inset 0 1px 0 oklch(1 0 0 / 0.03), inset 0 0 0 1px oklch(1 0 0 / 0.03),
  0 0 0 1px oklch(0 0 0 / 0.1), 0 2px 2px oklch(0 0 0 / 0.1), 0 4px 4px oklch(0 0 0 / 0.1);
```

## Shape

Nested corners are concentric: outer radius = inner radius + inset, counting the parent's border. Clamp the inner radius at 0.

## Type

- Mono sets numbers and labels apart as data, identifiers and code; `font-variant-numeric: tabular-nums` in the body face aligns numbers that change or sit in columns. Right-align numeric columns, and check the face has tabular figures.
- `text-wrap: balance` on headings and short blocks; `text-wrap: pretty` on paragraphs and list items. Set once in base styles.
- Tracking in `em`: tighter as size grows, body left alone.
- Prose under about 75 characters per line. Inputs at least 16px on phones.

## Controls

- Semantic elements, and a visible label on every control.
- Hit areas: 24px minimum, 44px on touch. Extend a small visual with a pseudo-element; adjacent rows touch, with padding inside rather than margin between.
- Focus: the ring token on `:focus-visible`. Never remove an outline without the ring replacing it.
- States: default, hover, focus-visible, active, disabled. A loading button keeps its label and adds a spinner. An invalid field sets `aria-invalid`, shows its message beside the control, and submit moves focus to the first error.
- Press: `--press-scale: 0.97` on `:active`, over 100 to 150ms, `transform` only. Larger surfaces shrink less. Off when disabled and under reduced motion.

## Motion

Tokens: two or three durations (about 120ms for presses and small changes, 200ms for popovers and panels, 300ms for larger moves) and two or three named easings, reused everywhere. Ease-out, such as `cubic-bezier(0.23, 1, 0.32, 1)`, for anything that answers the person.

The ladder, cheapest first; stop at the first step that reaches the effect:

1. CSS transitions on the tokens, each property named; never `transition: all`.
2. `@starting-style`, or the primitive's starting-style attribute, for elements entering.
3. Keyframes for loops and one-shots nothing interrupts.
4. View Transitions for a DOM swap on a route or state change: feature-detect, fall back to an instant change, name only what is in view.
5. The Web Animations API for programmatic one-offs.
6. A library, only for an effect the steps above cannot reach.

Rules:

- The more often something happens, the less motion it gets. Hover gets no movement, a short color change at most; tab switches and keyboard-triggered changes are instant; rare moments can carry more.
- Transitions retarget when interrupted. Never gate input on an animation.
- Content and input land at once; motion dresses the change. A repeat retargets or skips.
- Pin actions, fix sticky heights, anchor the scroll, and animate the smallest part, so nothing around the motion moves.
- Entrances start at a scale of 0.9 to 0.98 with opacity, never from scale 0. Popovers grow from their trigger. Exits are shorter than entrances.
- One ambient animation at a time; continuous loops stop at rest. Flashing stays within WCAG's limits.
