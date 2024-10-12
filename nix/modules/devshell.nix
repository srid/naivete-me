{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "naivete-me-shell";
      inputsFrom = [
        self'.devShells.rust
        config.treefmt.build.devShell
      ];
      packages = with pkgs; [
        just
        nixd # Nix language server
        leptosfmt
        bacon
        trunk
      ];
    };
  };
}
