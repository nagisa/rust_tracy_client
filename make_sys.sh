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
DEFINES=("-DTRACY_ENABLE" "-DTRACY_MANUAL_LIFETIME" "-DTRACY_FIBERS")

echo "::set-output name=tracy-tag::$TAG"
mkdir -p "$DESTINATION"
curl -sL "$TARBALL" -o - | tar -f - -zxC "$DESTINATION"
BASEDIR=("$DESTINATION"/*)

rm -rf "tracy-client-sys/tracy/"
cp -r "$BASEDIR/public" "tracy-client-sys/tracy"
cp "$BASEDIR/LICENSE" "tracy-client-sys/tracy/"

bindgen "tracy-client-sys/tracy/tracy/TracyC.h" \
  -o 'tracy-client-sys/src/generated.rs' \
  --whitelist-function='.*[Tt][Rr][Aa][Cc][Yy].*' \
  --whitelist-type='.*[Tt][Rr][Aa][Cc][Yy].*' \
  --size_t-is-usize \
  --disable-header-comment \
  -- \
  ${DEFINES[@]}
# The space after type avoids hitting members called "type".
sed -i 's/pub type /type /g' 'tracy-client-sys/src/generated.rs'

# Avoid running the other steps if we haven't really updated tracy (e.g. if bindgen/rustfmt version
# changed)
if ! git diff --quiet "tracy-client-sys/tracy"; then
    echo "::set-output name=tracy-changed::true"
else
    exit 0
fi

CURRENT_SYS_VERSION=$(sed -n 's/^version = "\(.*\)" # AUTO-BUMP$/\1/p' tracy-client-sys/Cargo.toml)
CURRENT_CLIENT_VERSION=$(sed -n 's/^version = "\(.*\)" # AUTO-BUMP$/\1/p' tracy-client/Cargo.toml)
CURRENT_TRACING_VERSION=$(sed -n 's/^version = "\(.*\)"$/\1/p' tracing-tracy/Cargo.toml)
NEXT_SYS_VERSION="0.$(echo "$CURRENT_SYS_VERSION" \
  | sed -nr 's,[0-9]+\.([0-9]+)\.[0-9]+,\1,p' \
  | awk '{print $0+1}').0"
NEXTNEXT_SYS_VERSION="0.$(echo "$CURRENT_SYS_VERSION" \
  | sed -nr 's,[0-9]+\.([0-9]+)\.[0-9]+,\1,p' \
  | awk '{print $0+2}').0"
NEXT_CLIENT_VERSION="0.14.$(echo "$CURRENT_CLIENT_VERSION" \
  | sed -nr 's,[0-9]+\.[0-9]+\.([0-9]+),\1,p' \
  | awk '{print $0+1}')"

# Adjust the table in the README file…
sed -i "/^<!-- AUTO-UPDATE -->$/i $(printf "| %-6s | %-15s | %-12s | %-13s |" "$TAG" "$NEXT_SYS_VERSION" "$NEXT_CLIENT_VERSION" "$CURRENT_TRACING_VERSION")" \
    README.mkd
# …the version in tracy-client-sys…
sed -i "s/^\(version =\) \".*\" \(# AUTO-BUMP\)$/\1 \"$NEXT_SYS_VERSION\" \2/" \
    tracy-client-sys/Cargo.toml
# …and the versions in tracy-client.
sed -i "s/^\(version =\) \".*\" \(# AUTO-BUMP\)$/\1 \"$NEXT_CLIENT_VERSION\" \2/" \
    tracy-client/Cargo.toml
sed -i "s/^\(version =\) \".*\" \(# AUTO-UPDATE\)$/\1 \">=0.17.0, <$NEXTNEXT_SYS_VERSION\" \2/" \
    tracy-client/Cargo.toml

# Make a commit that we'll PR
NAME=tracy-client-sys-auto-update[bot]
MAIL="GitHub <noreply@github.com>"
git add tracy-client-sys tracy-client/Cargo.toml README.mkd
git -c user.name="$NAME" -c user.email="$MAIL" commit -m "Update Tracy client bindings to $TAG"
