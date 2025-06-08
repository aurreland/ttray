{
  description = "Ttray Flake configuration";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit system;};
        });
  in {
    packages = forEachSupportedSystem ({pkgs}: {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "ttray";
        version = "0.1.0";

        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        meta = {
          description = "A Simple Tui for Tray Applications";
          license = pkgs.lib.licenses.gpl3;
        };
      };
    });

    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rust-analyzer
        ];

        env.RUST_SRC_PATH = "${pkgs.rustc}/lib/rustlib/src/rust/library";
      };
    });

    formatter = forEachSupportedSystem ({pkgs}: pkgs.alejandra);
  };
}
