{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
          targets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-pc-windows-gnu"
          ];
        };

        # Add MinGW (Windows GNU) cross toolchain for C/ASM parts of crates
        mingw = pkgs.pkgsCross.mingwW64;

        buildInputs = with pkgs; [
          rustToolchain
          expat
          fontconfig
          freetype
          freetype.dev
          libGL
          pkg-config
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          libxkbcommon
          openssl
          # Cross toolchain components (gcc, binutils, etc.)
          mingw.stdenv.cc
          # Windows threading libraries
          mingw.windows.pthreads
          # Wine for running Windows executables
          wine
          winetricks
        ];
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          # Linker + tool overrides for cargo/cc when targeting Windows
          CC_x86_64_pc_windows_gnu = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
          CXX_x86_64_pc_windows_gnu = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-g++";
          AR_x86_64_pc_windows_gnu = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-ar";
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
          # (Optional) if a build script needs pkg-config for Windows libs:
          PKG_CONFIG_x86_64_pc_windows_gnu = "${mingw.stdenv.cc}/bin/x86_64-w64-mingw32-pkg-config";
          
          # Windows cross-compilation library paths
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L ${mingw.windows.pthreads}/lib -C link-arg=-static-libgcc -C link-arg=-static -C link-arg=-lpthread";

          LD_LIBRARY_PATH =
            builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
        };
      }
    );
}
