{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "birdhunt-devshell";

  packages = with pkgs; [
    pkg-config

    # uing deps
    wayland
    fontconfig

    # ssh2 deps
    openssl
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
    with pkgs;
    [
      # winit deps
      wayland
      libxkbcommon
      libGL

      # ssh2 deps
      libssh2
    ]
  );
}
