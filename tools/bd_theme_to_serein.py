#!/usr/bin/env python3
"""Convert BetterDiscord .theme.css themes to Serein declarative theme packages (.serein-extension).

Parses BetterDiscord CSS metadata headers (@name, @author, @version, @description, @source)
and CSS custom properties (--background-*, --text-*, --accent*) and compiles them into a
validated Serein theme package conforming to catalog.py.
"""
import argparse
import json
import os
import re
import sys
from pathlib import Path

DEFAULT_STYLE = {
    "body_size": 15,
    "heading_size": 20,
    "button_size": 14,
    "small_size": 12,
    "monospace_size": 14,
    "item_spacing": [8, 8],
    "button_padding": [12, 6],
    "control_height": 34,
    "widget_radius": 6,
    "window_radius": 10,
    "menu_radius": 8,
}

DARK_DEFAULTS = {
    "base": "#1E222A",
    "sidebar": "#191D24",
    "chat": "#242933",
    "raised": "#2E3440",
    "hover": "#3B4252",
    "selected": "#434C5E",
    "border": "#3B4252",
    "text_strong": "#ECEFF4",
    "text": "#D8DEE9",
    "muted": "#7B88A1",
    "accent": "#88C0D0",
    "accent_text": "#1E222A",
    "link": "#81A1C1",
    "positive": "#A3BE8C",
    "warning": "#EBCB8B",
    "danger": "#BF616A",
    "mention_bg": "#263A48",
    "mention_text": "#88C0D0",
}

LIGHT_DEFAULTS = {
    "base": "#E5E9F0",
    "sidebar": "#D8DEE9",
    "chat": "#ECEFF4",
    "raised": "#FFFFFF",
    "hover": "#D8DEE9",
    "selected": "#C2D0E0",
    "border": "#D8DEE9",
    "text_strong": "#2E3440",
    "text": "#3B4252",
    "muted": "#4C566A",
    "accent": "#5E81AC",
    "accent_text": "#FFFFFF",
    "link": "#88C0D0",
    "positive": "#A3BE8C",
    "warning": "#EBCB8B",
    "danger": "#BF616A",
    "mention_bg": "#D0E4EC",
    "mention_text": "#5E81AC",
}


def parse_bd_metadata(css_text: str) -> dict:
    """Extract @key value metadata from BetterDiscord theme header comment."""
    meta = {}
    header_match = re.search(r"/\*\*([\s\S]*?)\*/", css_text)
    if not header_match:
        return meta
    header = header_match.group(1)
    for line in header.splitlines():
        match = re.search(r"@([a-zA-Z0-9_-]+)\s+(.+)", line)
        if match:
            key = match.group(1).lower().strip()
            val = match.group(2).strip()
            meta[key] = val
    return meta


def sanitize_id(name: str) -> str:
    """Convert theme name to valid Serein extension ID."""
    clean = re.sub(r"[^a-zA-Z0-9]+", "-", name.strip().lower()).strip("-")
    if not clean or not clean[0].isalnum():
        clean = f"theme-{clean}".strip("-")
    if len(clean) > 60:
        clean = clean[:60].rstrip("-")
    if not clean.endswith("-theme"):
        if len(clean) <= 54:
            clean = f"{clean}-theme"
    return clean


def color_to_hex(color_val: str, base_bg=(30, 34, 42)) -> str | None:
    """Normalize hex, rgb, or rgba to uppercase 6-digit #RRGGBB."""
    color_val = color_val.strip()
    # #RGB or #RRGGBB
    hex_match = re.fullmatch(r"#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})", color_val)
    if hex_match:
        h = hex_match.group(1)
        if len(h) == 3:
            h = "".join(c * 2 for c in h)
        return f"#{h.upper()}"

    # rgb(r, g, b)
    rgb_match = re.fullmatch(r"rgb\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)", color_val)
    if rgb_match:
        r, g, b = (int(rgb_match.group(i)) for i in (1, 2, 3))
        return f"#{r:02X}{g:02X}{b:02X}"

    # rgba(r, g, b, a) - blend over base background
    rgba_match = re.fullmatch(r"rgba\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*([0-9.]+)\s*\)", color_val)
    if rgba_match:
        r, g, b = (int(rgba_match.group(i)) for i in (1, 2, 3))
        a = float(rgba_match.group(4))
        # Composite over base_bg
        br, bg_g, bb = base_bg
        fr = int(r * a + br * (1.0 - a))
        fg = int(g * a + bg_g * (1.0 - a))
        fb = int(b * a + bb * (1.0 - a))
        return f"#{min(255, max(0, fr)):02X}{min(255, max(0, fg)):02X}{min(255, max(0, fb)):02X}"

    return None


