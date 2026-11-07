#!/usr/bin/env python3
"""Unit tests for bd_theme_to_serein converter."""
import json
import sys
import unittest
from pathlib import Path

# Add project root to sys.path to import catalog validate
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "tools"))

from bd_theme_to_serein import convert_bd_theme, color_to_hex, sanitize_id, parse_bd_metadata, parse_css_variables
from catalog import validate


SAMPLE_BD_CSS = """/**
 * @name ClearVision V6
 * @author ClearVision Team
 * @version 6.5.1
 * @description Highly customizable and beautiful theme for Discord.
 * @source https://github.com/ClearVision/ClearVision-v6
 * @license MIT
 */

:root {
  --background-primary: #121722;
  --background-secondary: #0d1118;
  --background-tertiary: rgb(8, 10, 15);
  --background-floating: #151a24;
  --background-modifier-hover: rgba(255, 255, 255, 0.05);
  --text-normal: #c5cbd6;
  --header-primary: #ffffff;
  --text-muted: #6b778d;
  --text-link: #00aff4;
  --accentcolor: #00aff4;
}
"""

SAMPLE_MINIMAL_CSS = """/**
 * @name SynthWave84
 */
:root {
  --accent: #ff7edb;
}
"""

SAMPLE_COMPLEX_CSS = """/**
 * @name Complex Theme
 */
:root {
  --my-accent: #7aa2f7 !important;
  --brand-experiment: var(--my-accent);
  --bg-dark: #1f2335;
  --background-primary: var(--bg-dark);
}
"""


class TestBDThemeConverter(unittest.TestCase):
    def test_sanitize_id(self):
        self.assertEqual(sanitize_id("ClearVision V6"), "clearvision-v6-theme")
        self.assertEqual(sanitize_id("Nord Dark"), "nord-dark-theme")
        self.assertEqual(sanitize_id("Already-Theme"), "already-theme")

    def test_color_to_hex(self):
        self.assertEqual(color_to_hex("#abc"), "#AABBCC")
        self.assertEqual(color_to_hex("#121722"), "#121722")
        self.assertEqual(color_to_hex("rgb(8, 10, 15)"), "#080A0F")
        self.assertEqual(color_to_hex("#121722 !important"), "#121722")
        rgba_res = color_to_hex("rgba(255, 255, 255, 0.1)", base_bg=(0, 0, 0))
        self.assertEqual(rgba_res, "#191919")
        self.assertIsNone(color_to_hex("invalid-color"))

    def test_metadata_parsing(self):
        meta = parse_bd_metadata(SAMPLE_BD_CSS)
        self.assertEqual(meta.get("name"), "ClearVision V6")
        self.assertEqual(meta.get("author"), "ClearVision Team")
        self.assertEqual(meta.get("version"), "6.5.1")
        self.assertEqual(meta.get("license"), "MIT")

    def test_convert_full_theme(self):
        pkg = convert_bd_theme(SAMPLE_BD_CSS)
        self.assertEqual(pkg["manifest"]["name"], "ClearVision V6")
        self.assertEqual(pkg["manifest"]["kind"], "theme")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["base"], "#121722")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["sidebar"], "#0D1118")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["accent"], "#00AFF4")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["text_strong"], "#FFFFFF")

        # Validate with catalog.py
        data = json.dumps(pkg, separators=(",", ":")).encode("utf-8")
        manifest = validate(data)
        self.assertEqual(manifest["id"], "clearvision-v6-theme")

    def test_convert_minimal_theme(self):
        pkg = convert_bd_theme(SAMPLE_MINIMAL_CSS)
        self.assertEqual(pkg["manifest"]["name"], "SynthWave84")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["accent"], "#FF7EDB")
        data = json.dumps(pkg, separators=(",", ":")).encode("utf-8")
        manifest = validate(data)
        self.assertEqual(manifest["id"], "synthwave84-theme")

    def test_important_and_var_indirection(self):
        vars_parsed = parse_css_variables(SAMPLE_COMPLEX_CSS)
        self.assertEqual(vars_parsed["my-accent"], "#7aa2f7")
        self.assertEqual(vars_parsed["brand-experiment"], "#7aa2f7")
        self.assertEqual(vars_parsed["background-primary"], "#1f2335")

        pkg = convert_bd_theme(SAMPLE_COMPLEX_CSS)
        self.assertEqual(pkg["theme"]["dark"]["colors"]["accent"], "#7AA2F7")
        self.assertEqual(pkg["theme"]["dark"]["colors"]["base"], "#1F2335")


if __name__ == "__main__":
    unittest.main()
