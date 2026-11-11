# Quick Panel visual prototype

Throwaway prototype for [制作 Quick Panel 白系视觉与动效原型](https://github.com/NibbleOrg/Nibble-desktop/issues/9). It is decision material, not production code.

Run it with one command:

```sh
python3 -m http.server 4173 --directory prototypes/quick-panel-visual
```

Open <http://localhost:4173/?variant=A>.

Switch structures with `?variant=A|B|C|D` or the floating bar:

- `A` — Linen ledger: compact search, ledger-like rows, quiet tool chrome.
- `B` — Ribbon desk: editorial search, split tool cards, warm paper and a side note rail.
- `C` — Toy precision: graph paper, physical key rows, dark rules, and a firmer snap.
- `D` — Frosted pane: translucent warm-white layers over a deliberately busy simulated source app, with heavy background blur and a brighter selected row. The colored backdrop demonstrates transparency; it is not part of the proposed Quick Panel design.

The **Prototype lab** opens every non-happy path, the shortcut-reference layer, and the 150/200/250ms motion rhythm. `motion=off` demonstrates the reduced-motion alternative.
