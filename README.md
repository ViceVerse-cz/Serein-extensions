# Serein extensions

Theme and plugin packages for the native [Serein client](https://github.com/ViceVerse-cz/Serein).
`catalog.json` contains manifest metadata, exact byte lengths, SHA-256 hashes, and immutable package URLs pinned to a full Git commit.
Clients download package data only. They never clone this repository or run its build scripts. Plugin capabilities and updates require the client's explicit consent flow.

## Maintenance

1. Edit themes under `themes/` or plugin sources under `plugins/`. Preserve `manifest.id` and bump `manifest.version` when releasing changes.
2. For plugins, follow [the SDK/build instructions](plugins/README.md) and regenerate the package under `plugins/packages/`. Keep the source manifest and packaged manifest identical.
3. Commit package bytes, source, and any preview images first; obtain the full commit SHA with `git rev-parse HEAD`.
4. Run `python catalog.py --commit <full-package-commit-sha>`, then `python test_catalog.py` and `python catalog.py --check`.
5. Commit `catalog.json`, then push both commits together.

Catalog tooling uses Python's standard library and checks identities, capabilities/actions, byte bounds, plugin Wasm headers, source manifests, hashes, and pinned Git bytes.
It intentionally does not duplicate Serein's complete theme schema or Wasm sandbox validation. The client performs canonical `extensions::parse_package` validation and sandbox checks before installation.
For a new package version, validate it with that parser and review its source and build before publishing.

PNG previews in `previews/` share the package filename stem and must fit 256 KiB. All current packages have previews: Ocean retains its original screenshot, Forest Piano and Soft White use downscaled package covers, and the other themes/plugins use rendered previews of the client's synthetic workspace. Full-size covers remain embedded in their packages.

## Imported plugin build evidence

Both plugins build from the included locked workspace with Rust 1.98.1 for `wasm32-unknown-unknown`.
Emoji & Sticker Images reproduces the imported 70,629-byte Wasm exactly.
The imported Message delete protector contains 115,250 bytes of Wasm; the current source/SDK builds 70,629 bytes and does not reproduce that older artifact byte-for-byte.
The existing distributed package is intentionally preserved. A future release should review and version a rebuilt artifact together with its source; this import is not a claim of reproducibility for that older package.

## Provenance and licenses

Package bytes and stable IDs are preserved from the existing client/theme repository:

- Nine original themes, two plugin packages, their Rust source/SDK/build files, and Ocean preview originate from `ViceVerse-cz/Serein` at `9e6bbcdef06b997ee3de3e228b2f2a54c940b7d0`.
- Forest Piano and Soft White originate from the owner's desktop packages normalized for Serein import at `a217f86c56b13ee51c4eb956f235b13eafb7609f`.
- All eleven theme packages are also recorded in `ViceVerse-cz/Serein-themes` at `6d2a97be4f8b0040aa9eada5648bceff55acb612`.
- Existing authors, license labels, and source URLs remain in every manifest. Serein's MIT and Apache-2.0 notices are retained at the repository root and do not replace individual artwork terms.
- Forest Piano and Soft White retain supplied `CC0-1.0` metadata; this is not independent verification of image rights. Consult their original sources for artwork provenance.

This repository does not relicense third-party artwork or assert additional redistribution rights.
