{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    python39
    rustc
    cargo
    clippy
    rustfmt
  ] ++ (with python39Packages; [
    pip
    grpcio
    grpcio-tools
  ]);
  shellHook = ''
    export PIP_PREFIX=$(pwd)/_build/pip_packages
    export PYTHONPATH="$PIP_PREFIX/${python39.sitePackages}:$PYTHONPATH"
    export PATH="$PIP_PREFIX/bin:$PATH"
    export PROTOC="${protobuf}/bin/protoc"
    export PROTOC_INCLUDE="${protobuf}/include"
    unset SOURCE_DATE_EPOCH
  '';
}
