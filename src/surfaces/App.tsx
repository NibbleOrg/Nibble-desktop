import styles from "./App.module.css";
export function App({ surface }: { surface: "workbench" | "quick-panel" }) {
  return (
    <main className={styles.shell}>
      <p>Nibble · 工程初始化</p>
      <h1>{surface === "workbench" ? "Workbench" : "Quick Panel"}</h1>
      <p>本地启动骨架，业务能力尚未实现。</p>
    </main>
  );
}
