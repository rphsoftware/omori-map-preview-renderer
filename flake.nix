{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay , ...}:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        native-deps = with pkgs; [
          pkg-config
        ];
      in 
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = ["x86_64-unknown-linux-gnu" "arm64e-apple-darwin" "x86_64-pc-windows-gnu"];
            })
            cargo-bloat
          ] ++ native-deps;
          LD_LIBRARY_PATH = (lib.makeLibraryPath native-deps);
        };
      }
    );
}