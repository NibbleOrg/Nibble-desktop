import styles from "./NativeProbe.module.css";

/** Development risk harness; the production bootstrap never opens this URL. */
export function NativeProbe() {
  return (
    <main className={styles.probe}>
      <header>
        <strong>原生风险探测 · 非产品界面</strong>
        <input
          aria-label="探测输入"
          placeholder="输入以检查焦点（不保存）"
          autoFocus
        />
      </header>
      <p>⌘2 搜索尺寸 · ⌘3 历史尺寸 · ⌘4 Base64 尺寸</p>
      <p>材质对照：⌘5 开启 · ⌘6 关闭 · ⌘7 输出终端诊断</p>
      <p>⌘W / Esc 关闭并恢复原应用 · 外部点击不抢回焦点</p>
      <p>使用 Native probe 菜单中的延迟打开，3 秒内切到目标应用。</p>
      <p>
        观察顶边、背景模糊、当前 Space 与全屏覆盖。原生尺寸直接切换，无动画。
      </p>
      <label>
        底部可达性检查 <input placeholder="小屏仍应可滚动到此处" />
      </label>
    </main>
  );
}
