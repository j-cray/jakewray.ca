# Build Caching Strategy

## Overview

Build caching is now optimized for both local development and production deployments to minimize rebuild times.

## Local Development Caching

### 1. Cargo Build Cache
- **Location**: `target/` directory (git-ignored)
- **What's cached**: Compiled dependencies and incremental builds
- **Benefit**: Only changed files recompile

### 2. Cargo Registry Cache
- **Location**: `.cargo/registry/` and `.cargo/git/`
- **What's cached**: Downloaded dependencies
- **Config**: See [.cargo/config.toml](.cargo/config.toml)

### 3. Nix Store Cache
- **Location**: `/nix/store/`
- **What's cached**: All Nix packages (Rust toolchain, cargo-leptos, etc.)
- **Benefit**: Instant environment loading with `direnv`

### 4. cargo-leptos Cache
- **Location**: `target/front/`, `target/site/`
- **What's cached**: WASM builds and compiled frontend assets
- **Benefit**: Frontend rebuilds only when source changes

### Local Cache Optimization

The `.cargo/config.toml` includes:
- **Incremental compilation** enabled for dev builds
- **Optimized dependencies** (O3) even in dev mode
- **Faster sparse registry** downloads

**Typical rebuild times:**
- First build: ~5-10 minutes (downloads all deps)
- Incremental rebuild: ~10-30 seconds (only changed files)
- No-op rebuild: ~2-5 seconds (just checking)

## Production Deployment Caching

### 1. Docker BuildKit Cache Mounts

The Dockerfile uses BuildKit cache mounts:
```dockerfile
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release
```

**Benefit**: Cargo dependencies persist between builds on the server.

### 2. Multi-Stage Build Caching

Layers are optimized for maximum cache hits:

**Stage 1: deps** (rarely changes)
- System packages
- Rust toolchain
- cargo-leptos, sass, sqlx-cli
- WASM target

**Stage 2: planner** (changes when Cargo.toml changes)
- Copies only `Cargo.toml` and `Cargo.lock` files
- Triggers rebuild only when dependencies change

**Stage 3: builder** (smart dependency caching)
1. Copy lockfiles
2. Create dummy source files
3. Build dependencies (cached!)
4. Copy real source
5. Build actual app (only app code compiles)

**Stage 4: runtime** (minimal, fast)
- Only final binaries copied
- Small image size

### 3. Docker Layer Caching

Each step creates a cacheable layer:
- `FROM rust:bookworm` → Cached base image
- `RUN apt-get install` → Cached package layer
- `RUN cargo binstall` → Cached tool installation
- Dependency build → Cached separately from app build

### 4. BuildKit Inline Cache

The build scripts now use:
```bash
export DOCKER_BUILDKIT=1
docker build --build-arg BUILDKIT_INLINE_CACHE=1
```

This pushes cache metadata into the image for reuse across builds.

## Deployment Build Times

**First deployment:**
- ~15-20 minutes (downloads everything)

**Subsequent deployments (code changes only):**
- ~2-5 minutes (only rebuilds changed code)

**Subsequent deployments (dependency changes):**
- ~8-12 minutes (rebuilds deps + code)

**No changes (redeploy):**
- ~1-2 minutes (cache hits on everything)

## Cache Management

### Clear Local Cache
```bash
# Remove build artifacts (keeps dependencies)
cargo clean

# Remove everything including downloaded deps
rm -rf target .cargo/registry .cargo/git
```

### Clear Docker Build Cache
On the server:
```bash
# Prune build cache
docker builder prune

# Remove all cache
docker system prune -a
```

### View Cache Usage
```bash
# Local cargo cache
du -sh target/

# Docker cache
docker system df
```

## Optimization Tips

### For Faster Local Development

1. **Keep `target/` intact** - Don't delete it between sessions
2. **Use `cargo leptos watch`** - Incremental compilation
3. **Upgrade to SSD** - Dramatically faster I/O for compilation
4. **Increase RAM** - More cache in memory

### For Faster Deployments

1. **Don't modify dependencies often** - Triggers full rebuild
2. **Use `./scripts/deploy.sh backend`** - Faster than `all`
3. **Keep Docker images** - Don't prune unless necessary
4. **Use BuildKit** - Already enabled in scripts

## Cache Locations Summary

| Type | Local Dev | Production |
|------|-----------|------------|
| Cargo deps | `target/` | Docker cache mount |
| Cargo registry | `.cargo/` | Docker cache mount |
| Nix packages | `/nix/store/` | N/A |
| Docker layers | N/A | Docker daemon |
| WASM builds | `target/front/` | Docker cache mount |
| Final artifacts | `target/site/` | Copied to runtime image |

## Troubleshooting

### "Out of disk space" during build
```bash
# Clean up Docker
docker system prune -a

# Clean up Nix (if using)
nix-collect-garbage -d
```

### Builds seem slow despite caching
```bash
# Verify BuildKit is enabled
echo $DOCKER_BUILDKIT  # Should be "1"

# Check cache hits in build output
docker build --progress=plain .
# Look for "CACHED" in output
```

### Cache not being used
```bash
# Ensure you're using the same Dockerfile
# Ensure Cargo.lock hasn't changed unexpectedly
git status Cargo.lock

# Rebuild base images
docker build --target deps -t portfolio-deps .
```

## Further Optimizations (Future)

- [ ] Use `sccache` for distributed Rust compilation caching
- [ ] Set up Docker registry for image caching
- [ ] Use GitHub Actions cache for CI/CD
- [ ] Implement cargo-chef for better dependency caching
- [ ] Use remote cache backend (GCS/S3)

## Resources

- [Docker BuildKit docs](https://docs.docker.com/build/buildkit/)
- [Cargo build cache](https://doc.rust-lang.org/cargo/guide/build-cache.html)
- [cargo-leptos caching](https://github.com/leptos-rs/cargo-leptos)
