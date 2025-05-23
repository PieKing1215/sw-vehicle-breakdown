# Based on https://nixos.wiki/wiki/Rust#Installation_via_rustup

{ pkgs ? import <nixpkgs> {} }:
  let
    overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
    libPath = with pkgs; lib.makeLibraryPath [
      # load external libraries that you need in your rust project here
      alsa-lib
      vulkan-loader
      udev
      xorg.libX11
      xorg.libXrandr
      xorg.libXcursor
      xorg.libXi
      libxkbcommon
    ];
in
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      clang
      # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
      llvmPackages_19.bintools
      rustup
      pkg-config
      openssl
      systemd
      (pkgs.rustPlatform.buildRustPackage rec {
        pname = "perseus-cli";
        version = "0.4.2"; # Pick version

        src = pkgs.fetchFromGitHub {
          owner = "framesurge";
          repo = "perseus";
          rev = "v${version}";
          sha256 = "sha256-zgT0wHf29NBgVOGACK1YiB9+ZdBq+mRV1+N9Vuftqok=";
        };

        buildAndTestSubdir = "packages/perseus-cli";

        doCheck = false;

        cargoPatches = [ ./perseus_Cargo.lock.patch ];

        cargoHash = "sha256-8w1uEFSXAufN8h9AL0jMXN/sTPogpsNDr7xWCEHXLeg=";

        nativeBuildInputs = [
          pkg-config
          openssl
        ];

        buildInputs = [
          pkg-config
          openssl
        ];
      })
    ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      '';
    # Add precompiled library to rustc search path
    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      # add libraries here (e.g. pkgs.libvmi)
    ]);
    LD_LIBRARY_PATH = libPath;
    # Add glibc, clang, glib, and other headers to bindgen search path
    BINDGEN_EXTRA_CLANG_ARGS =
    # Includes normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # add dev libraries here (e.g. pkgs.libvmi.dev)
      pkgs.glibc.dev
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];
  }
