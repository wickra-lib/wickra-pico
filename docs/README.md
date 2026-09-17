# Documentation

The reference documentation for Wickra Pico lives at
**[pico.wickra.org](https://pico.wickra.org)** — the demo, the wiring, the
flashing steps and the determinism argument.

What stays here, beside the code, is the material that only makes sense next to
the implementation and has to change in the same commit as it:

- [`DETERMINISM.md`](DETERMINISM.md)
- [`FLASHING.md`](FLASHING.md)
- [`SIGNAL.md`](SIGNAL.md)
- [`VIDEO_SCRIPT.md`](VIDEO_SCRIPT.md)
- [`WIRING.md`](WIRING.md)

## Editing the docs

The documentation site is a separate git repository at
`https://github.com/wickra-lib/wickra-pico-site`. Open a pull request there to
propose changes; the site is built with VitePress and deploys to
`pico.wickra.org`. The files in this directory change in the same commit as
the code they describe.
