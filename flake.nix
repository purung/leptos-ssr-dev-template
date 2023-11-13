{
  description = "A basic Rust devshell for NixOS users developing Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        inherit (pkgs) lib;
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        cargo-leptos = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "0.2.2";
          buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to download Ruby and other things without this feature

          src = pkgs.fetchgit {
            url = "https://github.com/purung/cargo-leptos";
            rev = "1d38a99f4302c3754ae1f353b5985d87bf0775b5";
            sha256 = "hGNlnTpy05EX6WPmo4QkFv7SO+DFtsgDghZc53iY0kg=";
          };
          nativeBuildInputs = [ pkgs.pkg-config pkgs.openssl ];
          cargoVendorDir = "vendor";
          doCheck = false;
          buildInputs = with pkgs;
            [ openssl pkg-config ]
            ++ lib.optionals stdenv.isDarwin [
              Security
            ];
          meta = with lib; {
            description = "A build tool for the Leptos web framework";
            homepage = "https://github.com/leptos-rs/cargo-leptos";
            changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
            license = licenses.asl20;
          };
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            cacert
            cargo-make
            trunk
            tailwindcss
            leptosfmt
            mold
            cargo-leptos
            binaryen
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" "rustc-codegen-cranelift-preview" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ];

          shellHook = ''
            '';
        };
      }
    );
}
