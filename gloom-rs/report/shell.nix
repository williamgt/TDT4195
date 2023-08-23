# This virtual environment may be loaded with the command `nix-shell`
# it require that you have nix installed, and nixpkgs configured as the a channel
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs;  [

    # pandoc and pandoc-lua-filters

    (pkgs.writeShellScriptBin "pandoc" ''
      export XDG_DATA_HOME=${pandoc-lua-filters}/share
      exec ${pkgs.pandoc}/bin/pandoc "$@"
    '')

    # optional pandoc filters:

    #pandoc-include
    #pandoc-imagine
    #haskellPackages.pandoc-crossref
    #haskellPackages.pandoc-include-code

    # latex environment

    (texlive.combine { inherit (texlive)
        scheme-small
        fontaxes
        atkinson
    ;})

  ];
}
