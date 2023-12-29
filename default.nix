{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "hygiene_mnwe";
  version = "0.1.0";

  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  buildType = "release";

  meta = with lib; {
    description = "Helper for creating a new Rust scripting project";
    homepage = "https://github.com/bzm3r/hygiene_mnwe";
    license = with licenses; [ asl20 mit ];
    mainProgram = "hygiene_mnwe";
  };
}
