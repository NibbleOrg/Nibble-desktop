# Workbench 原生验收记录

Status: Draft
Owner: 项目维护者
Last verified: 2026-11-17 — 合并提交上的 Intel macOS 原生 smoke；未覆盖项逐项列明

## 版本与环境

- 源码：`5bd3aa21e09a30240a15bc41920e6d09d7d346d3`（PR 49 合并提交），无业务代码追加修改。
- 本机：Intel x86_64，macOS 26.7.1，build 25G241。
- 命令：`pnpm exec tauri build --bundles app`，Vite、Rust release 和本地 `.app` 打包通过。
- 运行对象：`src-tauri/target/release/bundle/macos/Nibble Local.app`，从该路径通过原生 UI 工具启动，URL 为 `tauri://localhost/index.html?surface=workbench`。不是浏览器或旧安装包。
- 这是本地验收产物，不是签名、公证、公开分发或 Apple Silicon 验收。

## 已观察结果

| 操作 | 实际观察与结论 |
|---|---|
| 启动 | Home 显示 November 2026，六行 42 个日期格；本机日期为 2026-11-17。当前地区标签为英文、周日开头。 |
| 下个月 | 变为 December 2026，跨年相邻日期包含 January 2027。 |
| 设置，再返回 Home | 设置显示 Quick Panel 和 ⌥Space 已注册；返回后仍为 December 2026。 |
| 设置中点击原生关闭按钮，再从同一 app 路径重新激活 | 设置路由保留，回 Home 仍为 December 2026。仅确认关闭请求/重新激活后状态保留；工具可能自动激活窗口，未独立量测隐藏期间的可见性或托盘/Dock 入口。 |
| 从 December 2026 按 ⌘Q，再从同一 app 路径启动 | 原生工具报告 App quit；重新启动回 Home November 2026。 |
| 从 Settings 按 ⌘Q，再重新启动 | 重新启动回 Home November 2026，路由与浏览月份重置。 |

[重新激活后仍为 December 2026](workbench-native-retained.jpg)；[从 Settings 完整退出后回 Home 当前月](workbench-native-restart.jpg)。

## 未完成项目

以下均 **not verified**，不能因为 UI 已确认或 PR 已合并而标为通过：

- Quick Panel → ⌘, → Settings → Quick Panel：工具发送 ⌥Space 后读取到的仍为 Workbench，未取得面板可验证状态；无法据此断言产品快捷键失败或集成成功。
- 系统地区/非公历默认地区、周首日用户覆盖、时区变化与午夜唤醒；尚未修改用户系统设置。
- 原生窄窗、键盘焦点、系统减少动态开关及 200ms 动态录屏；此前浏览器窄窗证据仅覆盖布局。
- 多显示器/DPI、四边距的实测量化、屏幕断开、Space/全屏、独立托盘/Dock 重新打开。
- Apple Silicon 原生验收：当前没有该硬件。
- 独立代码评审：PR 无正式 review，不将人类审批等同独立 CR。
- PR 49 远端 CI：[run 43910204920](https://github.com/NibbleOrg/Nibble-desktop/actions/runs/43910204920) 为 cancelled，job 无执行步骤；不是 passed，也没有可归因的代码失败日志。合并提交对应 [main run 43913987099](https://github.com/NibbleOrg/Nibble-desktop/actions/runs/43913987099) 查询时为 queued，仍未验证。

实施与剩余矩阵见[Workbench 月历](../workbench-calendar.md)。本记录不关闭实施议题。
