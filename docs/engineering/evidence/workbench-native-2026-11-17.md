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

## 授权系统偏好测试与修复复测

维护者明确授权临时切换系统地区、周首日与减少动态，测试后恢复。原值：English 主语言、简体中文次语言；China mainland、Gregorian、Sunday、Celsius、Metric、日期 2026/8/19、数字 1,234,567.89、Universal 排序；Reduce motion=off。

在原合并版发现可重复缺陷：First day of week=Monday 时，月历仍从 Sunday November 1 开始，完整退出重启也不变；United Kingdom 地区默认周一也未被采用。Foundation 原生读取替代仅靠 WebKit locale 的实现后，追加工作树候选复测：

- 中国大陆地区单独将周首日改为 Monday，点击激活 Workbench 后首格变为 Monday October 26，六行仍完整，见[周一覆盖截图](workbench-native-monday.jpg)。
- 切到 United Kingdom 并保留 English (US) 主语言，运行中地区格式存在进程缓存；完整退出重启后首格为 Monday, 26 October 2026，日期为英国格式，见[英国地区冷启动截图](workbench-native-uk.jpg)。这里只验证冷启动，不将运行中地区自动更新标为通过。
- 原合并版系统临时设为 Buddhist（系统显示 2569 BE），应用在重新启动后仍为 November 2026：固定公历通过。追加原生接口只读偏好，不改变显式 gregory 格式化。
- 原合并版 Reduce motion=on 时月份浏览正常；没有逐帧录像或原生 DOM 动画样式采样，不能宣称无位移/交叉淡入已通过，此项仍 not verified。
- 每轮均恢复设置；最后回读 China mainland、Gregorian、Sunday、原日期/数字格式、Celsius、Metric、English 主语言和 Reduce motion=off，并完整重启应用回到周日开头的 November 2026。未修改时间、时区、账户、网络或任何隐私权限。

截图仅包含 Nibble 窗口，没有保存系统设置中的账户画面。原生修复候选尚未合并，原合并提交不因本记录而自动变为通过。

最终本地检查：`pnpm test`（13 前端 / 23 Rust）、`pnpm lint`、`pnpm typecheck`（含生成漂移）、`pnpm format:check`、`git diff --check` 及 `pnpm exec tauri build --bundles app` 全部通过。文档相对链接核对通过。未重复隔离探测 feature 的原生构建，仍 not verified；本修复无探测行为变更。修复 PR 的独立 CR/远端 CI 与提交级人类审批尚待完成。

## PR 50 合并后的窄窗与键盘核对

PR 50 已合并为 `a8a5fae3a1806c71c5fcb03e1644efbaa147e6be`；其最终候选 `22ba370509746793bc8d52c7c5bfb0ea3a91e990` 与合并提交的 `src` / `src-tauri` 无差异。本轮继续使用该候选构建的本地 `.app`，没有更改业务代码。PR 的 macos CI 成功；维护者本人已补充最终候选审批。历史段落中的“候选尚未合并”仅描述当时状态。

- 将原生 Workbench 手动拖小，窗口截图为 906×706 像素。月份导航与日期区域可独立滚动到顶部/底部；底部设置轨道保持可见并可进入 Settings。滚动位置在缩小时可能不在顶部，但向上滚动可完整到达月份控制，不存在导航永久不可达。
  [窄窗顶部](workbench-native-narrow-top.jpg)、[窄窗底部](workbench-native-narrow-bottom.jpg)、[窄窗设置](workbench-native-narrow-settings.jpg)。
- 在 Home 按 Tab，原生 AX 焦点到“设置”按钮；按 Space 后进入 Settings → Quick Panel，验证键盘入口。其他焦点顺序与所有控件的完整键盘矩阵未扩大为通过。
- Workbench 前台及 Finder 前台分别通过工具发送 ⌥Space，读取结果均仍为 Workbench。工具对 app/window 的激活与选取可能干扰面板，不能将其断言为产品快捷键失败，也不能标记 Quick Panel → ⌘, 集成为通过。此项需要维护者手动确认。
- 完成后 ⌘Q 退出并从同一路径重启，回到 Home 当前月及等距大窗口；没有残留测试的小窗口尺寸，没有改系统设置。

剩余实质验收：Quick Panel → Settings 原生整合，以及减少动态模式的无位移/淡入视觉确认。已向维护者提供精确步骤，未收到结果前保持 not verified。根据[原生阶段验收决议](https://github.com/NibbleOrg/Nibble-desktop/issues/21#issuecomment-6854118028)，Apple Silicon、多显示器、Space/全屏及动态短录屏仍是整体原生体验的阶段门槛，本机 Intel smoke 不能替代。这些环境缺项不阻塞继续开发，但不能宣称整体原生验收通过，也不由 Agent 静默豁免。

合并后 CI 回读：[PR 50 合并提交 main run](https://github.com/NibbleOrg/Nibble-desktop/actions/runs/43984425243) success；[PR 49 合并提交 main run](https://github.com/NibbleOrg/Nibble-desktop/actions/runs/43913987099) success。此前 queued 记录保留为历史快照，不再代表最新运行状态。

## 维护者手动验收补充（2026-11-17）

在上述 PR 50 合并后本地应用的两项指定步骤中，维护者明确回复“两项均通过，已恢复系统设置”：

1. 浏览到下个月后，⌥Space → ⌘, 进入 Settings → Quick Panel，返回 Home 保留所浏览月份：维护者实机确认通过。
2. 临时开启 macOS Reduce motion，切换月份/设置直接切换、没有位移或淡入：维护者实机确认通过；随后已恢复系统设置。

本段取代前文这两项“等待维护者确认”的当前状态；工具自身不能可靠观察面板的限制仍保留为历史事实。证据类型为维护者交互反馈，没有新增短录屏，不替代 Apple Silicon、多屏、Space/全屏及整体原生阶段矩阵，也不扩大为未测试环境的通过。Workbench 本机功能验收项至此已取得反馈，整体原生阶段仍有环境与证据缺项。
