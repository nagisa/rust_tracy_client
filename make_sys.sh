#!/usr/bin/env bash
set -xe

if [ $# -eq 0 ]; then
  LAST_RELEASE=($(curl -s "https://api.github.com/repos/wolfpld/tracy/releases/latest" | jq -r '"\(.tag_name)\n\(.tarball_url)"'))
else
  LAST_RELEASE=($(curl -s "https://api.github.com/repos/wolfpld/tracy/releases/tags/$1" | jq -r '"\(.tag_name)\n\(.tarball_url)"'))
fi

TAG="${LAST_RELEASE[0]}"
TARBALL="${LAST_RELEASE[1]}"
DESTINATION=/tmp/tracy-$TAG # could use mktemp, but unnecessary complexity.
echo "::set-output name=tracy-tag::$TAG"
mkdir -p "$DESTINATION"

curl -sL "$TARBALL" -o - | tar -f - -zxC "$DESTINATION"

BASEDIR=("$DESTINATION"/*)
REQUIRED=($(gcc --dependencies -DTRACY_ENABLE "$BASEDIR/TracyClient.cpp" | grep -o "$BASEDIR/[^ \\]*"))

mkdir -p "tracy-client-sys/tracy"

bindgen "$BASEDIR/TracyC.h" \
  -o 'tracy-client-sys/src/generated.rs' \
  --whitelist-function='.*[Tt][Rr][Aa][Cc][Yy].*' \
  --whitelist-type='.*[Tt][Rr][Aa][Cc][Yy].*' \
  --size_t-is-usize \
  -- \
  -DTRACY_ENABLE

sed -i 's/^pub type/type/g' 'tracy-client-sys/src/generated.rs'
for REQUIRED_FILE in ${REQUIRED[@]}
do
  DEST_PATH=tracy-client-sys/tracy"${REQUIRED_FILE#$BASEDIR}"
  mkdir -p $(dirname "$DEST_PATH")
  cp "$REQUIRED_FILE" "$DEST_PATH"
done

cp "$BASEDIR/LICENSE" "tracy-client-sys/tracy/"
cp "$BASEDIR/libbacktrace/LICENSE" "tracy-client-sys/tracy/libbacktrace/"
