{ stdenv, fetchurl }:

stdenv.mkDerivation rec {
  name = "openssl-replit";
  src = fetchurl {
    url = "https://www.openssl.org/source/openssl-1.1.1l.tar.gz";
    sha256 = "0z76l78glchx4sbn5cs0yl5zfi0xq3px3vjwz0z3x56nj1vwn92i";
  };
  phases = [ "unpackPhase" "buildPhase" "installPhase" ];
  installPhase = ''
    mkdir -p $out
    cp -r * $out
  '';
}
