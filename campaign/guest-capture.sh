#!/bin/sh
# Source inside the disposable guest only. Capture frames contain no expected verdict.
lab_emit() {
    local case_id=$1 kind=$2 file=$3 bytes hash
    bytes=$(wc -c <"$file") || return 125
    bytes=$((bytes + 0))
    hash=$(sha256sum "$file") || return 125
    hash=${hash%% *}
    printf '\nALIBI_BLOB_BEGIN %s %s %s %s %s\n' "$RUN_ID" "$case_id" "$kind" "$bytes" "$hash"
    /bin/busybox base64 "$file"
    printf 'ALIBI_BLOB_END %s %s %s\n' "$RUN_ID" "$case_id" "$kind"
}

lab_run() {
    local case_id=$1 directory started ended rc
    shift
    case "$case_id" in ''|*[!A-Za-z0-9_-]*) echo INVALID_CAPTURE_CASE_ID; return 125;; esac
    directory=/run/alibi-captures/$case_id
    mkdir -p /run/alibi-captures || return 125
    mkdir "$directory" || return 125
    printf '%s\0' "$@" >"$directory/argv" || return 125
    started=$(cut -d' ' -f1 /proc/uptime) || return 125
    if /bin/time -v -o "$directory/resource" /bin/timeout -k 2 30 "$@" >"$directory/stdout" 2>"$directory/stderr"; then
        rc=0
    else
        rc=$?
    fi
    ended=$(cut -d' ' -f1 /proc/uptime) || return 125
    printf '{"case_id":"%s","exit_status":%s,"started_uptime_seconds":%s,"ended_uptime_seconds":%s,"timeout_seconds":30,"kill_after_seconds":2}\n' "$case_id" "$rc" "$started" "$ended" >"$directory/metadata"
    lab_emit "$case_id" argv "$directory/argv" || return 125
    lab_emit "$case_id" stdout "$directory/stdout" || return 125
    lab_emit "$case_id" stderr "$directory/stderr" || return 125
    lab_emit "$case_id" resource "$directory/resource" || return 125
    lab_emit "$case_id" metadata "$directory/metadata" || return 125
    # Alibi exit 1/2 is a result, not a shell control-flow failure.
    return 0
}
