import { useEffect, useRef, useState } from "react";
import {
  commands,
  type SettingsError,
  type SettingsSnapshot,
} from "../ipc/bindings";
import { createVersionReader } from "../ipc/lifecycle-sync";
import styles from "./Workbench.module.css";
const errors: Record<SettingsError, string> = {
  invalidShortcut:
    "快捷键无效。请使用 Alt、Control 或 Super 修饰键与一个按键，例如 Alt+Space。",
  restartRequired:
    "系统未能确认移除快捷键。配置恢复未完成，请完整退出并重启后再修改；仍可从托盘打开面板。",
  registrationFailed:
    "系统拒绝注册快捷键。原有配置保留；可从托盘打开面板，或换一个组合重试。",
  saveFailed: "保存失败，已恢复原有配置。请检查本地配置目录是否可写，再重试。",
  recoveryFailed:
    "配置恢复未完成，磁盘与注册状态可能不一致。请从托盘进入设置，重新保存快捷键；重启后检查状态。",
  serviceUnavailable: "快捷键服务未启动。请退出后重启；仍可从托盘打开面板。",
  forbiddenWindow: "此窗口没有修改设置的权限。",
};
function describe(error: unknown) {
  return typeof error === "string" && error in errors
    ? errors[error as SettingsError]
    : "设置服务暂不可用，请重新进入设置或重启后重试。";
}
export function Settings({ version }: { version: number }) {
  const [snapshot, setSnapshot] = useState<SettingsSnapshot | null>(null);
  const [draft, setDraft] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [pending, setPending] = useState(false);
  const mounted = useRef(true);
  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);
  useEffect(() => {
    const reader = createVersionReader(
      commands.settingsSnapshot,
      (next) =>
        setSnapshot((old) =>
          !old || next.version >= old.version ? next : old,
        ),
      () => setMessage("设置服务暂不可用，请重新进入设置或重启后重试。"),
    );
    void reader.refresh(version);
    return () => reader.dispose();
  }, [version]);
  async function save() {
    if (!snapshot || pending) return;
    setPending(true);
    setMessage(null);
    setSuccess(false);
    try {
      const next = await commands.settingsSave(draft ?? snapshot.shortcut);
      if (!mounted.current) return;
      setSnapshot((old) => (!old || next.version >= old.version ? next : old));
      setDraft(null);
      setSuccess(true);
    } catch (error) {
      if (mounted.current) setMessage(describe(error));
    } finally {
      if (mounted.current) setPending(false);
    }
  }
  return (
    <section className={`${styles.settings} ${styles.route}`}>
      <p className={styles.eyebrow}>SETTINGS</p>
      <h1>设置</h1>
      <h2>Quick Panel</h2>
      {!snapshot && !message && <p>正在读取本地设置…</p>}
      {snapshot && (
        <>
          <p>
            当前快捷键：{snapshot.shortcut}（
            {snapshot.available ? "已注册" : "未注册；请从托盘打开面板"}）
          </p>
          <form
            onSubmit={(event) => {
              event.preventDefault();
              void save();
            }}
          >
            <label htmlFor="global-shortcut">全局快捷键</label>
            <input
              id="global-shortcut"
              value={draft ?? snapshot.shortcut}
              disabled={pending}
              maxLength={128}
              aria-describedby="shortcut-help"
              onChange={(event) => {
                setDraft(event.target.value);
                setSuccess(false);
                setMessage(null);
              }}
            />
            <p id="shortcut-help">
              使用 Alt、Control、Shift、Super 与按键名称组合，例如
              Alt+Space（⌥Space）。至少包含 Alt、Control 或 Super。Super 对应
              ⌘。采用独占注册，不支持媒体键。
            </p>
            <button type="submit" disabled={pending}>
              {pending ? "正在保存…" : "保存快捷键"}
            </button>
            <button
              type="button"
              disabled={pending}
              onClick={() => {
                setDraft("Alt+Space");
                setSuccess(false);
                setMessage(null);
              }}
            >
              使用默认组合
            </button>
          </form>
          {snapshot.startupRecovered && (
            <p>
              本地配置无法读取或格式不兼容，已尝试默认快捷键；原文件保留，保存前不会覆盖。
            </p>
          )}
          {!message && snapshot.error && (
            <p role="alert">{errors[snapshot.error]}</p>
          )}
        </>
      )}
      {message && <p role="alert">{message}</p>}
      {success && <p role="status">快捷键已生效并保存。</p>}
      <h2>快捷键参考</h2>
      <dl>
        <dt>整个应用</dt>
        <dd>
          {snapshot?.shortcut ?? "已配置的全局快捷键"}：打开或关闭 Quick
          Panel；⌘,：打开设置；⌘Q：完整退出。
        </dd>
        <dt>Workbench</dt>
        <dd>关闭窗口：隐藏并继续驻留托盘。</dd>
        <dt>Quick Panel 搜索</dt>
        <dd>↑ / ↓：选择工具；Return：打开所选工具。</dd>
        <dt>通用控件</dt>
        <dd>Tab：遍历控件；Space / 方向键：操作按钮与选项。</dd>
        <dt>Quick Panel 与工具</dt>
        <dd>
          Esc / ⌘W：关闭面板；⌘[：返回搜索；参考层内第一次 Esc 仅关闭参考层。
        </dd>
        <dt>Clipboard History 演示</dt>
        <dd>
          ↑ /
          ↓：选择演示条目；⌘P：暂停或恢复演示；Return：演示复制，不写系统剪贴板。
        </dd>
        <dt>Base64 演示</dt>
        <dd>
          ⌘Return：有有效预设结果时模拟完成；Return：换行。复制仅演示，不写系统剪贴板。
        </dd>
      </dl>
      <h2>关于</h2>
      <p>{snapshot ? `Nibble · 版本 ${snapshot.appVersion}` : "版本读取中"}</p>
      <p>
        本地体验版本。仅快捷键配置写入本地；工具使用演示数据，不读取或写入系统剪贴板，不保存工具内容。无自动上传或更新入口。
      </p>
      <p>关闭窗口后继续驻留托盘；⌘Q 完整退出。</p>
    </section>
  );
}
