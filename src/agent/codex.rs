//! Codex App Server over stdio. Only this module owns the child process.

use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

pub enum Control { Send(String, Option<String>), ListModels, Stop, Decide(Value, bool), Shutdown }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model { pub id: String, pub name: String }
pub enum Event {
    Models(Vec<Model>),
    ModelError(String),
    Delta(String, String),
    Message(String, String),
    Status(String),
    Approval(Value, String),
    Resolved(Value),
    Error(String),
    Done,
    Exited,
}

pub struct Client { pub tx: Sender<Control>, pub rx: Receiver<Event>, worker: Option<std::thread::JoinHandle<()>> }
impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.tx.send(Control::Shutdown);
        if let Some(worker) = self.worker.take() { let _ = worker.join(); }
    }
}

pub fn executable() -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    if let Some(home) = dirs::home_dir() { dirs.extend([home.join(".local/bin"), home.join(".cargo/bin")]); }
    dirs.extend([PathBuf::from("/opt/homebrew/bin"), PathBuf::from("/usr/local/bin")]);
    dirs.into_iter().map(|d| d.join(if cfg!(windows) { "codex.exe" } else { "codex" })).find(|p| p.is_file())
}

struct OwnedChild(Child);
impl Drop for OwnedChild {
    fn drop(&mut self) { let _ = self.0.kill(); let _ = self.0.wait(); }
}

pub fn start(bin: &Path, cwd: &Path) -> std::io::Result<Client> {
    let mut command = Command::new(bin);
    command.args(["app-server", "--listen", "stdio://"]).current_dir(cwd)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; command.creation_flags(0x08000000); }
    let mut child = OwnedChild(command.spawn()?);
    let stdout = child.0.stdout.take().expect("piped stdout");
    let mut stdin = child.0.stdin.take().expect("piped stdin");
    let (tx, controls) = mpsc::channel();
    let (events, rx) = mpsc::channel();
    let (lines_tx, lines) = mpsc::sync_channel(256);
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            if lines_tx.send(line).is_err() { break }
        }
    });
    let cwd = cwd.to_path_buf();
    let worker = std::thread::spawn(move || {
        let mut protocol = Protocol::new(cwd);
        let mut out = vec![];
        protocol.initialize(&mut out);
        loop {
            match controls.recv_timeout(Duration::from_millis(15)) {
                Ok(Control::Shutdown) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Ok(c) => protocol.control(c, &mut out),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            for _ in 0..256 {
                match lines.try_recv() {
                    Ok(Ok(line)) => match serde_json::from_str(&line) {
                        Ok(v) => protocol.receive(v, &mut out, &events),
                        Err(_) => { let _ = events.send(Event::Error("Invalid Codex protocol message".into())); return; }
                    },
                    Ok(Err(e)) => { let _ = events.send(Event::Error(e.to_string())); return; }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => { let _ = events.send(Event::Exited); return; }
                }
            }
            for v in out.drain(..) {
                if writeln!(stdin, "{v}").and_then(|_| stdin.flush()).is_err() {
                    let _ = events.send(Event::Exited);
                    return;
                }
            }
            if child.0.try_wait().ok().flatten().is_some() { let _ = events.send(Event::Exited); break }
        }
    });
    Ok(Client { tx, rx, worker: Some(worker) })
}

struct Protocol {
    cwd: PathBuf,
    ready: bool,
    thread: Option<String>,
    turn: Option<String>,
    prompt: Option<(String, Option<String>)>,
    default_model: Option<String>,
    list_models: bool,
    models: Vec<Model>,
    stopping: bool,
    next: u64,
    requests: HashMap<u64, String>,
    changes: HashMap<String, String>,
}

impl Protocol {
    fn new(cwd: PathBuf) -> Self {
        Self { cwd, ready: false, thread: None, turn: None, prompt: None, default_model: None, list_models: false, models: vec![], stopping: false, next: 0, requests: HashMap::new(), changes: HashMap::new() }
    }

