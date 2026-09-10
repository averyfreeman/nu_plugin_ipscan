# rustdoc-ipscan documentation roadmap

## Current direction

- Keep Astro + Starlight and the generated Rustdoc pipeline.
- Use the selected documentation-first Matcha-inspired direction as the homepage
  and visual reference.
- Keep the existing Starlight search, sidebar, theme controls, and accessible
  document structure.
- Keep the landing page as a dedicated Astro route at `src/pages/index.astro`,
  using Starlight's splash metadata without navigation rails so the overview
  remains centered at every width.
- Keep the generated documentation routes in Starlight. Documentation pages
  begin with equal-width navigation and table-of-contents rails collapsed, then
  expose corner controls for opening them as needed.
- Resolve all landing-page documentation CTAs through Astro's configured base
  URL. The primary hero action is the single Installation entry point; the
  lower `Get started` action points to that same route.
- Keep a project-owned 404 page and favicon so local development and deployed
  previews do not emit missing-content or missing-asset warnings.
- Treat `mockup_screenshot.png` as a visual reference, not as a fixed
  viewport constraint. The site remains responsive.

## Visual system

- Dark green-black surfaces, thin rules, quiet panels, and restrained depth.
- Use a bright pastel orange accent instead of blue:
  - accent: `#f2a65a`
  - high/text accent: `#ffd9a3`
- Keep secondary emphasis warm and clear so the palette stays orange rather
  than drifting into red, pink, or magenta.
- When a heading gradient is useful, it runs from pale orange toward amber.
- Use IBM Plex-style mono/sans typography with italic serif emphasis and
  system fallbacks.

## Homepage structure

1. Compact hero with a dual-style headline.
2. Valid Nushell terminal demo with replay.
3. Output schema table.
4. Installation commands.
5. Practical examples.
6. Documentation navigation preview.
7. Final call to action and footer.

## Interaction rules

- Terminal playback is deterministic, auto-types once when visible, and exposes
  an explicit replay button.
- Homepage sections and feature blocks use one-shot IntersectionObserver reveals.
- Reduced-motion users receive the complete content immediately with no movement
  or typewriter delay.
- Inner documentation pages do not animate every paragraph.

## Validated Nushell examples

Validated with Nushell `0.115.1` and the loaded `ipscan` plugin:

```nu
ipscan --list
ipscan --list | where is_up and not is_loopback
ipscan --network 192.168.1.0/24 --numeric
ipscan --network 192.168.1.0/24 --numeric | where vendor != ''
ipscan --network 192.168.1.0/24 --numeric | to json | save hosts.json
```

The plugin requests JSON from the standalone scanner and converts its interface
and scan records into native Nushell values.

## Implementation order

1. Add the shared theme stylesheet and Starlight configuration hook.
2. Add reusable terminal playback and viewport-reveal components.
3. Replace the homepage content with the selected structure.
4. Add the repeatable Nushell example validator.
5. Build the site and inspect desktop/mobile rendering.
6. Verify keyboard focus, copy controls, search, sidebar navigation, replay,
   reduced motion, and generated Rustdoc links.

## Reuse notes

Keep theme tokens, section classes, reveal behavior, and the terminal component
generic enough to copy into other Astro/Starlight repositories. Keep product
content and command examples in the page/component data rather than coupling
the reusable visual layer to this plugin.
