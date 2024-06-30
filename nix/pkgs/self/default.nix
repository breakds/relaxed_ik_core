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
    hash = "sha256-VFGiA1fkBnI3AY1ifRQqW8V71KtlD7uYZNbM/LClLlc=";
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
