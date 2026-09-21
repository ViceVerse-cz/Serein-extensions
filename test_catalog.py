"""Run with python test_catalog.py; no external test dependencies."""
import json

import catalog

theme = (catalog.ROOT / "themes/ocean.serein-extension").read_bytes()
plugin = (catalog.ROOT / "plugins/packages/message-delete-protector.serein-extension").read_bytes()
assert catalog.validate(theme)["id"] == "serein-ocean"
assert catalog.validate(plugin)["kind"] == "plugin"
for data, mutate in (
    (theme, lambda p: p["manifest"].update(id="../escape")),
    (theme, lambda p: p["manifest"].update(capabilities=["composer"])),
    (theme, lambda p: p.update(cover_image=[256])),
    (plugin, lambda p: p["manifest"].update(source="https://user:secret@example.com")),
    (plugin, lambda p: p.update(wasm=[0, 1])),
    (plugin, lambda p: p["manifest"].update(capabilities=["unknown"])),
    (plugin, lambda p: p["manifest"]["actions"][0].update(surface="composer")),
):
    invalid = json.loads(data)
    mutate(invalid)
    try:
        catalog.validate(json.dumps(invalid).encode())
    except ValueError:
        pass
    else:
        raise AssertionError("invalid extension accepted")
print("Theme/plugin identity, byte, capability, action and URL checks passed.")
