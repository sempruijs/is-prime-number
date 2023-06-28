# syntax=docker/dockerfile:1
FROM nixos/nix
RUN nix build "github:sempruijs/is-prime-number" --extra-experimental-features nix-command --extra-experimental-features flakes

CMD ./result/bin/is-prime-number
