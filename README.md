# Powerflow

Powerflow monitors power consumption, battery condition, and charging activity on Apple Silicon Macs.

## Requirements

- Apple Silicon Mac (ARM64)
- macOS 27 or later
- For connected iPhone and iPad monitoring: unlock the device, connect it, and allow the trust prompt. Wi-Fi monitoring requires device pairing and Wi-Fi connectivity enabled in Finder.

## Features

- Live system, battery, display, and heatpipe power readings
- Menu bar power indicator and quick-access popover
- Charging history with charts and JSON export
- Daily battery health snapshots
- On-demand process energy activity
- Adapter details and estimated readings when direct power telemetry is unavailable
- English and Simplified Chinese interface options

Available measurements depend on the hardware and telemetry exposed by the operating system. Estimated power is marked in the interface. Process energy scores are relative activity values, not watts.

## Development

Use the `dev` branch for development. Install Xcode 27, the current stable Rust toolchain, Node.js 24 LTS, and the pnpm version specified in `package.json`.

```sh
rustup target add aarch64-apple-darwin
pnpm install --frozen-lockfile
pnpm tauri dev
```

## Build and validation

To compile and link an unsigned ARM64 executable:

```sh
CODE_SIGNING_ALLOWED=NO MACOSX_DEPLOYMENT_TARGET=27.0 \
  pnpm tauri build --target aarch64-apple-darwin --no-bundle -- --locked
```

The executable is written to `target/aarch64-apple-darwin/release/powerflow`.

```sh
pnpm test
pnpm lint
pnpm build
cargo fmt --all -- --check
cargo test --workspace --locked
pnpm typecheck
```

Rust tests regenerate the frontend command bindings. GitHub Actions runs these checks on the `xcode-27` runner and verifies the executable's ARM64 architecture and macOS 27 deployment target. Build validation does not sign, notarize, or publish a release.

## License

[MIT](LICENSE)
