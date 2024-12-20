{
  lib,
  craneLib,
}: let
  commonArgs = {
    src = craneLib.cleanCargoSource (craneLib.path ./.);
  };
in
  craneLib.buildPackage (
    commonArgs
    // {
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    }
  )
