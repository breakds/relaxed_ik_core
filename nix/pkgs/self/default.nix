{
  stdenv,
  lib,
  buildPythonPackage,
  fetchFromGitHub,
  pytestCheckHook,
  pythonOlder,
  rustPlatform,
  numpy
}:

buildPythonPackage rec {
  pname = "relaxed-ik";
  version = "0.1.0";
  format = "pyproject";

  disabled = pythonOlder "3.7";

  src = ../../..;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    name = "${pname}-${version}";
    hash = "sha256-0knwf3x0+FF7+czACh+Htds9m3p/wnv84j4qpDe5Hb8=";
  };

  nativeBuildInputs = with rustPlatform; [
    cargoSetupHook
    maturinBuildHook
  ];

  propagatedBuildInputs = [
    numpy
  ];

  pythonImportsCheck = [ "relaxed_ik_lib" ];
}
