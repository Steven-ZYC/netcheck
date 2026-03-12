#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_root="$(cd "${script_dir}/.." && pwd)"
dist_dir="${project_root}/dist"

mkdir -p "${dist_dir}"

if [[ "$#" -eq 0 ]]; then
    echo "usage: $0 <asset> [<asset> ...]" >&2
    exit 1
fi

for asset in "$@"; do
    if [[ ! -f "${asset}" ]]; then
        echo "missing asset: ${asset}" >&2
        exit 1
    fi

    filename="$(basename "${asset}")"
    cp "${asset}" "${dist_dir}/${filename}"
    (
        cd "${dist_dir}"
        sha256sum "${filename}" > "${filename}.sha256"
    )
done
