"""Agent panel regression using a fake local Codex, never the user's agent or library.

Build with --release --features devtools (on CLT-only Macs also gpui_platform/runtime_shaders).
Run python3 tools/harness/agent_panel.py (needs Pillow). Artifacts stay in a temporary directory.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
from PIL import Image, ImageChops

here = Path(__file__).resolve().parent
repo = here.parent.parent
work = Path(tempfile.mkdtemp(prefix="quiet-agent-panel-")).resolve()
library = work / "library"
shutil.copytree(here / "lib_clean", library)
(library / "preferences.json").write_text(json.dumps({
    "language": "en", "english": True, "appearance": "light", "opaque_window": True,
    "auto_update": False, "quick_hotkey": False, "seen_version": tomllib.loads((repo / "Cargo.toml").read_text())["package"]["version"],
}))
project = work / "project with spaces and a deliberately long directory name for layout verification"
project.mkdir()
other = work / "other-project"
other.mkdir()
bin_dir = work / "bin"
bin_dir.mkdir()
fake = bin_dir / "codex"
fake.write_text(f"#!{sys.executable}\n" + r'''
import json, os, sys, time
from pathlib import Path
log = Path(os.environ["AGENT_FIXTURE_LOG"])
def record(v):
    with log.open("a") as f: f.write(json.dumps(v) + "\n")
def emit(v):
    print(json.dumps(v), flush=True)
def finish(text, delta=None):
    emit({"method":"item/agentMessage/delta","params":{"itemId":str(turn),"delta":text if delta is None else delta}})
    emit({"method":"item/completed","params":{"item":{"type":"agentMessage","id":str(turn),"text":text}}})
    emit({"method":"turn/completed","params":{"turn":{"id":str(turn),"status":"completed"}}})
record({"started":os.getcwd(),"pid":os.getpid()})
turn = 0
for line in sys.stdin:
    v = json.loads(line)
    record(v)
    method, p = v.get("method"), v.get("params", {})
    if method == "initialize": emit({"id":v["id"],"result":{}})
    elif method == "thread/start":
        assert Path(p["cwd"]).resolve() == Path.cwd()
        assert p["approvalPolicy"] == "on-request"
        emit({"id":v["id"],"result":{"thread":{"id":"fixture"},"model":"fixture-default"}})
    elif method == "model/list":
        if os.environ.get("AGENT_FIXTURE_MODE") == "error":
            emit({"id":v["id"],"error":{"code":-32000,"message":"Fixture catalog unavailable"}})
        elif p.get("cursor") == "second":
            emit({"id":v["id"],"result":{"data":[{"model":"fixture-b","displayName":"Fixture B"}]}})
        else:
            emit({"id":v["id"],"result":{"data":[{"id":"catalog-a","model":"fixture-a","displayName":"Fixture A"}],"nextCursor":"second"}})
    elif method == "turn/start":
        turn += 1
        emit({"id":v["id"],"result":{"turn":{"id":str(turn)}}})
        emit({"method":"turn/started","params":{"turn":{"id":str(turn)}}})
        prompt = p["input"][0]["text"]
        try:
            context = json.loads(prompt)
            if isinstance(context, dict) and "referenced_documents" in context:
                prompt = context["user_message"]
        except json.JSONDecodeError:
            pass
        if prompt == "**markdown-demo**":
            finish("## Rendered heading\n\n**Bold** *italic* ~~strike~~ `inline` [Docs](https://example.com)\n\n- first\n  - nested\n\n> Quoted reply\n\n---")
        elif prompt == "markdown-code":
            partial = "```rust\nlet message = \"A deliberately long code line for horizontal scrolling in a narrow chat panel\";\n"
            emit({"method":"item/agentMessage/delta","params":{"itemId":str(turn),"delta":partial}})
            time.sleep(1.5)
            suffix = "```\n\n| Name | Value | State | Description |\n| --- | --- | --- | --- |\n| 中文 | **ready** | open | details |"
            finish(partial + suffix, delta=suffix)
        elif prompt == "approval":
            emit({"id":"approve","method":"item/commandExecution/requestApproval","params":{
                "command":"echo fixture", "reason":"Fixture approval", "cwd":os.getcwd()}})
        elif prompt != "slow": finish("Fixture response " + str(turn))
    elif v.get("id") == "approve":
        assert v["result"]["decision"] in ("accept", "decline")
        finish(v["result"]["decision"] + " fixture")
    elif method == "turn/interrupt":
        emit({"id":v["id"],"result":{}})
        emit({"method":"turn/completed","params":{"turn":{"id":str(turn),"status":"interrupted"}}})
''')
fake.chmod(0o755)

def run(name, steps, fixture_mode="normal", lib=library):
    folder = work / name
    folder.mkdir()
    script = folder / "script.txt"
    script.write_text(f"resize 1200 800\nwait 600\nmove 600 400\n{steps}\nquit\n")
    binary = Path(os.environ.get("FDL_BIN", repo / "target/release/quiet_md"))
    assert b"devtools: cannot read" in binary.read_bytes(), "Build with --features devtools"
    env = dict(os.environ, QUIET_MD_HOME=str(lib), FDL_SCRIPT=str(script), FDL_SHOTS=str(folder),
               FDL_NOBLINK="1", PATH=str(bin_dir) + os.pathsep + os.defpath,
               AGENT_FIXTURE_LOG=str(work / "protocol.jsonl"), AGENT_FIXTURE_MODE=fixture_mode)
    with (folder / "log").open("w") as log:
        result = subprocess.run([str(binary)], env=env, cwd=folder, stdout=log, stderr=log, timeout=40)
    assert result.returncode == 0, (folder / "log").read_text()
    assert (folder / "final.png").exists(), f"No screenshot; see {folder}"

run("conversation", f"""
shot collapsed
click 1158 757
wait 100
click 900 649
type hello
key enter
wait 100
shot no-directory
pick {project}
click 813 683
wait 150
click 900 649
type hello
key enter
wait 500
shot first-response
click 1038 683
wait 1500
shot models
click 1000 641
wait 100
click 900 649
type selected-model
key enter
wait 300
click 1038 683
wait 1500
click 1000 577
wait 100
click 900 649
type restored-model
key enter
wait 300
move 813 683
wait 700
shot directory-tooltip
click 1152 172
wait 100
shot folded
click 1158 757
wait 100
shot resumed
click 900 649
type approval
key enter
wait 400
shot approval
click 900 524
wait 400
shot approved
click 900 649
type approval
key enter
wait 300
click 811 524
wait 300
shot declined
click 900 649
type slow
key enter
wait 300
click 1038 683
wait 1500
shot model-disabled
click 1137 683
wait 300
shot stopped
click 1116 172
wait 100
click 900 649
type new-session
key enter
wait 400
pick {other}
click 813 683
wait 100
click 900 649
type changed-directory
key enter
wait 400
shot final
""")
records = [json.loads(line) for line in (work / "protocol.jsonl").read_text().splitlines()]
starts = [r["started"] for r in records if "started" in r]
assert starts == [str(project), str(project), str(other)], starts
turns = [r for r in records if r.get("method") == "turn/start"]
assert len(turns) == 8, records
assert [r["params"]["model"] for r in turns[:3]] == ["fixture-default", "fixture-b", "fixture-default"], turns[:3]
assert len([r for r in records if r.get("method") == "thread/start"]) == 3
assert len([r for r in records if r.get("method") == "model/list"]) == 4
assert any(r.get("id") == "approve" and r.get("result", {}).get("decision") == "accept" for r in records)
assert any(r.get("id") == "approve" and r.get("result", {}).get("decision") == "decline" for r in records)
assert any(r.get("method") == "turn/interrupt" for r in records)
settings = json.loads((library / ".quiet/settings.json").read_text())
assert settings["agent_directory"] == str(other)
(work / "empty-clipboard.txt").write_text("")
run("markdown", f"""
click 1158 757
wait 100
click 900 649
type **markdown-demo**
key enter
wait 500
move 600 400
shot inline
click 1116 172
wait 100
click 900 649
type markdown-code
key enter
wait 500
shot streaming-code
wait 1700
shot code-table
wheel 1000 400 0 200
wait 100
shot wide-table
clip text {work / 'empty-clipboard.txt'}
click 1126 297
wait 100
click 900 649
key cmd-v
key enter
wait 300
shot copied-code
wheel 1000 340 0 200
wait 100
shot final
""")
def changed_region(before, after, box):
    a = Image.open(work / "markdown" / f"{before}.png").convert("RGB")
    b = Image.open(work / "markdown" / f"{after}.png").convert("RGB")
    scale = a.width / 1200
    crop = tuple(round(v * scale) for v in box)
    return ImageChops.difference(a.crop(crop), b.crop(crop)).getbbox() is not None
assert changed_region("code-table", "wide-table", (800, 360, 1170, 445)), "Wide table did not scroll"
assert changed_region("copied-code", "final", (814, 325, 1164, 355)), "Code block did not scroll"
for name in ["collapsed", "folded"]:
    image = Image.open(work / "conversation" / f"{name}.png").convert("RGB")
    x, y = round(1199 * image.width / 1200), round(200 * image.height / 800)
    assert min(image.getpixel((x, y))) >= 250, f"Collapsed floating card still covers the note: {name}"
run("small-window", """
resize 500 340
wait 150
click 458 298
wait 150
shot small
click 452 92
wait 150
shot final
""")
small_img = Image.open(work / "small-window" / "small.png").convert("RGB")
small_closed = Image.open(work / "small-window" / "final.png").convert("RGB")
assert small_img.getpixel((round(452 * small_img.width / 500), round(92 * small_img.height / 340))) != small_closed.getpixel((round(452 * small_closed.width / 500), round(92 * small_closed.height / 340))), "Small-window close button was clipped or did not collapse card"
library_dark = work / "library-dark"
shutil.copytree(here / "lib_clean", library_dark)
(library_dark / "preferences.json").write_text(json.dumps({
    "language": "en", "english": True, "appearance": "dark", "opaque_window": True,
    "auto_update": False, "quick_hotkey": False, "seen_version": tomllib.loads((repo / "Cargo.toml").read_text())["package"]["version"],
}))
run("dark-mode", f"""
click 1158 757
wait 150
pick {other}
click 813 683
wait 150
click 900 649
type dark-hello
key enter
wait 500
shot final
""", lib=library_dark)
dark_img = Image.open(work / "dark-mode" / "final.png").convert("RGB")
assert max(dark_img.getpixel((round(900 * dark_img.width / 1200), round(400 * dark_img.height / 800)))) < 80, "Dark mode card background is not dark"
run("focus-mode", """
click 1158 757
wait 150
key cmd-.
wait 250
shot hidden
key cmd-.
wait 250
shot final
""")
focus_hidden = Image.open(work / "focus-mode" / "hidden.png").convert("RGB")
focus_restored = Image.open(work / "focus-mode" / "final.png").convert("RGB")
assert min(focus_hidden.getpixel((round(1158 * focus_hidden.width / 1200), round(757 * focus_hidden.height / 800)))) >= 250, "Agent avatar remained visible in focus mode"
assert focus_hidden.getpixel((round(1158 * focus_hidden.width / 1200), round(757 * focus_hidden.height / 800))) != focus_restored.getpixel((round(1158 * focus_restored.width / 1200), round(757 * focus_restored.height / 800))), "Agent avatar did not restore after exiting focus mode"
run("reopen", "click 1158 757\nwait 100\nclick 900 649\ntype reopened\nkey enter\nwait 400\nshot final")
run("home-header", "key cmd-alt-home\nwait 200\nclick 1158 757\nwait 100\nclick 1152 172\nwait 100\nshot final")
run("model-error", "click 1158 757\nwait 100\nclick 1038 683\nwait 1500\nshot model-error\nkey escape\nwait 100\nclick 900 649\ntype after-model-error\nkey enter\nwait 400\nshot final", fixture_mode="error")
run("document-references", """
check-agent-references setup
wait 100
click 1158 757
wait 100
check-agent-references check-a
shot reference-added
click 1152 172
wait 100
click 1158 757
wait 100
check-agent-references check-a
check-agent-references edit-a
check-agent-references preview-a
click 1152 172
wait 100
click 1158 757
wait 100
click 900 649
type live-reference
key enter
wait 300
check-agent-references exit-preview
check-agent-references open-b
wait 150
click 900 649
type pinned-reference
key enter
wait 300
click 1152 172
wait 100
click 1158 757
wait 100
check-agent-references check-both
shot reference-two
click 900 649
type two-references
key enter
wait 300
check-agent-references reopen-a
wait 150
click 900 649
type reopened-reference
key enter
wait 300
check-agent-references peer
wait 150
click 900 649
type peer-reference
key enter
wait 300
check-agent-references close-peer
wait 100
click 1152 600
wait 100
click 1152 600
wait 100
check-agent-references check-empty
click 900 649
type without-reference
key enter
wait 300
click 1152 172
wait 100
check-agent-references home
wait 100
click 1158 757
wait 100
check-agent-references check-empty
shot reference-home
click 1152 172
wait 100
check-agent-references settings
wait 100
click 1158 757
wait 100
check-agent-references check-empty
check-agent-references missing
wait 100
click 900 649
type missing-reference
key enter
wait 100
check-agent-references check-missing
shot final
""")
records = [json.loads(line) for line in (work / "protocol.jsonl").read_text().splitlines()]
assert [r["started"] for r in records if "started" in r][-1] == str(other)
assert any(r.get("method") == "turn/start" and "after-model-error" in r["params"]["input"][0]["text"] for r in records)
contexts = {}
plain = []
for record in records:
    if record.get("method") != "turn/start":
        continue
    text = record["params"]["input"][0]["text"]
    try:
        context = json.loads(text)
    except json.JSONDecodeError:
        plain.append(text)
        continue
    contexts[context["user_message"]] = context["referenced_documents"]
assert len(contexts["live-reference"]) == 1
assert contexts["live-reference"][0]["title"] == "Reference A"
assert "UPDATEDA" in contexts["live-reference"][0]["markdown"]
assert contexts["pinned-reference"] == contexts["live-reference"]
assert [doc["title"] for doc in contexts["two-references"]] == ["Reference A", "Reference B"]
assert "UNSAVEDB" in contexts["two-references"][1]["markdown"]
assert "REOPENEDA" in contexts["reopened-reference"][0]["markdown"]
assert "PEERA" in contexts["peer-reference"][0]["markdown"]
assert "without-reference" in plain
assert any("A deliberately long code line" in text for text in [*plain, *contexts]), "Code copy/paste lost the original code"
assert "missing-reference" not in plain and "missing-reference" not in contexts
for path in library.rglob("*.xml"):
    assert not any(marker in path.read_text() for marker in ["selected-model", "restored-model", "after-model-error"]), f"Agent input edited a note: {path}"
for record in records:
    if "pid" not in record:
        continue
    try:
        os.kill(record["pid"], 0)
    except ProcessLookupError:
        continue
    raise AssertionError(f"Agent process survived application exit: {record['pid']}")
print(f"Agent panel: conversation, fold, approval, stop, new session, directory change and persistence passed. Artifacts: {work}")
