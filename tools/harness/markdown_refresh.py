"""Markdown refresh while its window is borrowed, without activating a window.

Build with --release --features gpui_platform/runtime_shaders,devtools on CLT-only Macs.
Uses only temporary documents; never accesses the user's library or clipboard.
"""
import os
from pathlib import Path
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
work = Path(tempfile.mkdtemp(prefix="quiet-markdown-refresh-"))
script = work / "script.txt"
script.write_text("wait 300\ncheck-markdown-refresh\nquit\n")
binary = Path(os.environ.get("FDL_BIN", repo / "target/release/quiet_md"))
env = dict(os.environ, QUIET_MD_HOME=str(work / "library"), QUIET_MD_LOG_DIR=str(work / "logs"), FDL_SCRIPT=str(script), FDL_SHOTS=str(work))
with (work / "log").open("w") as log:
    result = subprocess.run([str(binary)], env=env, cwd=repo, stdout=log, stderr=subprocess.STDOUT, timeout=60)
output = (work / "log").read_text()
assert result.returncode == 0 and "markdown-refresh: PASS" in output, f"{work}\n{output}"
assert "PANIC" not in output, output
print(f"markdown_refresh: passed; artifacts: {work}")
