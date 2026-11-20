---
name: ui-design
description: Project rules for the look of every screen of {{PROJECT}}: site pages, dashboard, mockups, design-tool boards and their copy. Use before choosing fonts, colours or a layout, before writing page copy, and when reviewing a screen, a prototype or a screenshot.
---

# How the screens of {{PROJECT}} look

Catalogues of AI-built sites list a small set of defaults that every generator reaches for. A
screen made of those defaults reads as generic, and the visitor stops trusting it. The rules below
give each screen its own look; each one ends with a **check** that can be answered by looking at
the design file, the CSS or a screenshot.

## When you create

1. Read the rules before you pick a palette, a type pair or a page structure.
2. Choose the base for this product: what it sells, who reads it, the real data it shows.
3. Before showing the work, run every check and fix what fails.

## When you review

Go through the checks one by one. For each rule report **pass** or **fail** with the evidence (the
CSS line, the board, the screenshot area). A fail is fixed, or kept with its reason written in the
project's `DESIGN.md`.

## Rules

1. **Own base.** Palette and type pair belong to this product, and every title has a single style.
   *Check:* the base is not the current AI default: cream paper, a serif display face with one
   italic or coloured word, a rust accent and sage green.
2. **Flat colour.** Surfaces are solid colours from the tokens. *Check:* the CSS and the design file
   have no gradients (purple or blue above all), glass or blur, glows, coloured shadows, blobs or
   3D illustrations.
3. **Opening that starts work.** The first screen says what the product does, aligned to the
   reading edge, next to the product itself. *Check:* the hero is not centred and has no pill or
   badge above the title.
4. **Features shown working.** Each feature appears as the product doing it, with its data.
   *Check:* no row of three identical cards with icon, title and line; no bento grid with one
   feature per tile.
5. **Lines before boxes.** Links, navigation and lists sit on the page background, separated by
   hairlines; a card holds one view with data. *Check:* no box contains only text or links.
6. **Links that say where they go.** The link text carries the meaning. *Check:* no arrow glyph
   (↗, →) repeated on every link or button.
7. **Real numbers.** Figures come from real data; sample data carries the label "illustrative
   data". *Check:* no round or made-up statistics.
8. **Concrete copy.** Titles name the thing or the result. *Check:* no empty slogans, triads,
   "not just X but Y" or buzzwords.
9. **Readable text.** *Check:* no text under 12 px, and at most a third of the text at 12.5 px or
   less. A dense interface that needs smaller labels keeps them only with a written exception in
   `DESIGN.md`.
10. **Monospace for code.** A monospace face appears only for real code and commands. *Check:*
    labels, numbers and decoration use the text face.
11. **Order from the reader's questions.** Sections follow what this reader needs to decide.
    *Check:* the page is not hero → logo strip → three features → testimonials → pricing → call to
    action.
12. **The real product.** Screens of the product, with real or labelled illustrative data, carry
    the page. *Check:* every product image is a real screen, not an abstract illustration.
13. **Logo test.** Swap the logo for a competitor's. *Check:* the page no longer works, because the
    copy, data and screens are this product's.
14. **Motion that explains.** Movement shows a change of state and then stops. *Check:* no
    continuous animation.

## Sources

Researched on 2026-11-20; read them again when the rules feel dated.

- https://www.utsubo.com/blog/built-with-ai-trust-signals-2026
- https://raw.githubusercontent.com/referodesign/refero_skill/f78b4eccf112d7a179b92afeafdd7e8684560ac2/references/anti-ai-slop.md
- https://github.com/Laith0003/ux-skill/wiki/Anti-AI-slop-ban-list
- https://docs.rs/crate/aiseo/0.6.2/source/src/audit/design_slop.rs
- https://cdn.jsdelivr.net/npm/pixelslop@0.3.8/dist/skill/resources/ai-slop-patterns.md
