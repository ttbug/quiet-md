"""Fractions keep their bar wherever they sit (the report of 2026-11-28: `\\frac{N}{2}` drew no bar in
the dark look, `\\frac{n}{2}` did). The bar is 0.04em, under a pixel; it now lies on whole device
pixels, so the numerator's height (lower or upper case, a superscript, a root, a fraction) can no
longer move it off every pixel's centre.

A scratch page holds rows of fractions, each row a little taller or shorter than the last so the
bars fall at every part of a pixel; in both looks every bar must be drawn. All but the bars is red
(`\\color{red}`), so the bars are the page's only pieces of ink in the text's colour, counted one by one. Reading and editing draw an
equation the same way (`math::Math::paint`); the `edit` case also clicks into the page.

Build with --release --features devtools, then run this file. FDL_BIN selects another build;
MATH_FRAC_SHOTS a folder to copy the screenshots to. Requires Pillow.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from PIL import Image

here = Path(__file__).resolve().parent
work = Path(tempfile.mkdtemp(prefix="quiet-math-frac-"))

# numerator and denominator (and what comes first) in red: a bar keeps the text's own colour, so it is
# the only ink of that colour on the page and each one is counted on its own
R = lambda t: r"{\color{red}" + t + "}"
FRACS = [(n, d) for n, d in [("n", "2"), ("N", "2"), ("N^2", "2"), (r"\sqrt{N}", "2"), ("A_1", "B"), ("x^{2}", "y")]]
# what comes first in each row changes its height by part of a pixel
LEADS = ["", "x", "x^2", "x_1", r"\sqrt{x}", "x^{2^2}", r"\hat{x}", r"\int", r"\tilde{x}", "x'"]
ROWS = [[(R(lead) if lead else "") + rf"\frac{{{R(n)}}}{{{R(d)}}}" for n, d in FRACS] for lead in LEADS]


def library(name, look):
    lib = work / f"source-{name}"
    (lib / "docs").mkdir(parents=True)
    doc_id = "aaaaaaaa-0000-0000-0000-000000000002"
    (lib / "index.json").write_text(json.dumps({doc_id: {"created": 1, "modified": 1, "opened": 1}}))
    (lib / "preferences.json").write_text(json.dumps({"appearance": look, "english": False, "seen_version": "9.9.9", "auto_update": False, "quick_hotkey": False}))
    body = "".join("<p>" + "　".join(f"<latex>{f}</latex>" for f in row) + "</p>\n" for row in ROWS)
    (lib / "docs" / f"{doc_id}.xml").write_text(f"<title>分数线</title>\n{body}")
    return lib


def run(name, look, extra=""):
    folder = work / name
    script = work / f"{name}.txt"
    script.write_text(f"resize 1100 900\nwait 1500\nmove 1090 890\nrow 分数线\nwait 1500\nmove 1090 890\n{extra}shot page\n")
    subprocess.run([sys.executable, str(here / "run.py"), str(script), str(library(name, look)), str(folder)], env=dict(os.environ, FDL_NOBLINK="1"), check=True, timeout=120)
    return folder / "page.png"


def bars(png):
    """The page's pieces of ink in the text's colour (grey, not red), each a bar: wider than tall."""
    im = Image.open(png).convert("RGB")
    w, h = im.size
    px = im.load()
    bg = sum(px[w - 40, h - 40]) / 3
    def plain(x, y):
        r, g, b = px[x, y]
        return abs(r - g) < 24 and abs(g - b) < 24 and abs((r + g + b) / 3 - bg) > 50
    seen, found = set(), []
    for y in range(150, h):
        for x in range(300, w):
            if (x, y) in seen or not plain(x, y): continue
            stack, xs, ys = [(x, y)], [], []
            seen.add((x, y))
            while stack:
                cx, cy = stack.pop()
                xs.append(cx); ys.append(cy)
                for nx in (cx - 1, cx, cx + 1):
                    for ny in (cy - 1, cy, cy + 1):
                        if 300 <= nx < w and 150 <= ny < h and (nx, ny) not in seen and plain(nx, ny):
                            seen.add((nx, ny)); stack.append((nx, ny))
            bw, bh = max(xs) - min(xs) + 1, max(ys) - min(ys) + 1
            # the caret is the one other plain mark: tall and thin
            if bw >= 6 and bh <= 3: found.append((min(xs), min(ys), bw, bh))
    return found


expected = sum(len(r) for r in ROWS)
cases = sys.argv[1:] or ["light", "dark", "edit"]
failed = []
found = {}
for case in cases:
    look = "light" if case == "light" else "dark"
    # edit: the first row's last fraction clicked, its source open in the box under it
    png = run(case, look, "click 596 195\nwait 600\nmove 1090 890\n" if case == "edit" else "")
    found[case] = got = bars(png)
    print(f"{case}: {len(got)} bars of {expected}")
    if case != "edit" and len(got) != expected: failed.append(case)
if "edit" in found:
    # each bar of the page read as it is drawn while one equation is edited (that one's under the
    # selection's tint): all but those under the source box (at 582–984 × 224–341 in this window)
    plain = found.get("dark") or bars(run("dark", "dark"))
    keep = [b for b in plain if not (570 <= b[0] <= 990 and 215 <= b[1] <= 350)]
    lost = [b for b in keep if not any(abs(b[0] - e[0]) <= 1 and abs(b[1] - e[1]) <= 1 for e in found["edit"])]
    print(f"edit: {len(keep) - len(lost)} of the {len(keep)} bars outside the source box")
    if lost or not any(b[1] < 215 and b[0] > 570 for b in found["edit"]): failed.append("edit")

shots = os.environ.get("MATH_FRAC_SHOTS")
if shots:
    Path(shots).mkdir(parents=True, exist_ok=True)
    for case in cases:
        shutil.copy(work / case / "page.png", Path(shots) / f"{case}.png")
assert not failed, f"fractions without their bar in {failed}: {work}"
print("math fractions ok:", work)
