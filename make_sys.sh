#!/usr/bin/env bash

# NOTE: this script is only intended to be run on CI.
set -xe

if [ $# -eq 0 ]; then
  LAST_RELEASE=($(curl -s "https://api.github.com/repos/wolfpld/tracy/releases/latest" \
                    | jq -r '"\(.tag_name)\n\(.tarball_url)"'))
else
  LAST_RELEASE=($(curl -s "https://api.github.com/repos/wolfpld/tracy/releases/tags/$1" \
                    | jq -r '"\(.tag_name)\n\(.tarball_url)"'))
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
  --disable-header-comment \
  -- \
  -DTRACY_ENABLE
sed -i 's/pub type/type/g' 'tracy-client-sys/src/generated.rs'

for REQUIRED_FILE in ${REQUIRED[@]}
do
  DEST_PATH=tracy-client-sys/tracy"${REQUIRED_FILE#$BASEDIR}"
  mkdir -p $(dirname "$DEST_PATH")
  cp "$REQUIRED_FILE" "$DEST_PATH"
done

cp -r "$BASEDIR/libbacktrace" "tracy-client-sys/tracy/"
cp "$BASEDIR/LICENSE" "tracy-client-sys/tracy/"

# Avoid running the other steps if we haven't really updated tracy (e.g. if bindgen/rustfmt version
# changed)
if ! git diff --quiet "tracy-client-sys/tracy"; then
    echo "::set-output name=tracy-changed::true"
else
    exit 0
fi

CURRENT_SYS_VERSION=$(sed -n 's/version = "\(.*\)" # AUTO-BUMP/\1/p' tracy-client-sys/Cargo.toml)
CURRENT_CLIENT_VERSION=$(sed -n 's/version = "\(.*\)" # AUTO-BUMP/\1/p' tracy-client/Cargo.toml)
NEXT_SYS_VERSION="0.$(echo "$CURRENT_SYS_VERSION" \
  | sed -nr 's,[0-9]+\.([0-9]+)\.[0-9]+,\1,p' \
  | awk '{print $0+1}').0"
NEXTNEXT_SYS_VERSION="0.$(echo "$CURRENT_SYS_VERSION" \
  | sed -nr 's,[0-9]+\.([0-9]+)\.[0-9]+,\1,p' \
  | awk '{print $0+2}').0"
NEXT_CLIENT_VERSION="0.12.$(echo "$CURRENT_CLIENT_VERSION" \
  | sed -nr 's,[0-9]+\.[0-9]+\.([0-9]+),\1,p' \
  | awk '{print $0+1}')"

# Adjust the table in the README file…
sed -i "/^<!-- AUTO-UPDATE -->$/i $(printf "| $TAG | $NEXT_SYS_VERSION | 0.12.* | 0.7.* |")" \
    README.mkd
# …the version in tracy-client-sys…
sed -i "s/^\(version =\) \".*\" \(# AUTO-BUMP\)$/\1 \"$NEXT_SYS_VERSION\" \2/" \
    tracy-client-sys/Cargo.toml
# …and the versions in tracy-client.
sed -i "s/^\(version =\) \".*\" \(# AUTO-BUMP\)$/\1 \"$NEXT_CLIENT_VERSION\" \2/" \
    tracy-client/Cargo.toml
sed -i "s/^\(version =\) \".*\" \(# AUTO-UPDATE\)$/\1 \">=0.14.0, <$NEXTNEXT_SYS_VERSION\" \2/" \
    tracy-client/Cargo.toml

# Make a commit that we'll PR
NAME=tracy-client-sys-auto-update[bot]
MAIL="GitHub <noreply@github.com>"
git add tracy-client-sys tracy-client/Cargo.toml README.mkd
git -c user.name="$NAME" -c user.email="$MAIL" commit -m "Update Tracy client bindings to $TAG"
