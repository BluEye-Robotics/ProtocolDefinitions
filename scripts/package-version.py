#!/usr/bin/env python3
"""Calculate one version for NuGet, Maven, and the downstream Swift release."""

import os
from pathlib import Path
import re


def package_version(base: str, env: dict[str, str]) -> str:
    if not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", base):
        raise ValueError("version.txt must contain a major.minor version, for example 6.5")

    run = env.get("GITHUB_RUN_NUMBER")
    if not run:
        return f"{base}.0-local"
    if not re.fullmatch(r"[1-9][0-9]*", run):
        raise ValueError("GITHUB_RUN_NUMBER must be a positive integer")

    version = f"{base}.{run}"
    if env.get("GITHUB_REF") == "refs/heads/master" and not env.get("GITHUB_HEAD_REF"):
        return version

    ref = env.get("GITHUB_HEAD_REF") or env.get("GITHUB_REF_NAME", "dev")
    branch = re.sub(r"[^a-z0-9]+", "-", ref.lower()).strip("-")[:63].rstrip("-") or "dev"
    # A fixed prefix also makes numeric branch names valid SemVer prereleases.
    return f"{version}-branch-{branch}"


if __name__ == "__main__":
    version_file = Path(__file__).resolve().parents[1] / "version.txt"
    try:
        print(package_version(version_file.read_text().strip(), dict(os.environ)))
    except ValueError as error:
        raise SystemExit(str(error))
