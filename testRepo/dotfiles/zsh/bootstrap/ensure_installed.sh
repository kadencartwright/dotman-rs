result=$(pacman -Q | grep -q "$1")
exit $result