def parse_css_variables(css_text: str) -> dict:
    """Parse all --var-name: value; declarations from CSS."""
    variables = {}
    for match in re.finditer(r"--([a-zA-Z0-9_-]+)\s*:\s*([^;]+);", css_text):
        var_name = match.group(1).lower().strip()
        val = match.group(2).strip()
        variables[var_name] = val
    return variables


def convert_bd_theme(css_text: str, override_id: str = None) -> dict:
    """Convert a BetterDiscord theme CSS string into a Serein theme package dictionary."""
    meta = parse_bd_metadata(css_text)
    name = meta.get("name") or "BetterDiscord Converted Theme"
    pkg_id = override_id or sanitize_id(name)
    author = meta.get("author") or "VenSerein Converter"
    version = meta.get("version") or "1.0.0"
    license_name = meta.get("license") or "MIT"
    source = meta.get("source") or "https://github.com/betterdiscord/betterdiscord"
    if not source.startswith("https://"):
        source = "https://github.com/betterdiscord/betterdiscord"

    vars_dict = parse_css_variables(css_text)

    # Map BD variables
    dark_colors = dict(DARK_DEFAULTS)

    var_mappings = {
        "base": ["background-primary", "background-base", "background"],
        "sidebar": ["background-secondary", "background-tertiary", "sidebar-color"],
        "chat": ["background-primary", "chat-background", "background"],
        "raised": ["background-floating", "background-accent", "card-background"],
        "hover": ["background-modifier-hover", "hover-color"],
        "selected": ["background-modifier-selected", "selected-color"],
        "border": ["border", "background-modifier-accent"],
        "text_strong": ["header-primary", "text-header", "heading-color"],
        "text": ["text-normal", "text-color"],
        "muted": ["text-muted", "muted-color"],
        "accent": ["accentcolor", "brand-experiment", "accent", "blurple"],
        "link": ["text-link", "link-color"],
    }

    for key, candidates in var_mappings.items():
        for cand in candidates:
            if cand in vars_dict:
                hex_c = color_to_hex(vars_dict[cand])
                if hex_c:
                    dark_colors[key] = hex_c
                    break

    # Derived accent and text
    dark_colors["mention_text"] = dark_colors["accent"]
    dark_backdrop = [dark_colors["sidebar"], dark_colors["chat"]]

    # Light colors (use standard light defaults or harmonize with accent)
    light_colors = dict(LIGHT_DEFAULTS)
    light_colors["accent"] = dark_colors["accent"]
    light_colors["mention_text"] = dark_colors["accent"]
    light_backdrop = [light_colors["sidebar"], light_colors["chat"]]

    manifest = {
        "api_version": 1,
        "id": pkg_id,
        "name": name[:128],
        "version": version[:128],
        "author": author[:128],
        "license": license_name[:128],
        "source": source[:2048],
        "kind": "theme",
    }

    theme_data = {
        "dark": {
            "colors": dark_colors,
            "backdrop": dark_backdrop,
        },
        "light": {
            "colors": light_colors,
            "backdrop": light_backdrop,
        },
        "style": DEFAULT_STYLE,
    }

    return {
        "manifest": manifest,
        "theme": theme_data,
    }


def main():
    parser = argparse.ArgumentParser(description="Convert BetterDiscord .theme.css to Serein theme package.")
    parser.add_argument("input_css", type=Path, help="Input .theme.css file")
    parser.add_argument("-o", "--output", type=Path, help="Output .serein-extension file path")
    parser.add_argument("--id", type=str, help="Override package ID")
    parser.add_argument("--stdout", action="store_true", help="Print package JSON to stdout")
    args = parser.parse_args()

    if not args.input_css.exists():
        parser.error(f"Input file not found: {args.input_css}")

    css_content = args.input_css.read_text(encoding="utf-8", errors="replace")
    package = convert_bd_theme(css_content, override_id=args.id)

    pkg_json = json.dumps(package, indent=2, ensure_ascii=False)

    if args.stdout:
        print(pkg_json)
        return

    output_path = args.output
    if not output_path:
        out_dir = Path("themes")
        out_dir.mkdir(parents=True, exist_ok=True)
        output_path = out_dir / f"{package['manifest']['id']}.serein-extension"

    output_path.parent.mkdir(parents=True, exist_ok=True)
    compact_data = json.dumps(package, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    output_path.write_bytes(compact_data)
    print(f"Theme converted successfully: {output_path} ({len(compact_data)} bytes)")


if __name__ == "__main__":
    main()
