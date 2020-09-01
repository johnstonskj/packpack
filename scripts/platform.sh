OSSYS=`uname -s | tr '[:upper:]' '[:lower:]'`
OSDIST=
OSVERSION=`uname -r`
OSARCH=`uname -m`
INSTALLER=
APP_INSTALLER=

case $OSSYS in
  darwin)
    STDOUT=$(defaults read loginwindow SystemVersionStampAsString)
    if [[ $? -eq 0 ]] ; then
      OSSYS=macos
      OSVERSION=$STDOUT
    fi
    ;;
  linux)
    # If available, use LSB to identify distribution
    if [ -f /etc/lsb-release -o -d /etc/lsb-release.d ]; then
      OSDIST=$(lsb_release -i | cut -d: -f2 | sed s/'^\t'// | tr '[:upper:]' '[:lower:]')
    else
    # Otherwise, use release info file
      OSDIST=$(ls -d /etc/[A-Za-z]*[_-](version|release) | cut -d'/' -f3 | cut -d'-' -f1 | cut -d'_' -f1 | grep -v system | tr '[:upper:]' '[:lower:]')
    fi
    ;;
  msys*)
    OSSYS=windows
    OSVERSION=`ver | cut - d [ -f 2 | cut -d ] -f 1 | cut -d ' ' -f 2`
    ;;
  *)
    echo "Unknown OS $OSSYS unsupported" 2>&1
    exit 1
    ;;
esac

if [[ $OSSYS = macos ]] ; then
  INSTALLER=homebrew
  APP_INSTALLER=homebrew-apps
elif [[ $OSSYS = linux ]] ; then
  STDOUT=$(yum --version 2>&1)
  if [[ $? -eq 0 ]] ; then
    INSTALLER=yum
    APP_INSTALLER=flatpack
  else
    STDOUT=$(apt --version 2>&1)
    if [[ $? -eq 0 ]] ; then
      INSTALLER=apt
      APP_INSTALLER=snap
    else
      if [[ "$OSDIST" == "" ]] ; then
        echo "No known installer for $OSSYS-$OSVERSION-$OSARCH" >&2
      else
        echo "No known installer for $OSSYS-$OSVERSION-$OSARCH-$OSDIST" >&2
      fi
      exit 1
    fi
  fi
fi
