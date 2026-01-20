# Nix & direnv Setup Guide

## What's Been Set Up

✅ **flake.nix** - Updated with all development tools:
- Rust toolchain with WASM target
- cargo-leptos for web development
- PostgreSQL & sqlx-cli
- Docker & podman
- SASS/Dart-sass for styling
- Google Cloud SDK
- Development helpers

✅ **.envrc** - direnv configuration:
- Automatically loads Nix flake
- Sets environment variables:
  - `RUST_LOG=info`
  - `DATABASE_URL` for local development
  - `RUST_BACKTRACE=1` for debugging

✅ **.direnvrc** - Additional direnv functions:
- `PATH_add_relative scripts` - Adds scripts to PATH
- Helper functions for dev tasks

✅ **.env.example** - Environment variable template:
- Database configuration
- JWT settings
- Leptos configuration

## Setup Instructions

### 1. Install direnv (if not already installed)
```bash
brew install direnv
```

### 2. Add direnv to your shell

For **zsh** (add to `~/.zshrc`):
```bash
eval "$(direnv hook zsh)"
```

For **bash** (add to `~/.bashrc`):
```bash
eval "$(direnv hook bash)"
```

Restart your terminal or run:
```bash
source ~/.zshrc  # or source ~/.bashrc
```

### 3. Allow direnv in the project
```bash
cd /Users/icarus/localcluster/computer-stuff/development-projects/jakewray.dev
direnv allow
```

### 4. Verify it worked
```bash
which cargo
cargo --version
```

Should show Nix-managed versions without error.

## Usage

### Automatic Setup (direnv)
Just `cd` into the project directory - direnv will:
- Load all environment variables
- Add tools to PATH
- Show helpful startup message

### Manual Setup (without direnv)
```bash
nix develop
```

This opens a shell with all dev tools available.

## Daily Workflow

```bash
# 1. Enter project (if not using direnv, run: nix develop)
cd jakewray.dev

# 2. Start database
./scripts/setup-dev.sh

# 3. Start dev server
cargo leptos watch
```

The environment variables from `.envrc` are automatically set:
- `DATABASE_URL` is set to connect to local PostgreSQL
- `RUST_LOG=info` enables info-level logging
- Rust development tools are in PATH

## Updating Dependencies

If you add new dependencies or update the Nix flake:

```bash
# Update flake.lock file
nix flake update

# Re-enter the environment
direnv reload
# or manually: nix develop
```

## Troubleshooting

### ".envrc is blocked" error
```bash
direnv allow
```

### Changes to `.envrc` not taking effect
```bash
direnv reload
```

### Nix not working
```bash
# Update flake.lock
nix flake update

# Clear cache
rm -rf ~/.cache/nix

# Try again
nix develop
```

### "Command not found" errors
Make sure you've:
1. ✅ Installed direnv and hooked it to your shell
2. ✅ Run `direnv allow` in the project
3. ✅ Reloaded your terminal

Verify: `which cargo` should show a `/nix/store/...` path

## Environment Variables Available

When direnv loads (or after `nix develop`), you have:

```bash
RUST_SRC_PATH          # Rust source for IDE
RUST_LOG               # Set to 'info' for logging
RUST_BACKTRACE         # Set to '1' for better error traces
DATABASE_URL           # PostgreSQL connection string
```

You can override these in your local shell:
```bash
export RUST_LOG=debug
cargo leptos watch
```

## What Gets Installed

From **flake.nix**:
- rust-toolchain (latest stable with wasm32 target)
- cargo-leptos
- pkg-config, openssl (for compilation)
- postgresql, sqlx-cli (database tools)
- sass, dart-sass (styling)
- docker, docker-compose (containerization)
- nodejs (optional JS tools)
- google-cloud-sdk (deployment)

All managed by Nix - no global installation needed!

## Benefits of This Setup

✅ **Reproducible** - Everyone gets exact same versions
✅ **Isolated** - No conflicts with other projects
✅ **Automatic** - Just `cd` into directory
✅ **Clean** - No `.bashrc` pollution
✅ **Easy** - `direnv allow` and you're done

## More Info

- [direnv docs](https://direnv.net/)
- [Nix docs](https://nixos.org/manual/nix/unstable/)
- [Rust flakes template](https://github.com/NixOS/templates)

## Next Steps

1. ✅ Set up direnv (or use `nix develop` manually)
2. Run `./scripts/setup-dev.sh` to initialize database
3. Run `cargo leptos watch` to start development
4. Visit `http://localhost:3000`

See [LOCAL_DEV.md](LOCAL_DEV.md) for full development guide.
