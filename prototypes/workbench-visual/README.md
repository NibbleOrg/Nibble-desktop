# Workbench dark-shell prototype

Throwaway prototype for [制作 Workbench 深色外壳与受限月历布局原型](https://github.com/NibbleOrg/Nibble-desktop/issues/14). It is decision material, not production code.

Run it with one command:

```sh
python3 -m http.server 4174 --directory prototypes/workbench-visual
```

Open <http://localhost:4174/?variant=A>.

Switch structures with `?variant=A|B|C` or the floating bar:

- `A` — Ink rail: a very narrow icon rail and a restrained, centered Calendar sheet.
- `B` — Editorial ledger: a wider text navigation rail, a more expansive Calendar surface, and a stronger month heading.
- `C` — Command shelf: a compact top command bar with a smaller floating Calendar card.

The Prototype lab exposes:

- `150`, `200`, and `250ms` motion rhythms.
- The reduced-motion alternative (`?motion=reduce`).
- The empty-month explanation state.
- The accepted white Quick Panel positioned over each dark Workbench variant.

The month grid uses the browser locale for month/weekday labels and week start. Previous/next month and Return to Today use real local dates.
