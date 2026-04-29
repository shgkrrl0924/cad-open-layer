# web/

Static landing page (`index.html`) and WASM Playground (`playground.html`)
for cadopenlayer.dev. Pure HTML + CSS + ES modules — no build step.

## Stack

- Vanilla HTML / CSS / JS (ES modules)
- `pkg/` is the wasm-pack output (gitignored, regenerated)
- Cabinet Grotesk + Switzer (Fontshare) + Geist Mono (Google Fonts)
- Design system: `../DESIGN.md`

## Build the WASM bundle

```sh
wasm-pack build crates/cad-wasm --target web --out-dir ../../web/pkg --no-pack
```

Run from repo root. The Playground imports `./pkg/cad_wasm.js`.

## Local preview

```sh
# Any static file server with proper MIME for .wasm works.
# Python:
python -m http.server 8000 -d web

# Node (npx):
npx http-server web -p 8000 -c-1
```

Then open `http://localhost:8000`.

The Playground fetches `../tests/corpus/synthetic/*.dxf` for the sample
buttons, so serve from the repo root if you want samples to work:

```sh
python -m http.server 8000
# http://localhost:8000/web/
```

## Deploy

Static. Drop `web/` + `pkg/` (built fresh) on any of:
- GitHub Pages
- Cloudflare Pages
- Vercel (static)
- Netlify

Set MIME for `.wasm` to `application/wasm` if the host doesn't auto-detect.
