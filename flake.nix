# ADAPTED FROM THE SHELL BY THE GOAT AND NIX MAGICIAN: https://github.com/ede1998
# - https://github.com/ede1998/ireplay/blob/main/shell.nix
# - https://github.com/oxalica/rust-overlay/issues/89#issuecomment-2535801379
{
  description = "ESP32-S3 Rust Development Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # --- Derivation for Rust Toolchain ---
        host-triple = "x86_64-unknown-linux-gnu";
        gcc-arch = "xtensa-esp-elf";
        gcc-release = "15.2.0_20251204";

        toolchain-pkg = pkgs.stdenv.mkDerivation rec {
          pname = "esp32-xtensa-rust-toolchain";
          version = "1.92.0.0";

          srcs = [
            (pkgs.fetchurl {
              url = "https://github.com/esp-rs/rust-build/releases/download/v${version}/rust-${version}-${host-triple}.tar.xz";
              hash = "sha256-vLCRgGo+El8ParhSFcHrcXT7w4g5dZdlUJ6d0lVl65U=";
            })
            (pkgs.fetchurl {
              url = "https://github.com/espressif/crosstool-NG/releases/download/esp-${gcc-release}/${gcc-arch}-${gcc-release}-x86_64-linux-gnu.tar.xz";
              hash = "sha256-PVD1zV8XOs/VJOB8HNabyZWFcxpBXKLlvOh5mX/mArg=";
            })
            (pkgs.fetchurl {
              url = "https://github.com/esp-rs/rust-build/releases/download/v${version}/rust-src-${version}.tar.xz";
              hash = "sha256-FZV3I1rHhZHIUKQyrRqYJzX+kQ0EldJufFwXZDrMS08=";
            })
          ];

          sourceRoot = ".";

          nativeBuildInputs = [ pkgs.autoPatchelfHook ];
          buildInputs = [
            pkgs.zlib
            pkgs.stdenv.cc.cc.lib # Provides libstdc++
          ];

          installPhase = ''
            runHook preInstall

            # The install scripts need shebang patching to run on NixOS
            patchShebangs --build rust-nightly-x86_64-unknown-linux-gnu/install.sh rust-src-nightly/install.sh

            # Install Rust
            rust-nightly-x86_64-unknown-linux-gnu/install.sh --destdir="$out" --prefix="" --without=rust-docs-json-preview,rust-docs --disable-ldconfig
            rust-src-nightly/install.sh --destdir="$out" --prefix="" --disable-ldconfig

            # Install GCC/Xtensa bins
            # We copy the whole directory and then symlink binaries to $out/bin so they are in PATH
            cp -pr --reflink=auto -- xtensa-esp-elf "$out";
            ln -s $out/xtensa-esp-elf/bin/* "$out/bin/"

            runHook postInstall
          '';
        };

      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.espflash
            toolchain-pkg
          ];

          # Optional: sometimes helper tools need to know where libs are
          shellHook = ''
            export LD_LIBRARY_PATH="${
              pkgs.lib.makeLibraryPath [
                pkgs.zlib
                pkgs.stdenv.cc.cc.lib
              ]
            }:$LD_LIBRARY_PATH"
            echo "ESP32-S3 Environment Loaded (Using ESP Rust v${toolchain-pkg.version})"
          '';
        };
      }
    );
}