    fn request(&mut self, method: &str, params: Value, out: &mut Vec<Value>) {
        self.next += 1;
        self.requests.insert(self.next, method.into());
        out.push(json!({"id":self.next,"method":method,"params":params}));
    }

    fn initialize(&mut self, out: &mut Vec<Value>) {
        if !self.ready && !self.requests.values().any(|m| m == "initialize") {
            self.request("initialize", json!({"clientInfo":{"name":"quiet_md","version":env!("CARGO_PKG_VERSION")}}), out);
        }
    }

    fn begin(&mut self, out: &mut Vec<Value>) {
        if !self.ready || self.prompt.is_none() { return }
        if let Some(thread) = self.thread.clone() {
            let (text, model) = self.prompt.take().unwrap();
            self.request("turn/start", json!({"threadId":thread,"model":model.or_else(|| self.default_model.clone()),"input":[{"type":"text","text":text,"text_elements":[]}]}), out);
        } else {
            self.request("thread/start", json!({"cwd":self.cwd,"approvalPolicy":"on-request","sandbox":"workspace-write"}), out);
        }
    }

    fn interrupt(&mut self, out: &mut Vec<Value>) {
        if let (Some(thread), Some(turn)) = (&self.thread, &self.turn) {
            self.request("turn/interrupt", json!({"threadId":thread,"turnId":turn}), out);
        }
    }

    fn control(&mut self, c: Control, out: &mut Vec<Value>) {
        match c {
            Control::Send(text, model) => {
                self.stopping = false;
                self.prompt = Some((text, model));
                self.initialize(out);
                self.begin(out);
            }
            Control::ListModels => {
                if self.list_models { return }
                self.list_models = true;
                self.models.clear();
                if self.ready { self.request("model/list", json!({"includeHidden":false}), out); }
                else { self.initialize(out); }
            }
            Control::Stop => {
                self.stopping = true;
                self.prompt = None;
                self.interrupt(out);
            }
            Control::Decide(id, accept) => {
                out.push(json!({"id":id,"result":{"decision":if accept { "accept" } else { "decline" }}}));
            }
            Control::Shutdown => {}
        }
    }

