# 交互与视觉契约

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

本页快捷键及 Space 行为针对 macOS；Windows 适配见[Windows 规划](../operations/windows-v4.md)。

## 应用与窗口

启动进入 Workbench Home；关闭 Workbench 隐藏窗口，托盘及已获准的后台服务继续运行。⌘Q 完整退出。Dock、应用激活与托盘打开 Workbench；托盘另提供 Quick Panel、暂停/恢复和退出。所有入口指向同一应用实例。

Quick Panel 默认可配置快捷键为 ⌥Space，注册失败明确报告，不静默改键。每次唤起在指针所在显示器顶部居中，进入当前 Space，可覆盖全屏应用但不跨 Space 常驻。唤起前记录原应用；搜索框立即获焦。

Esc、⌘W、成功复制关闭面板，并在原应用仍存在时恢复应用级焦点；不保证原输入框或光标。外部点击将焦点交给所点击应用。进入 Workbench/Settings 则激活目标。无模拟点击、按键或粘贴。

## 工具、返回与快捷键

空查询列出工具；Return 进入选中工具。同一原生窗口顶部锚定，向下调整大小，切换期间禁用重复选择，完成后聚焦主控件。无标签页、直接工具互跳或第二个工具窗口。

工具内 ⌘[ 或返回按钮回到搜索并清空查询与工具临时状态。Esc 关闭整个面板；唯快捷键参考层打开时第一次 Esc 关闭参考层并恢复触发控件焦点，第二次才关闭面板。

Clipboard History 用上下键选择、Return 复制并关闭，⌘P 切换暂停且保持面板。空、暂停、权限、条目不可用及复制失败就地显示。Base64 用 Tab 遍历输入、方向、字符集与结果；方向/字符集支持方向键或 Space；Return 换行，⌘Return 复制有效结果。

操作失败保留输入并就地提示；成功先写剪贴板再关闭。每次重新唤起从空搜索开始。⌘, 关闭面板并打开 Settings → Quick Panel，不传递内容。Workbench 在同次运行隐藏后保留路由和浏览月份，完整重启回到 Home 当前月。

Settings 提供按作用范围分组的完整快捷键参考；Quick Panel 显示当前可用动作的紧凑提示，并可打开临时参考层。输入缺失、权限或操作失败就地呈现，短暂成功轻量确认；只有启动失败或整体不可用才使用应用级阻塞状态。

## 视觉与动态

Quick Panel 采用 D · Frosted pane：暖白半透明、强背景模糊、细边缘与克制高光；可读性优先，必要时中性遮罩。单列工具，选中项为唯一强抬升行，暖橙前标；紧凑搜索和内嵌底栏。原型约 650×264、46px 搜索/行高、19px 搜索字、25px 外圆角均是调优起点。

Workbench 采用 A · Ink rail：近黑低噪背景、居中轻抬升月历、暖橙今天/焦点强调。窄图标轨道，首页为主入口、设置固定底部且窄窗口仍可达；无 Quick Panel 触发按钮或嵌入内容。

月历为公历六行 42 格，相邻月日期弱化，今天为唯一强强调填充。系统地区控制标签与周首日。原型最大宽约 1060px、高约 430–660px，随可用空间压缩；空月局部提示“本月暂无日历事件”。

两界面共享设计 token 与基础组件，各有外壳和布局。200ms 为动态基线；Workbench 路由短淡入加约 5–6px 位移，月份约 9px 方向交叉淡入。Quick Panel 顶边锚定、内容交叉淡入。减少动态效果时直接切换，无空间或交叉淡入动画。具体缓动、原生尺寸和性能须实机调优。

浏览器原型仅验证方向，不证明跨应用 vibrancy 或透明效果；生产实现重建并验证。参考 [Quick Panel 原型](../../prototypes/quick-panel-visual/README.md) 与 [Workbench 原型](../../prototypes/workbench-visual/README.md)。


来源：[#5](https://github.com/NibbleOrg/Nibble-desktop/issues/5)、[#6](https://github.com/NibbleOrg/Nibble-desktop/issues/6)、[#9](https://github.com/NibbleOrg/Nibble-desktop/issues/9)、[#14](https://github.com/NibbleOrg/Nibble-desktop/issues/14)
