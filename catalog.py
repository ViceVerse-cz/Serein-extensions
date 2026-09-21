#!/usr/bin/env python3
"""Generate an immutable extension catalog; canonical package validation lives in Serein."""
import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parent
CAPABILITIES = {"selected_message", "composer", "storage", "deleted_messages", "image_sharing", "appearance"}
RESERVED = {"con", "prn", "aux", "nul"} | {f"{prefix}{n}" for prefix in ("com", "lpt") for n in range(1, 10)}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def valid_id(value):
    return isinstance(value, str) and re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", value) and value not in RESERVED


def validate(data):
    require(0 < len(data) <= 16 * 1024 * 1024, "package exceeds 16 MiB")
    package = json.loads(data)
    manifest = package["manifest"]
    require(manifest["api_version"] == 1 and valid_id(manifest["id"]), "invalid API version or ID")
    for key in ("name", "version", "author", "license"):
        value = manifest[key]
        require(isinstance(value, str) and 0 < len(value.encode()) <= 128 and not any(ord(c) < 32 or 127 <= ord(c) <= 159 for c in value), f"invalid {key}")
    source = urlsplit(manifest["source"])
    require(len(manifest["source"].encode()) <= 2048 and source.scheme == "https" and source.hostname and not source.username and source.password is None, "credential-free HTTPS source required")
    capabilities = manifest.get("capabilities", [])
    actions = manifest.get("actions", [])
    require(isinstance(capabilities, list) and len(capabilities) <= 4 and len(set(capabilities)) == len(capabilities) and set(capabilities) <= CAPABILITIES, "invalid capabilities")
    require(isinstance(actions, list) and len(actions) <= 16 and len({a["id"] for a in actions}) == len(actions), "invalid action list")
    require(sum(a["surface"] == "activation" for a in actions) <= 1, "multiple activation actions")
    for action in actions:
        require(valid_id(action["id"]) and action["surface"] in ("message", "composer", "panel", "activation"), "invalid action")
        label = action["label"]
        require(isinstance(label, str) and 0 < len(label.encode()) <= 128 and not any(ord(c) < 32 or 127 <= ord(c) <= 159 for c in label), "invalid action label")
        capability = {"message": "selected_message", "composer": "composer"}.get(action["surface"])
        require(capability is None or capability in capabilities, "action lacks required capability")
    for field, limit in (("wasm", 4 * 1024 * 1024), ("background_image", 2 * 1024 * 1024), ("cover_image", 2 * 1024 * 1024)):
        values = package.get(field, [])
        require(isinstance(values, list) and len(values) <= limit and all(type(b) is int and 0 <= b <= 255 for b in values), f"invalid {field} bytes")
    if manifest["kind"] == "theme":
        require(isinstance(package.get("theme"), dict) and not package.get("wasm") and not capabilities and not actions, "invalid declarative theme")
    else:
        require(manifest["kind"] == "plugin" and actions and package.get("theme") is None and not package.get("background_image") and not package.get("cover_image"), "invalid plugin")
        require(bytes(package.get("wasm", [])).startswith(b"\0asm\x01\0\0\0"), "Wasm v1 module required")
    return manifest


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], stderr=subprocess.PIPE)


def generate(commit, repository):
    require(re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", commit), "explicit full Git commit SHA required")
    require(git("rev-parse", f"{commit}^{{commit}}").decode().strip() == commit, "commit must exist locally")
    require(re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository), "repository must be owner/name")
    base = f"https://raw.githubusercontent.com/{repository}/{commit}/"
    paths = sorted((ROOT / "themes").glob("*.serein-extension")) + sorted((ROOT / "plugins/packages").glob("*.serein-extension"))
    require(0 < len(paths) <= 256, "catalog requires 1 to 256 packages")
    entries, ids = [], set()
    for path in paths:
        relative = path.relative_to(ROOT).as_posix()
        data = path.read_bytes()
        require(data == git("show", f"{commit}:{relative}"), f"{relative} differs from commit")
        manifest = validate(data)
        require(manifest["id"] not in ids, "duplicate extension ID")
        ids.add(manifest["id"])
        entry = dict(manifest=manifest, release_url=base + relative, sha256=hashlib.sha256(data).hexdigest(), download_bytes=len(data), source_commit=commit)
        if manifest["kind"] == "plugin":
            source_manifest = ROOT / "plugins" / path.stem / "manifest.json"
            require(json.loads(source_manifest.read_bytes()) == manifest, "plugin source manifest differs from package")
        preview = ROOT / "previews" / (path.stem + ".png")
        if preview.is_file():
            image = preview.read_bytes()
            require(0 < len(image) <= 256 * 1024 and image == git("show", f"{commit}:previews/{preview.name}"), "invalid or uncommitted preview")
            entry["preview"] = dict(url=base + "previews/" + preview.name, sha256=hashlib.sha256(image).hexdigest(), download_bytes=len(image))
        entries.append(entry)
    output = (json.dumps(dict(api_version=1, entries=entries), indent=2, ensure_ascii=False) + "\n").encode()
    require(len(output) <= 1024 * 1024, "catalog exceeds 1 MiB")
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--commit", help="full local commit containing current package and preview bytes")
    parser.add_argument("--repository", default="ViceVerse-cz/Serein-extensions")
    parser.add_argument("--check", action="store_true", help="verify without writing; infer catalog commit if omitted")
    args = parser.parse_args()
    catalog = ROOT / "catalog.json"
    commit = args.commit
    if args.check and not commit:
        commits = {entry["source_commit"] for entry in json.loads(catalog.read_bytes())["entries"]}
        require(len(commits) == 1, "catalog must pin one package commit")
        commit = commits.pop()
    require(commit is not None, "--commit is required when generating")
    output = generate(commit, args.repository)
    if args.check:
        require(catalog.read_bytes() == output, "catalog is stale; regenerate after committing packages")
    else:
        catalog.write_bytes(output)
    print(f"Catalog {'verified' if args.check else 'generated'}: {len(json.loads(output)['entries'])} packages at {commit}")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, TypeError, subprocess.CalledProcessError) as error:
        sys.exit(f"Catalog error: {error}")
