{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "birdhunt-devshell";

  packages = with pkgs; [
    wayland
    fontconfig
    pkg-config
  ];

  # winit needs these
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
    with pkgs;
    [
      wayland
      libxkbcommon
      libGL
    ]
  );
}
