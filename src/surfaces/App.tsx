import { useEffect, useState } from "react";
import {
  commands,
  type LifecycleError,
  type Snapshot_Serialize,
} from "../ipc/bindings";
import { useLifecycle } from "../ipc/useLifecycle";
import styles from "./App.module.css";
import workbench from "./Workbench.module.css";
import { Calendar } from "./CalendarView";
import { Panel } from "./Panel";
import { Settings } from "./Settings";
const errors: Record<LifecycleError, string> = {
  shortcutRegistrationFailed:
    "快捷键注册失败。可从托盘打开 Quick Panel，并进入设置重试。",
  focusRestoreRejected: "系统未接受焦点恢复；面板已关闭，可自行切回原应用。",
  windowOperationFailed: "窗口操作失败，请从托盘重试。",
  displayUnavailable: "没有可用显示器，请连接显示器后重试。",
  windowUnavailable: "窗口不可用，请退出后重新启动。",
  staleOrBusy: "当前切换尚未完成，请稍后重试。",
  forbiddenWindow: "此窗口没有执行操作的权限。",
  staleSession: "此会话已关闭，请重新打开面板。",
  returnToSearchFirst: "请先返回搜索再选择其他工具。",
};
export function App({ surface }: { surface: "workbench" | "quick-panel" }) {
  const { snapshot, failed } = useLifecycle();
  if (surface === "quick-panel")
    return (
      <main className={styles.panel}>
        <h1>Nibble · Quick Panel</h1>
        {snapshot?.session != null ? (
          <Panel
            key={snapshot.session}
            snapshot={snapshot}
            fault={snapshot.fault ? errors[snapshot.fault] : undefined}
          />
        ) : (
          <p>本地启动骨架，业务能力尚未实现。</p>
        )}
        {failed && (
          <p role="alert">窗口服务暂不可用，请用 ⌥Space 或托盘关闭后重试。</p>
        )}
      </main>
    );
  return <Workbench snapshot={snapshot} failed={failed} />;
}
function Workbench({
  snapshot,
  failed,
}: {
  snapshot: Snapshot_Serialize | null;
  failed: boolean;
}) {
  const [route, setRoute] = useState("home");
  const [seen, setSeen] = useState(-1);
  if (snapshot?.settingsRequested && snapshot.version !== seen) {
    setSeen(snapshot.version);
    setRoute("settings");
  }
  useEffect(() => {
    if (snapshot?.settingsRequested)
      void commands.settingsAcknowledged().catch(() => {});
  }, [snapshot?.settingsRequested]);
  return (
    <main
      className={workbench.shell}
      data-reduced-motion={snapshot?.reducedMotion ?? false}
    >
      <nav className={workbench.rail} aria-label="Workbench">
        <span className={workbench.brand} aria-label="Nibble">
          n.
        </span>
        <button
          aria-label="Home"
          aria-current={route === "home" ? "page" : undefined}
          onClick={() => setRoute("home")}
          title="Home"
        >
          ⌂
        </button>
        <button
          className={workbench.settingsButton}
          aria-label="设置"
          aria-current={route === "settings" ? "page" : undefined}
          onClick={() => setRoute("settings")}
          title="设置"
        >
          ⚙
        </button>
      </nav>
      <div className={workbench.content}>
        <div hidden={route !== "home"} className={workbench.route}>
          <Calendar reducedMotion={snapshot?.reducedMotion ?? false} />
        </div>
        {route === "settings" && <Settings version={snapshot?.version ?? 0} />}
        {snapshot?.fault && (
          <p role="alert">
            {errors[snapshot.fault] ?? "窗口服务异常，请从托盘重试。"}
          </p>
        )}
        {snapshot?.shortcutFailure && (
          <p role="alert">{snapshot.shortcutFailure}</p>
        )}
        {failed && (
          <p role="alert">窗口服务暂不可用，重新激活 Workbench 后重试。</p>
        )}
      </div>
    </main>
  );
}
