{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/2af23d5f787161f7ea8d994c394bd37c9dcf6958";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      llvm = pkgs.llvmPackages_21;
    in
    {
      devShells."${system}".default = pkgs.mkShell.override { stdenv = llvm.libcxxStdenv; } {
        packages = [
          pkgs.cmake
          llvm.clang-tools
          llvm.lld
          pkgs.cargo
          pkgs.rustc
          pkgs.wasm-pack
          pkgs.wasm-bindgen-cli
          pkgs.wasm-language-tools
          pkgs.wasm-tools
          pkgs.wabt
          pkgs.rust-analyzer
          pkgs.typescript-language-server
        ];
      };
    };
}
