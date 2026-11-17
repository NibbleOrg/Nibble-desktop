import { lazy, Suspense, useEffect, useRef, useState } from "react";
import { commands, type Snapshot_Serialize, type View } from "../ipc/bindings";
import shell from "./App.module.css";
import styles from "./Panel.module.css";
const Demo =
  import.meta.env.MODE === "local-experience"
    ? lazy(() =>
        import("./PanelDemo").then((module) => ({ default: module.DemoTools })),
      )
    : null;
const tools = [
  {
    view: "history",
    name: "Clipboard History",
    aliases: "clipboard history 剪贴板 历史",
    description: "查看最近的文本条目",
  },
  {
    view: "base64",
    name: "Base64",
    aliases: "base64 编码 解码",
    description: "文本编码与解码",
  },
] as const;
export function Panel({
  snapshot,
  fault,
}: {
  snapshot: Snapshot_Serialize;
  fault?: string;
}) {
  const session = snapshot.session!;
  const busy = snapshot.transition !== null;
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  const [error, setError] = useState("");
  const [reference, setReference] = useState(false);
  const leaving = useRef(false);
  useEffect(() => {
    leaving.current = false;
  }, [snapshot.view]);
  const input = useRef<HTMLInputElement>(null);
  const content = useRef<HTMLDivElement>(null);
  const referenceButton = useRef<HTMLButtonElement>(null);
  const dismiss = useRef<HTMLButtonElement>(null);
  const matched = tools.filter((tool) =>
    tool.aliases.includes(query.trim().toLowerCase()),
  );
  function run(action: Promise<unknown>) {
    void action.catch(() =>
      setError("操作未完成，请重试；可随时按 Esc 关闭。"),
    );
  }
  function change(view: View) {
    if (!busy && view !== snapshot.view) {
      setQuery("");
      setSelected(0);
      setError("");
      leaving.current = true;
      void commands.panelTransition(session, view).catch(() => {
        leaving.current = false;
        setError("切换未完成，请重试；可随时按 Esc 关闭。");
      });
    }
  }
  function closeReference() {
    setReference(false);
    window.queueMicrotask(() => referenceButton.current?.focus());
  }
  useEffect(() => {
    if (!busy) {
      if (snapshot.view === "search") input.current?.focus();
      else
        content.current
          ?.querySelector<HTMLElement>(
            'textarea, [aria-label="历史演示"], [data-primary]',
          )
          ?.focus();
    }
  }, [busy, snapshot.view]);
  useEffect(() => {
    if (reference) dismiss.current?.focus();
  }, [reference]);
  useEffect(() => {
    if (snapshot.transition === null || !snapshot.reducedMotion) return;
    const request = snapshot.transition;
    const timer = window.setTimeout(() => {
      void commands.panelTransitionComplete(session, request).catch(() => {});
    }, 32);
    return () => window.clearTimeout(timer);
  }, [session, snapshot.transition, snapshot.reducedMotion]);
  useEffect(() => {
    function key(event: KeyboardEvent) {
      if (event.key === "Escape") {
        event.preventDefault();
        if (reference) closeReference();
        else run(commands.panelClose(session));
      }
      if (event.metaKey && event.key.toLowerCase() === "w") {
        event.preventDefault();
        run(commands.panelClose(session));
      }
      if (event.metaKey && event.key === ",") {
        event.preventDefault();
        run(commands.panelSettings(session));
      }
      if (!reference && event.metaKey && event.key === "[") {
        event.preventDefault();
        change("search");
      }
      if (reference && event.key === "Tab") {
        event.preventDefault();
        dismiss.current?.focus();
      }
    }
    window.addEventListener("keydown", key);
    return () => window.removeEventListener("keydown", key);
  });
  return (
    <section
      className={`${styles.content} ${busy && !snapshot.reducedMotion ? shell.transition : ""}`}
      aria-busy={busy}
      onAnimationEnd={(event) => {
        if (
          event.target === event.currentTarget &&
          snapshot.transition !== null
        )
          run(commands.panelTransitionComplete(session, snapshot.transition));
      }}
    >
      <div inert={reference}>
        {snapshot.view === "search" ? (
          <div className={styles.search}>
            <input
              ref={input}
              aria-label="搜索工具"
              placeholder="搜索工具…"
              value={query}
              onChange={(e) => {
                setQuery(e.target.value);
                setSelected(0);
              }}
              role="combobox"
              aria-expanded={true}
              aria-controls="tool-list"
              aria-activedescendant={
                matched[selected] ? `tool-${matched[selected].view}` : undefined
              }
              onKeyDown={(event) => {
                if (event.nativeEvent.isComposing || busy) return;
                if (
                  ["ArrowDown", "ArrowUp"].includes(event.key) &&
                  matched.length
                ) {
                  event.preventDefault();
                  setSelected(
                    (value) =>
                      (value +
                        (event.key === "ArrowDown" ? 1 : matched.length - 1)) %
                      matched.length,
                  );
                }
                if (event.key === "Enter" && matched[selected]) {
                  event.preventDefault();
                  change(matched[selected].view);
                }
              }}
            />
            <div id="tool-list" role="listbox" aria-label="工具">
              {matched.map((tool, index) => (
                <button
                  id={`tool-${tool.view}`}
                  role="option"
                  aria-selected={selected === index}
                  className={styles.row}
                  key={tool.view}
                  disabled={busy}
                  onClick={() => change(tool.view)}
                >
                  <span>{tool.name}</span>
                  <small>{tool.description}</small>
                </button>
              ))}
            </div>
            {!matched.length && <p>没有匹配的工具</p>}
          </div>
        ) : (
          <>
            <header className={styles.toolbar}>
              <button disabled={busy} onClick={() => change("search")}>
                返回搜索 · ⌘[
              </button>
              <h2>
                {snapshot.view === "history" ? "Clipboard History" : "Base64"}
              </h2>
            </header>
            <div ref={content} key={snapshot.view} inert={busy}>
              {Demo ? (
                <Suspense fallback={<p>载入演示…</p>}>
                  <Demo
                    view={snapshot.view}
                    paused={snapshot.demoPaused}
                    onPause={() => run(commands.panelToggleDemoPause(session))}
                    onComplete={() => {
                      if (!leaving.current) run(commands.panelClose(session));
                    }}
                  />
                </Suspense>
              ) : (
                <p tabIndex={-1} data-primary>
                  当前构建未提供此工具能力，未读取或写入系统剪贴板，未执行转换。
                </p>
              )}
            </div>
          </>
        )}
        {error && <p role="alert">{error}</p>}
        {fault && <p role="alert">{fault}</p>}
        <footer className={styles.toolbar}>
          <button onClick={() => run(commands.panelClose(session))}>
            关闭 · Esc
          </button>
          <button ref={referenceButton} onClick={() => setReference(true)}>
            快捷键参考
          </button>
          <button onClick={() => run(commands.panelSettings(session))}>
            设置 · ⌘,
          </button>
        </footer>
      </div>
      {reference && (
        <div
          className={styles.reference}
          role="dialog"
          aria-modal="true"
          aria-label="快捷键参考"
        >
          <h2>快捷键参考</h2>
          <dl>
            <dt>Esc / ⌘W</dt>
            <dd>关闭面板；参考层内首次 Esc 仅关闭参考层</dd>
            <dt>⌘,</dt>
            <dd>打开设置</dd>
            <dt>↑ ↓ / Return</dt>
            <dd>搜索中选择工具并进入；历史中选择条目并模拟完成</dd>
            <dt>⌘[</dt>
            <dd>返回空搜索</dd>
            <dt>⌘P</dt>
            <dd>历史演示中暂停/恢复</dd>
            <dt>Tab / Space / 方向键</dt>
            <dd>遍历控件与切换选项</dd>
            <dt>⌘Return</dt>
            <dd>Base64 有有效预设结果时模拟完成；Return 换行</dd>
          </dl>
          <button ref={dismiss} onClick={closeReference}>
            关闭参考
          </button>
        </div>
      )}
    </section>
  );
}
