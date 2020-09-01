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
