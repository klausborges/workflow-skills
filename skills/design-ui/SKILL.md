---
name: design-ui
description: A small design system, tokens and a core set of components, set before screens, with a direction drawn from the product. Use when starting or restyling a UI, choosing a look, or building a screen or visual component; not for data, routing or state work.
---

# Design UI

The job is a UI that is "not cookie cutter", built from one small system instead of controls invented as the work goes. Directions rarely survive the first round; tokens set before the first screen make each later round a token change.

Read [ai-default-looks.md](references/ai-default-looks.md) before showing directions and when reviewing a screen, and [craft-defaults.md](references/craft-defaults.md) when writing tokens, building the core set, or adding motion.

## Start

- **design.md, tokens and core set exist**: work inside them; design.md wins over the switchable defaults below.
- **design.md exists, its tokens or core set don't**: do steps 2 and 3 before any screen.
- **No design.md, no UI yet**: run steps 1 to 4.
- **No design.md, existing UI, a narrow request** (a fix, one small component): use the app's existing styles, add no new one-off values, and propose the extraction in the report instead of starting it.
- **No design.md, existing UI, a new screen, a redesign, or the owner asks**: extract what the app already does into a proposed design.md, tokens and core set on one rendered page. Extend the app's existing token file; never add a parallel set. The proposal states which defaults it adopts and which it turns off. After approval, steps 2 to 4 run as one sweep. If the owner wants a new look, step 1 comes first.

If the owner can't pick or approve now, stop after the directions or extraction page and report it. Don't build tokens on a guess.

## 1 Direction

Render 2 or 3 directions on one HTML page. Each shows the same representative screen plus the core set, light and dark, with realistic data: a long value, an empty state, an error state.

- Draw each from the product's subject: its materials, tools, documents and vernacular. Borrow how non-UI media organize information (print, signage, instruments, maps), not how they look.
- Make them differ in type, color logic, density, shape and surfaces, not just palette.
- Decide per project where the expression goes and how much.
- Run the check in ai-default-looks.md on each and put its one-line result on the page.

The owner picks or mixes. The page stays next to design.md as the visual reference, never code to port.

## 2 design.md and tokens

`design.md`, lowercase, holds the direction in a few lines, the rules, the don'ts, and the defaults the project turns off. It names tokens by role and never copies values.

Tokens live in code, in one place, light and dark both authored: color, type, space, radius, shadow, motion, control height, focus ring. Name them by role, not scale step; the names in craft-defaults.md are examples, and an app keeps its existing role names. The styling layer (Tailwind `@theme`, custom properties, scoped styles) is the project's call; it reads the tokens and doesn't redefine them.

## 3 The core set

Build button, input, field (label, control, description, error) and surface before any screen, with every state listed in craft-defaults.md. Every control uses the focus-ring and control-height tokens. Select, dialog, tabs and other controls join the same way when a screen first needs them.

Primitives are the project's choice, often headless: Base UI (React) or bits-ui (Svelte) first; React Aria or Ariakit also fit. The primitive, the library's part only, supplies behavior and accessibility; the tokens supply the look, styled once in the shared layer. The call site owns only placement; a different look is a variant on the component.

## 4 Screens

Screens only consume the system. A screen that needs something new extends the system first: a token, a variant, a component, or a rule in design.md. No literal values where a token exists, no `.toolbar button {}` selector, no repeated class string standing in for a missing component.

A look change that needs edits in more than one screen means a value sits outside the system; move it in.

## Defaults

On unless design.md turns them off; values are in craft-defaults.md.

- OKLCH ramps.
- Light and dark from day one, each designed, the theme resolved before first paint.
- Layered soft shadows instead of borders on containers; inputs, dividers and table rows keep borders.
- A press-scale token on pressables.
- Reduced motion drops translate, scale, zoom and parallax, and keeps short fades and color changes that show what changed.
- Type roles: display, body, and mono for numbers and labels. At most two families plus mono, chosen in the direction round. Stock faces such as Inter count as the generic look unless the owner picks them; a system stack is fine when it fits.

The rest of craft-defaults.md always applies and isn't switchable.

## Motion

On top of the ladder and rules in craft-defaults.md:

- A visible swap or jump is a defect: give the change a transition or an anchor.
- Motion meant to be seen must be noticeable; unnoticeable motion is as wrong as flashy motion. At most one signature moment per surface.
- For a taste call, give the owner replayable variants or a tuner on one page. The picks become tokens.

## Done means rendered

Check in a real browser: light and dark, phone (about 390px) and desktop (about 1280px), keyboard path with visible focus, reduced motion. Screenshots go to the owner with the report, not into git.

## Review

A reviewer checks that every value comes from a token, new patterns entered the system first, design.md and the defaults it kept hold, no screen falls into an unchosen AI-default look, and the rendered check was done.
