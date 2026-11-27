#!/usr/bin/env bash
# Records each test's trace with `tracy-capture` and prints it with `tracy-csvexport`:
#
#     TRACY_TOOLS_DIR=/path/to/tracy/bin cargo nextest run
#
# The tools must match the Tracy version vendored in `tracy-client-sys`.
set -u

if [ -z "${TRACY_TOOLS_DIR:-}" ]; then
    exec "$@"
fi
capture="$TRACY_TOOLS_DIR/tracy-capture"
csvexport="$TRACY_TOOLS_DIR/tracy-csvexport"
for tool in "$capture" "$csvexport"; do
    if [ ! -x "$tool" ]; then
        echo "tracy-capture-wrapper: \`$tool\` not found (TRACY_TOOLS_DIR=$TRACY_TOOLS_DIR)" >&2
        exit 1
    fi
done

sanitize() { printf '%s' "$1" | tr -c 'A-Za-z0-9_.-' '_'; }
dir="${TRACY_CAPTURE_DIR:-${NEXTEST_WORKSPACE_ROOT:-.}/target/tracy-captures}/$(sanitize "${NEXTEST_BINARY_ID:-unknown}")"
out="$dir/$(sanitize "${NEXTEST_TEST_NAME:-unknown}").tracy"
port=$(( ${TRACY_CAPTURE_BASE_PORT:-21000} + ${NEXTEST_TEST_GLOBAL_SLOT:-0} ))
mkdir -p "$dir"
rm -f "$out"

TRACY_PORT=$port TRACY_NO_EXIT=1 "$@" &
test_pid=$!
"$capture" -o "$out" -a 127.0.0.1 -p "$port" -f > "$out.log" 2>&1 &
capture_pid=$!

alive() { kill -0 "$1" 2> /dev/null; }
fail() {
    [ "${GITHUB_ACTIONS:-}" = true ] && echo "::endgroup::"
    echo "tracy-capture-wrapper: $1" >&2
    [ -e "$out.log" ] && cat "$out.log" >&2
    exit 1
}

while alive "$test_pid"; do
    if ! alive "$capture_pid" && [ ! -e "$out" ]; then
        kill -9 "$test_pid" 2> /dev/null
        wait "$test_pid" 2> /dev/null
        fail "tracy-capture failed"
    fi
    sleep 0.1
done
wait "$test_pid"
test_status=$?

deadline=$(( SECONDS + ${TRACY_CAPTURE_GRACE:-2} ))
while alive "$capture_pid" && [ "$SECONDS" -lt "$deadline" ]; do
    sleep 0.1
done
if alive "$capture_pid"; then
    kill "$capture_pid" 2> /dev/null
    wait "$capture_pid" 2> /dev/null
    echo "tracy-capture-wrapper: nothing was captured" >&2
    exit "$test_status"
fi
[ -e "$out" ] || fail "tracy-capture exited without saving the trace"

if [ "${GITHUB_ACTIONS:-}" = true ]; then
    echo "::group::Tracy trace: ${NEXTEST_BINARY_ID:-} ${NEXTEST_TEST_NAME:-}"
fi
echo "tracy-capture-wrapper: trace saved to $out"
for section in "zones:" "messages:-m" "frames:-F" "gpu zones:-g"; do
    echo "--- ${section%%:*}"
    "$csvexport" ${section#*:} "$out" || fail "\`tracy-csvexport ${section#*:}\` could not read the trace"
done
if [ "${GITHUB_ACTIONS:-}" = true ]; then
    echo "::endgroup::"
fi
exit "$test_status"
