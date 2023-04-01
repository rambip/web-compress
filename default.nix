# the definition is in "flake.nix"
# it is a more modern tool, this file is for compatibility
(import (fetchTarball https://github.com/edolstra/flake-compat/archive/master.tar.gz) {
  src = builtins.fetchGit ./.;
}).defaultNix
