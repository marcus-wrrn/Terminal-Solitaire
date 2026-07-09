#!/bin/sh
# Installer for Terminal Solitaire.
# Usage: curl -sSfL https://raw.githubusercontent.com/marcus-wrrn/Terminal-Solitaire/main/install.sh | sh
#
# Environment variables:
#   SOLITAIRE_INSTALL_DIR  install location (default: ~/.local/bin)
#   SOLITAIRE_VERSION      tag to install, e.g. v0.1.5 (default: latest release)

set -eu

REPO="marcus-wrrn/Terminal-Solitaire"
BIN="solitaire"
INSTALL_DIR="${SOLITAIRE_INSTALL_DIR:-$HOME/.local/bin}"

err() {
    printf 'error: %s\n' "$1" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || err "'$1' is required but not installed"
}

download() {
    # download <url> <dest>
    if command -v curl >/dev/null 2>&1; then
        curl -sSfL "$1" -o "$2"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$1" -O "$2"
    else
        err "curl or wget is required"
    fi
}

detect_target() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$arch" in
        x86_64 | amd64) arch="x86_64" ;;
        aarch64 | arm64) arch="aarch64" ;;
        *) err "unsupported architecture: $arch" ;;
    esac

    case "$os" in
        Linux)
            libc="gnu"
            # Static musl build for systems without glibc (e.g. Alpine)
            if ! ldd --version 2>&1 | grep -qiE 'glibc|gnu libc'; then
                libc="musl"
            fi
            if [ "$arch" = "aarch64" ]; then
                libc="gnu" # no aarch64 musl build published
            fi
            target="${arch}-unknown-linux-${libc}"
            ext="tar.gz"
            ;;
        Darwin)
            target="${arch}-apple-darwin"
            ext="tar.gz"
            ;;
        MINGW* | MSYS* | CYGWIN* | Windows_NT)
            target="${arch}-pc-windows-msvc"
            ext="zip"
            BIN="solitaire.exe"
            ;;
        *)
            err "unsupported operating system: $os"
            ;;
    esac
}

resolve_version() {
    if [ -n "${SOLITAIRE_VERSION:-}" ]; then
        VERSION="$SOLITAIRE_VERSION"
        return
    fi
    # Follow the redirect from /releases/latest to find the newest tag
    url="https://github.com/$REPO/releases/latest"
    if command -v curl >/dev/null 2>&1; then
        VERSION="$(curl -sSfL -o /dev/null -w '%{url_effective}' "$url" | sed 's|.*/tag/||')"
    else
        VERSION="$(wget -q --max-redirect=0 -S -O /dev/null "$url" 2>&1 | sed -n 's|.*Location:.*/tag/\([^ ]*\).*|\1|p' | tr -d '\r')"
    fi
    [ -n "$VERSION" ] || err "could not determine the latest release; set SOLITAIRE_VERSION and retry"
}

main() {
    detect_target
    resolve_version

    archive="solitaire-$VERSION-$target.$ext"
    url="https://github.com/$REPO/releases/download/$VERSION/$archive"

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    printf 'Downloading %s %s (%s)...\n' "$BIN" "$VERSION" "$target"
    download "$url" "$tmpdir/$archive"

    case "$ext" in
        tar.gz)
            need tar
            tar -xzf "$tmpdir/$archive" -C "$tmpdir"
            ;;
        zip)
            need unzip
            unzip -q "$tmpdir/$archive" -d "$tmpdir"
            ;;
    esac

    [ -f "$tmpdir/$BIN" ] || err "archive did not contain the '$BIN' binary"

    mkdir -p "$INSTALL_DIR"
    install -m 755 "$tmpdir/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null \
        || { cp "$tmpdir/$BIN" "$INSTALL_DIR/$BIN" && chmod 755 "$INSTALL_DIR/$BIN"; }

    printf 'Installed %s to %s\n' "$BIN" "$INSTALL_DIR/$BIN"

    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            printf '\nwarning: %s is not on your PATH.\n' "$INSTALL_DIR"
            printf 'Add this to your shell profile:\n'
            printf '  export PATH="%s:$PATH"\n' "$INSTALL_DIR"
            ;;
    esac
}

main
