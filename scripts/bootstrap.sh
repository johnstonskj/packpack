LOG_LEVEL=6

LOG="$(basename "$(test -L "$0" && readlink "$0" || echo "$0")")"

log() {
  level=$1
  shift
  level_name=$1
  shift
  if [ $level -le $LOG_LEVEL ] ; then
    if [[ "$PACKAGE_NAME" == "" ]] ; then
      pname=
    else
      pname="${PACKAGE_NAME}::"
    fi
    printf "%8s %s > %s\n" "$level_name" "${pname}${LOG}" "$*"
  fi
  if [ $level -eq 0 ] ; then
    exit 1
  fi
}

log-critical() {
  log 0 "CRITICAL" "$@"
}
log-error() {
  log 1 "ERROR" "$@"
}
log-warning() {
  log 2 "WARNING" "$@"
}
log-info() {
  log 3 "INFO" "$@"
}
log-debug() {
  log 4 "DEBUG" "$@"
}
log-trace() {
  log 5 "TRACE" "$@"
}

# ---------------------------------------------------------------------------------------------------------------------

PACKAGE_NAME="packpack"
REPOSITORY_URL="https://raw.githubusercontent.com/johnstonskj/packpack/master/"

log-info "Checking curl exists"
if ! CURL=$(command -v curl)
then
    log-critical "curl could not be found, required to continue"
fi
log-debug "Using $CURL"

INSTALL="$HOME/.$PACKAGE_NAME"

log-info "Making directories in $INSTALL"
mkdir -p "$INSTALL/bin"
mkdir -p "$INSTALL/config"
mkdir -p "$INSTALL/pkgroot"

log-debug "Create environment file $INSTALL/env"
cat > "$INSTALL/env" <<EOF
export PATH="$INSTALL/bin:\$PATH"
EOF

log-info "Fetching logging script"
if ! ERR=$(curl -fsSLo "$INSTALL/bin/logging.sh" "$REPOSITORY_URL/scripts/logging.sh" 2>&1)
then
    log-critical "could not fetch platform script: $ERR"
fi
source "$INSTALL/bin/logging.sh"

log-info "Fetching platform script"
if ! ERR=$(curl -fsSLo "$INSTALL/bin/platform.sh" "$REPOSITORY_URL/scripts/platform.sh" 2>&1)
then
    log-critical "could not fetch platform script: $ERR"
fi
source "$INSTALL/bin/platform.sh"

log-debug "installer: $INSTALLER"
log-debug "application installer: $APP_INSTALLER"

log-info "Check for Git"
if ! CURL=$(command -v git)
then
    log-info "git could not be found, trying to install..."
fi

log-info "Check for Zsh"
if ! CURL=$(command -v zsh)
then
    log-info "zsh could not be found, trying to install..."
fi