    fn receive(&mut self, v: Value, out: &mut Vec<Value>, events: &Sender<Event>) {
        let p = &v["params"];
        if let Some(method) = v["method"].as_str() {
            match method {
                "item/agentMessage/delta" => {
                    let _ = events.send(Event::Delta(p["itemId"].as_str().unwrap_or_default().into(), p["delta"].as_str().unwrap_or_default().into()));
                }
                "item/completed" if p["item"]["type"] == "agentMessage" => {
                    let _ = events.send(Event::Message(p["item"]["id"].as_str().unwrap_or_default().into(), p["item"]["text"].as_str().unwrap_or_default().into()));
                }
                "turn/started" => {
                    if self.turn.is_none() {
                        self.turn = p["turn"]["id"].as_str().map(str::to_owned);
                        if self.stopping { self.interrupt(out); }
                    }
                }
                "turn/completed" => {
                    self.turn = None;
                    self.changes.clear();
                    self.stopping = false;
                    if let Some(error) = p["turn"]["error"]["message"].as_str() { let _ = events.send(Event::Error(error.into())); }
                    let _ = events.send(Event::Done);
                }
                "item/started" => {
                    if p["item"]["type"] == "fileChange" {
                        let changes = p["item"]["changes"].as_array().map(|changes| changes.iter().map(|c|
                            format!("{}\n{}", c["path"].as_str().unwrap_or_default(), c["diff"].as_str().unwrap_or_default())
                        ).collect::<Vec<_>>().join("\n")).unwrap_or_default();
                        self.changes.insert(p["item"]["id"].as_str().unwrap_or_default().into(), changes);
                    }
                    if let Some(kind) = p["item"]["type"].as_str() { let _ = events.send(Event::Status(kind.into())); }
                }
                "serverRequest/resolved" => { let _ = events.send(Event::Resolved(p["requestId"].clone())); }
                "item/commandExecution/requestApproval" | "item/fileChange/requestApproval" => {
                    let mut description = [p["command"].as_str(), p["reason"].as_str(), p["cwd"].as_str(), p["grantRoot"].as_str()].into_iter().flatten().collect::<Vec<_>>().join("\n");
                    for key in ["networkApprovalContext", "additionalPermissions"] {
                        if !p[key].is_null() { description.push_str(&format!("\n{key}: {}", p[key])); }
                    }
                    let changes = p["itemId"].as_str().and_then(|id| self.changes.get(id)).map(String::as_str).unwrap_or_default();
                    let title = if method == "item/fileChange/requestApproval" { "File changes" } else { "Run command" };
                    let _ = events.send(Event::Approval(v["id"].clone(), format!("{title}\n{description}\n{changes}")));
                }
                _ if !v["id"].is_null() => {
                    out.push(json!({"id":v["id"],"error":{"code":-32601,"message":"This interaction is not supported by Quiet.md"}}));
                    let _ = events.send(Event::Error(format!("Unsupported Codex interaction: {method}")));
                }
                _ => {}
            }
            return;
        }
        let Some(method) = v["id"].as_u64().and_then(|id| self.requests.remove(&id)) else { return };
        if let Some(error) = v["error"]["message"].as_str() {
            if method == "model/list" {
                self.list_models = false;
                let _ = events.send(Event::ModelError(error.into()));
                return;
            }
            if method == "initialize" && self.list_models {
                self.list_models = false;
                let _ = events.send(Event::ModelError(error.into()));
            }
            self.prompt = None;
            let _ = events.send(Event::Error(error.into()));
            if method != "turn/interrupt" { let _ = events.send(Event::Done); }
            return;
        }
        match method.as_str() {
            "initialize" => {
                self.ready = true;
                out.push(json!({"method":"initialized"}));
                if self.list_models { self.request("model/list", json!({"includeHidden":false}), out); }
                if self.stopping { let _ = events.send(Event::Done); } else { self.begin(out); }
            }
            "model/list" => {
                let Some(data) = v["result"]["data"].as_array() else {
                    self.list_models = false;
                    let _ = events.send(Event::ModelError("Codex did not return a model list".into()));
                    return;
                };
                for item in data {
                    let Some(id) = item["model"].as_str().filter(|s| !s.is_empty()) else { continue };
                    if item["hidden"] == true || self.models.iter().any(|m| m.id == id) { continue }
                    self.models.push(Model { id: id.into(), name: item["displayName"].as_str().unwrap_or(id).into() });
                }
                if let Some(cursor) = v["result"]["nextCursor"].as_str() {
                    self.request("model/list", json!({"includeHidden":false,"cursor":cursor}), out);
                } else {
                    self.list_models = false;
                    let _ = events.send(Event::Models(std::mem::take(&mut self.models)));
                }
            }
            "thread/start" => {
                self.thread = v["result"]["thread"]["id"].as_str().map(str::to_owned);
                self.default_model = v["result"]["model"].as_str().map(str::to_owned);
                if self.thread.is_none() { let _ = events.send(Event::Error("Codex did not return a thread ID".into())); let _ = events.send(Event::Done); }
                else if self.stopping { let _ = events.send(Event::Done); }
                else { self.begin(out); }
            }
            "turn/start" if self.turn.is_none() => {
                self.turn = v["result"]["turn"]["id"].as_str().map(str::to_owned);
                if self.stopping { self.interrupt(out); }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_stream_and_continue_in_selected_directory() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp/project with spaces"));
        let mut out = vec![];
        p.request("initialize", json!({}), &mut out);
        p.control(Control::Send("hello".into(), None), &mut out);
        p.receive(json!({"id":1,"result":{}}), &mut out, &tx);
        assert_eq!(out[1]["method"], "initialized");
        assert_eq!(out[2]["params"]["cwd"], "/tmp/project with spaces");
        assert_eq!(out[2]["params"]["approvalPolicy"], "on-request");
        p.receive(json!({"id":2,"result":{"thread":{"id":"thread"}}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["params"]["input"][0]["text"], "hello");
        p.receive(json!({"method":"item/agentMessage/delta","params":{"itemId":"a","delta":"你好"}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Delta(id, s) if id == "a" && s == "你好"));
        p.receive(json!({"method":"turn/completed","params":{"turn":{"status":"completed"}}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Done));
        p.control(Control::Send("again".into(), None), &mut out);
        assert_eq!(out.last().unwrap()["params"]["threadId"], "thread");
    }

    #[test]
    fn stop_before_turn_response_and_approval_denial() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        let mut out = vec![];
        p.ready = true;
        p.thread = Some("t".into());
        p.control(Control::Send("hello".into(), None), &mut out);
        p.control(Control::Stop, &mut out);
        p.receive(json!({"id":1,"result":{"turn":{"id":"turn"}}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["method"], "turn/interrupt");
        p.receive(json!({"id":"approval","method":"item/fileChange/requestApproval","params":{"reason":"edit"}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Approval(id, _) if id == "approval"));
        p.control(Control::Decide(json!("approval"), false), &mut out);
        assert_eq!(out.last().unwrap(), &json!({"id":"approval","result":{"decision":"decline"}}));
    }

    #[test]
    fn errors_finish_turn_and_unknown_requests_are_rejected() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        let mut out = vec![];
        p.request("thread/start", json!({}), &mut out);
        p.receive(json!({"id":1,"error":{"message":"Login required"}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Error(s) if s == "Login required"));
        assert!(matches!(rx.recv().unwrap(), Event::Done));
        p.receive(json!({"id":9,"method":"item/tool/requestUserInput","params":{}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["error"]["code"], -32601);
    }

    #[test]
    fn stop_during_handshake_does_not_start_a_turn() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        let mut out = vec![];
        p.control(Control::Send("cancel me".into(), None), &mut out);
        p.control(Control::Stop, &mut out);
        p.receive(json!({"id":1,"result":{}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Done));
        assert!(!out.iter().any(|m| m["method"] == "thread/start" || m["method"] == "turn/start"));
        p.control(Control::Send("next".into(), None), &mut out);
        assert_eq!(out.last().unwrap()["method"], "thread/start");
    }

    #[test]
    fn approval_includes_file_diff_and_accepts_once() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        let mut out = vec![];
        p.receive(json!({"method":"item/started","params":{"item":{"id":"edit","type":"fileChange",
            "changes":[{"path":"hello.txt","diff":"+hello"}]}}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Status(_)));
        p.receive(json!({"id":8,"method":"item/fileChange/requestApproval","params":{"itemId":"edit"}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Approval(_, text) if text.contains("hello.txt") && text.contains("+hello")));
        p.control(Control::Decide(json!(8), true), &mut out);
        assert_eq!(out.last().unwrap()["result"]["decision"], "accept");
    }

    #[test]
    fn model_pages_use_model_identifiers_and_filter_hidden_duplicates() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        let mut out = vec![];
        p.control(Control::ListModels, &mut out);
        p.control(Control::ListModels, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0]["method"], "initialize");
        p.receive(json!({"id":1,"result":{}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["method"], "model/list");
        p.receive(json!({"id":2,"result":{"data":[
            {"id":"catalog-id","model":"actual-model","displayName":"Model A"},
            {"model":"hidden","hidden":true}],"nextCursor":"page-two"}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["params"]["cursor"], "page-two");
        p.receive(json!({"id":3,"result":{"data":[{"model":"actual-model"},{"model":"other"}]}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Models(models) if models == vec![
            Model { id:"actual-model".into(), name:"Model A".into() }, Model { id:"other".into(), name:"other".into() }
        ]));
    }

    #[test]
    fn switching_models_and_restoring_config_keeps_the_thread() {
        let (tx, _rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        p.ready = true;
        let mut out = vec![];
        p.control(Control::Send("first".into(), Some("chosen".into())), &mut out);
        p.receive(json!({"id":1,"result":{"thread":{"id":"same-thread"},"model":"configured-custom-model"}}), &mut out, &tx);
        assert_eq!(out.last().unwrap()["params"]["model"], "chosen");
        p.receive(json!({"method":"turn/completed","params":{"turn":{"status":"completed"}}}), &mut out, &tx);
        p.control(Control::Send("restore".into(), None), &mut out);
        assert_eq!(out.last().unwrap()["params"]["model"], "configured-custom-model");
        assert_eq!(out.last().unwrap()["params"]["threadId"], "same-thread");
        assert_eq!(out.iter().filter(|m| m["method"] == "thread/start").count(), 1);
    }

    #[test]
    fn model_list_error_does_not_cancel_a_prompt_and_can_retry() {
        let (tx, rx) = mpsc::channel();
        let mut p = Protocol::new(PathBuf::from("/tmp"));
        p.ready = true;
        p.prompt = Some(("pending".into(), None));
        let mut out = vec![];
        p.control(Control::ListModels, &mut out);
        p.receive(json!({"id":1,"error":{"message":"Catalog unavailable"}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::ModelError(s) if s == "Catalog unavailable"));
        assert!(rx.try_recv().is_err());
        assert_eq!(p.prompt.as_ref().unwrap().0, "pending");
        p.control(Control::ListModels, &mut out);
        p.receive(json!({"id":2,"result":{"data":[]}}), &mut out, &tx);
        assert!(matches!(rx.recv().unwrap(), Event::Models(models) if models.is_empty()));
    }

    #[test]
    #[ignore = "requires a locally installed, signed-in Codex and network access"]
    fn live_codex_round_trip() {
        let bin = executable().expect("Codex installed");
        let cwd = std::env::temp_dir().join(format!("quiet-agent-live-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&cwd).unwrap();
        let client = start(&bin, &cwd).unwrap();
        client.tx.send(Control::ListModels).unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(45);
        loop {
            match client.rx.recv_timeout(deadline.saturating_duration_since(std::time::Instant::now())).expect("Model list within 45 seconds") {
                Event::Models(models) => {
                    assert!(models.iter().all(|m| !m.id.is_empty()));
                    eprintln!("Codex model catalog: {} entries", models.len());
                    break;
                }
                Event::ModelError(e) | Event::Error(e) => panic!("Codex model list: {e}"),
                Event::Exited => panic!("Codex exited"),
                _ => {}
            }
        }
        let context = crate::agent::DocumentContext { title: "Synthetic reference".into(), markdown: "# Synthetic reference\n\nQUIET_AGENT_OK".into() };
        let first = crate::agent::compose_prompt("Read the referenced document and reply with its exact token only. Do not use any tools.", &[context]);
        for prompt in [first, "What exact token did you just reply? Reply with it only. Do not use any tools.".into()] {
            client.tx.send(Control::Send(prompt, None)).unwrap();
            let deadline = std::time::Instant::now() + Duration::from_secs(90);
            let mut answer = String::new();
            loop {
                match client.rx.recv_timeout(deadline.saturating_duration_since(std::time::Instant::now())).expect("Codex response within 90 seconds") {
                    Event::Message(_, s) => answer.push_str(&s),
                    Event::Error(e) => panic!("Codex: {e}"),
                    Event::Approval(_, _) => panic!("Unexpected tool approval for a text-only prompt"),
                    Event::Exited => panic!("Codex exited"),
                    Event::Done => break,
                    _ => {}
                }
            }
            assert!(answer.contains("QUIET_AGENT_OK"), "Unexpected answer: {answer}");
        }
    }
}
