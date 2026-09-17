#!/usr/bin/env bash

ensure_mise() {
  export PATH="$HOME/.local/bin:$PATH"
  curl --fail --silent --show-error --location https://mise.run |
    MISE_INSTALL_PATH="$HOME/.local/bin/mise" MISE_INSTALL_SKIP_IF_EXISTS=1 sh
}
