import { useEffect, useRef, useState } from "react";
import styles from "./Panel.module.css";
type Scenario = "normal" | "empty" | "permission" | "unavailable" | "failure";
let currentScenario: Scenario = "normal";
const entries = [
  "周五下午整理本周笔记",
  "https://example.com/nibble",
  "演示长文本\n保留换行，用于体验列表选择与可读性。",
];
const presets = {
  encode: { input: "Hello", result: "SGVsbG8=" },
  decode: { input: "SGVsbG8=", result: "Hello" },
  url: { input: "???", result: "Pz8_" },
  invalid: { input: "%%%", result: "" },
};
export function DemoTools({
  view,
  paused,
  onPause,
  onComplete,
}: {
  view: "history" | "base64";
  paused: boolean;
  onPause: () => void;
  onComplete: () => void;
}) {
  const [scenario, setScenario] = useState<Scenario>(currentScenario);
  const [lab, setLab] = useState(false);
  const [selection, setSelection] = useState(0);
  const [input, setInput] = useState("");
  const [preset, setPreset] = useState<keyof typeof presets | null>(null);
  const [direction, setDirection] = useState("encode");
  const [alphabet, setAlphabet] = useState("standard");
  const [feedback, setFeedback] = useState("");
  const [error, setError] = useState("");
  const root = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (view === "history") root.current?.focus();
    else root.current?.querySelector("textarea")?.focus();
  }, [view]);
  const timer = useRef<number | null>(null);
  const complete = useRef(onComplete);
  useEffect(() => {
    complete.current = onComplete;
  }, [onComplete]);
  useEffect(
    () => () => {
      if (timer.current !== null) window.clearTimeout(timer.current);
    },
    [],
  );
  const historyReady =
    !paused && scenario !== "empty" && scenario !== "permission";
  const result = preset === null ? "" : presets[preset].result;
  function finish() {
    if (feedback || (view === "history" ? !historyReady : !result)) return;
    if (
      view === "history" &&
      (scenario === "failure" || scenario === "unavailable")
    ) {
      setError(
        scenario === "failure"
          ? "模拟复制失败，请重试；未写入剪贴板。"
          : "此演示条目不可用，请选择其他场景。",
      );
      return;
    }
    setError("");
    setFeedback("演示操作完成，未写入剪贴板");
    timer.current = window.setTimeout(() => complete.current(), 1000);
  }
  function choose(value: keyof typeof presets) {
    if (feedback) return;
    setPreset(value);
    setInput(presets[value].input);
    setError("");
    setDirection(
      value === "decode" || value === "invalid" ? "decode" : "encode",
    );
    setAlphabet(value === "url" ? "url" : "standard");
  }
  function configuration(nextDirection: string, nextAlphabet: string) {
    setDirection(nextDirection);
    setAlphabet(nextAlphabet);
    setError("");
    // Only exact, explicitly selected fixture combinations have results.
    if (preset !== null) {
      const key =
        nextDirection === "decode"
          ? "decode"
          : nextAlphabet === "url"
            ? "url"
            : "encode";
      setPreset(key);
      setInput(presets[key].input);
    }
  }
  return (
    <div
      ref={root}
      className={styles.tool}
      onKeyDown={(event) => {
        if (
          view === "history" &&
          event.metaKey &&
          event.key.toLowerCase() === "p"
        ) {
          event.preventDefault();
          onPause();
        }
        if (
          view === "history" &&
          !event.metaKey &&
          ["ArrowUp", "ArrowDown"].includes(event.key)
        ) {
          event.preventDefault();
          setSelection(
            (value) =>
              (value + (event.key === "ArrowDown" ? 1 : entries.length - 1)) %
              entries.length,
          );
        }
        if (
          (view === "history" &&
            event.key === "Enter" &&
            (event.target === event.currentTarget ||
              (event.target as HTMLElement).getAttribute("role") ===
                "option")) ||
          (view === "base64" && event.metaKey && event.key === "Enter")
        ) {
          event.preventDefault();
          finish();
        }
      }}
      tabIndex={view === "history" ? 0 : undefined}
      aria-label={view === "history" ? "历史演示" : "Base64 演示"}
    >
      <p className={styles.notice}>
        演示模式 · 未读取系统剪贴板，未执行真实转换
      </p>
      {view === "history" ? (
        <>
          <button onClick={onPause} disabled={!!feedback}>
            {paused ? "恢复演示 · ⌘P" : "暂停演示 · ⌘P"}
          </button>
          <p>仅切换临时演示状态，不控制真实捕获。</p>
          {paused ? (
            <p>演示已暂停</p>
          ) : scenario === "empty" ? (
            <p>暂无演示历史</p>
          ) : scenario === "permission" ? (
            <p>模拟权限不可用；本阶段不请求权限。</p>
          ) : (
            <div
              role="listbox"
              aria-label="演示条目"
              aria-activedescendant={`demo-entry-${selection}`}
            >
              {entries.map((entry, index) => (
                <button
                  id={`demo-entry-${index}`}
                  role="option"
                  aria-selected={index === selection}
                  key={entry}
                  onFocus={() => setSelection(index)}
                  onClick={() => {
                    setSelection(index);
                    setError("");
                  }}
                  disabled={!!feedback}
                >
                  {entry}
                </button>
              ))}
            </div>
          )}
        </>
      ) : (
        <>
          <div className={styles.presets} aria-label="预设样例">
            <button disabled={!!feedback} onClick={() => choose("encode")}>
              编码样例
            </button>
            <button disabled={!!feedback} onClick={() => choose("decode")}>
              解码样例
            </button>
            <button disabled={!!feedback} onClick={() => choose("url")}>
              URL-safe 样例
            </button>
            <button disabled={!!feedback} onClick={() => choose("invalid")}>
              无效样例
            </button>
          </div>
          <label>
            输入
            <textarea
              aria-label="输入"
              value={input}
              disabled={!!feedback}
              onChange={(e) => {
                setInput(e.target.value);
                setPreset(null);
                setError("");
              }}
              placeholder="自由编辑仅体验输入；请选择预设样例查看结果"
            />
          </label>
          <div className={styles.controls}>
            <label>
              方向
              <select
                value={direction}
                disabled={!!feedback}
                onChange={(e) => configuration(e.target.value, alphabet)}
              >
                <option value="encode">编码</option>
                <option value="decode">解码</option>
              </select>
            </label>
            <label>
              字符集
              <select
                value={alphabet}
                disabled={!!feedback}
                onChange={(e) => configuration(direction, e.target.value)}
              >
                <option value="standard">标准</option>
                <option value="url">URL-safe</option>
              </select>
            </label>
          </div>
          <label>
            演示结果
            <textarea aria-label="演示结果" readOnly value={result} />
          </label>
          {preset === null && <p>本阶段未启用转换</p>}
          {preset === "invalid" && (
            <p role="alert">预设无效输入，未生成结果。</p>
          )}
        </>
      )}
      <button
        disabled={!!feedback || (view === "history" ? !historyReady : !result)}
        onClick={finish}
      >
        模拟完成
      </button>
      {feedback && <p role="status">{feedback}</p>}
      {error && <p role="alert">{error}</p>}
      <aside className={styles.lab} aria-label="独立开发工具区">
        <button aria-expanded={lab} onClick={() => setLab(!lab)}>
          开发场景
        </button>
        {lab && (
          <label>
            历史演示场景
            <select
              value={scenario}
              disabled={!!feedback}
              onChange={(e) => {
                const value = e.target.value as Scenario;
                currentScenario = value;
                setScenario(value);
                setError("");
                setSelection(0);
              }}
            >
              <option value="normal">正常</option>
              <option value="empty">空历史</option>
              <option value="permission">权限不可用</option>
              <option value="unavailable">条目不可用</option>
              <option value="failure">复制失败</option>
            </select>
          </label>
        )}
      </aside>
    </div>
  );
}
