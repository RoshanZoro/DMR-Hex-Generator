# DMR AES Key Generator

A focused desktop utility for generating cryptographically random hex keys for
**DMR AES** encryption (AES-128 and AES-256). Keys are produced from the
operating system's secure random source, exist only in memory, and are wiped as
soon as they are no longer needed. Nothing is ever written to disk.

## Features

- AES-256 (64 hex characters) or AES-128 (32 hex characters)
- Generate 1–100 keys at once
- Per-key copy to clipboard, with automatic timed wipe
- Keys can be masked on screen and revealed individually
- Optional automatic clearing of keys from memory after a delay
- Clean interface that follows the system light/dark theme

## Security model

| Concern | Mitigation |
| --- | --- |
| Weak or predictable keys | Keys come directly from the operating system CSPRNG (`BCryptGenRandom` on Windows, `getrandom(2)` / `getentropy` on Linux and macOS). If the source ever fails, generation aborts and no key is produced. |
| Keys lingering in memory | Raw key bytes are held in a dedicated type that zeroizes its buffer on drop. Hex strings are likewise zeroized when they leave scope. |
| Keys paged to disk via swap | The key buffer is memory-locked (best effort) so the OS will not page it to swap. The status bar reports whether the lock succeeded. |
| Keys persisted to disk | Nothing — keys, settings, or otherwise — is written to disk. No log files, temporary files, or saved configuration. |
| Clipboard exposure | Copying is per-key and explicit. The clipboard is automatically wiped after a configurable delay, and only if it still holds the copied key, so it never overwrites unrelated content. It is also wiped on exit. |
| Onlookers | Keys can start hidden and be revealed one at a time, with bulk reveal/hide controls. |
| Stale keys on screen | Keys can be automatically cleared and zeroized from memory after a configurable timeout. |

> **Note:** Copying to the system clipboard is inherently exposed on systems
> with clipboard history (for example, Windows Win+V). The automatic wipe
> shortens the exposure window but cannot undo a history capture. Treat any
> copied key as exposed and program it promptly.

## Usage

1. Choose the key strength and how many keys to generate.
2. Adjust clipboard and memory options as needed.
3. Click **Generate keys**.
4. Use **Copy** on any key, or **Clear & wipe now** to remove all keys from
   memory immediately.

## Options

| Option | Description |
| --- | --- |
| Key strength | AES-256 (default) or AES-128 |
| Number of keys | 1 to 100 per generation |
| Uppercase hex | Render hex in upper or lower case |
| Hide keys by default | Start each key masked on screen |
| Clipboard auto-wipe | Enable/disable and set the delay in seconds |
| Auto-clear from memory | Enable/disable and set the delay in seconds |

The interface follows the system light/dark preference and updates live if it
changes. To force a theme, set the environment variable `DMR_THEME` to `dark`
or `light`.

## Build and run

The project is pinned to the GNU (MinGW) Rust toolchain through
`rust-toolchain.toml`, so it builds without Visual Studio or the MSVC linker.
MinGW's `gcc` must be on `PATH` (for example, `C:\tools\mingw64\bin`).

```sh
cargo run                     # debug build
cargo build --release         # optimized build
./target/release/dmr_hex.exe
```

To use the MSVC toolchain instead, delete `rust-toolchain.toml`.

## Project structure

Each concern lives in its own module:

```
src/
├── main.rs       Entry point and window setup
├── app/          Application state, layout, and event handling
├── crypto/       CSPRNG key generation and hex encoding
├── security/     Key buffer: memory zeroizing and locking
├── clipboard/    Clipboard writes and verified auto-wipe
├── config/       User options (held in memory only)
└── ui/           Theme and shared visual constants
```

## Dependencies

| Crate | Purpose |
| --- | --- |
| `eframe` / `egui` | Graphical interface |
| `getrandom` | Operating system CSPRNG |
| `zeroize` | Memory wiping |
| `arboard` | Clipboard access |
| `region` | Memory locking |
