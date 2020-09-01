LOG_COLOR=yes
LOG_LEVEL=6

LOG="$(basename "$(test -L "$0" && readlink "$0" || echo "$0")")"

CRED="$(tput setaf 1)"
CGREEN="$(tput setaf 2)"
CYELLOW="$(tput setaf 3)"
CRESET="$(tput sgr0)"
CBOLD="$(tput bold)"
CDIM="$(tput dim)"

__log() {
  color=$1
  shift
  level=$1
  shift
  if [[ "$PACKAGE_NAME" == "" ]] ; then
    pname=
  else
    pname="${PACKAGE_NAME}::"
  fi
  if [[ "$LOG_COLOR" == "" ]] ; then
    printf "%8s %s > %s\n" "$level" "${pname}${LOG}" "$*"
  elif [[ "$color" == "$CDIM" ]] ; then
    printf "%s%8s%s %s > %s\n" "$color" "$level" "$CRESET" "${pname}${LOG}" "${CDIM}$*${CRESET}"
  else
    printf "%s%8s%s %s%s%s > %s\n" "$color" "$level" "$CRESET" "${CBOLD}" "${pname}${LOG}" "${CRESET}" "$*"
  fi
}

log() {
  level=$1
  shift
  if [ $level -le $LOG_LEVEL ] ; then
    case $level in
        0)
      __log "$CRED" "CRITICAL" "$@";
      exit 1
      ;;
        1)
      __log "$CRED" "ERROR" "$@"
      ;;
        2)
      __log "$CYELLOW" "WARNING" "$@"
      ;;
        3)
      __log "$CGREEN" "INFO" "$@"
      ;;
        4)
      __log "$CDIM" "DEBUG" "$@"
      ;;
        *)
      __log "$CDIM" "TRACE" "$@"
      ;;
    esac
  fi
}

log-critical() {
  log 0 "$@"
}
log-error() {
  log 1 "$@"
}
log-warning() {
  log 2 "$@"
}
log-info() {
  log 3 "$@"
}
log-debug() {
  log 4 "$@"
}
log-trace() {
  log 5 "$@"
}

# ---------------------------------------------------------------------------------------------------------------------

PACKAGE_NAME="packpack"

log-info "Checking curl exists"
if ! CURL=$(command -v curl)
then
    log-critical "curl could not be found, required to continue"
fi
log-debug "Using $CURL"

log-info "Determine the current platform"
OSSYS=$(uname -s | tr '[:upper:]' '[:lower:]')
OSARCH=$(uname -m)
log-debug "platform = $OSSYS-$OSARCH"

INSTALL="$HOME/.$PACKAGE_NAME"

log-info "Making directories in $INSTALL"
mkdir -p "$INSTALL/bin"
mkdir -p "$INSTALL/config"
mkdir -p "$INSTALL/pkgroot"

log-debug "Create environment file $INSTALL/env"
cat > "$INSTALL/env" <<EOF
export PATH="$INSTALL/bin:\$PATH"
EOF

log-info "Fetching platform script"
if ! ERR=$(curl "" 2>&1)
then
    log-critical "could not fetch platform script: $ERR"
fi

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

