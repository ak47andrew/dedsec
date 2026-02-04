URL=https://github.com/ak47andrew/dedsec/releases/latest/download/dedsec-x86_64-unknown-linux-gnu
TMP_DIR=$(mktemp -d)
FILENAME="$TMP_DIR/app"
curl -L -o "$FILENAME" $URL > /dev/null
chmod +x "$FILENAME"
clear
"$FILENAME"
rm -rf TMP_DIR