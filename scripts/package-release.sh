#!/usr/bin/env bash
set -euo pipefail

version="${1:?tag version is required}"
target="${2:?rust target is required}"
platform="${3:?platform label is required}"

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_root="$(cd "${script_dir}/.." && pwd)"
binary_name="${BINARY_NAME:-netcheck}"
binary_path="${project_root}/target/${target}/release/${binary_name}"
default_binary_path="${project_root}/target/release/${binary_name}"
dist_dir="${project_root}/dist"
bundle_name="${binary_name}-${version}-${platform}"
bundle_dir="${dist_dir}/${bundle_name}"
archive_path="${dist_dir}/${bundle_name}.tar.gz"

if [[ ! -f "${binary_path}" ]]; then
    host_target="$(rustc -vV | awk '/^host:/ { print $2 }')"
    if [[ "${target}" == "${host_target}" && -f "${default_binary_path}" ]]; then
        binary_path="${default_binary_path}"
    else
        echo "missing binary: ${binary_path}" >&2
        exit 1
    fi
fi

rm -rf "${bundle_dir}"
mkdir -p "${bundle_dir}"

install -m 755 "${binary_path}" "${bundle_dir}/${binary_name}"
install -m 644 "${project_root}/README.md" "${bundle_dir}/README.md"
install -m 644 "${project_root}/LICENSE" "${bundle_dir}/LICENSE"

tar -C "${dist_dir}" -czf "${archive_path}" "${bundle_name}"

(
    cd "${dist_dir}"
    sha256sum "${bundle_name}.tar.gz" > "${bundle_name}.tar.gz.sha256"
)

rm -rf "${bundle_dir}"
