# 原生窗口风险探测

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — 先行探测收尾与维护者反馈核对；正式窗口整体验收仍 not verified

对应[验证原生窗口关键风险](https://github.com/NibbleOrg/Nibble-desktop/issues/28)，验收依据[确定原生窗口体验与阶段验收](https://github.com/NibbleOrg/Nibble-desktop/issues/21#issuecomment-6854118028)。这是可复现的临时风险探测，不能作为完整窗口服务或通过验收的替代品。

## 先行探测收尾与交接

维护者要求本任务先行收尾，并明确允许材质在正式 UI 加入后再优化。本票结束的是隔离探测、已发现缺陷修复与后续约束交接；它不替代正式窗口服务和最终整体验收。以下未验证项继续由承接任务负责，不因此填成 passed。

| 项目 | 当前事实与证据 |
| --- | --- |
| 可复现探测 | [原生探测实现](https://github.com/NibbleOrg/Nibble-desktop/pull/37)已合并为 d62a557b823b187beffa3dd023d5022e8ff3e269，macOS CI 成功；保留显式 native-probe feature |
| 唤起/重开缺陷 | [重试与 Esc 修复](https://github.com/NibbleOrg/Nibble-desktop/pull/38)已合并为 b21ffcd3d5e912d04c38011e928aa472de9ed1fd，macOS CI 成功；维护者复测回复“可以” |
| 实机反馈 | 维护者说明另有 Apple Silicon Mac 与外接显示器，随后对测试指引反馈“没问题，但背景色目前看起来没有模糊感”；未提供每项运行的 OS/显示器配置输出或录屏，不把综合反馈改写为逐项证据 |
| 材质对照 | [临时诊断 PR](https://github.com/NibbleOrg/Nibble-desktop/pull/39)的开启/关闭对照反馈“两种没啥区别”；原生层结构检查存在不代表视觉模糊正确。根因未确认，不能归因为尚未加入 UI |
| 延期决定 | 按维护者“可以后续 UI 加上后再优化”的指示，材质视觉调优转入正式 UI 阶段；最终 Frosted pane 合同保留 |
| 临时诊断处置 | PR 39 已关闭且未合并，分支 codex/28-probe-material-diagnostics 与提交 257ee7a 保留供回查；其 ⌘5/6/7 对照菜单不在 main 中 |

承接责任：

- [实现应用生命周期与 Quick Panel 窗口协调](https://github.com/NibbleOrg/Nibble-desktop/issues/30)：正式双窗口、托盘/单实例/快捷键生命周期，重新验证焦点、Space/全屏、显示器恢复、顶部缩放和旧回调边界；不能直接将第三个探测窗口当作正式服务。
- [实现 Quick Panel 工具演示与开发隔离](https://github.com/NibbleOrg/Nibble-desktop/issues/32)：正式 UI 加入后承接材质缺失，使用实际内容、明暗文字/图案背景和系统减少透明度状态验证模糊与可读性，记录调优结果。
- [完成本地体验整体验收与实施交接](https://github.com/NibbleOrg/Nibble-desktop/issues/34)：收齐最终组合应用的 commit、OS/架构、显示器/Space/全屏配置、步骤、预期/实际、状态及动态证据；本票关闭不解除该任务的完整原生门禁。

每项完整证据不足时继续写 not verified。后续发现平台限制，仍需保留复现证据交维护者决策，不能静默常驻所有 Space、申请额外权限或取消模糊要求。本轮不再反复要求维护者测试临时探测；正式实现后在上述承接任务统一验收。

## 运行和隔离

```sh
pnpm dev:probe
# 或编译无公开安装包的本地可执行文件
pnpm build:probe
# 可选：将刚构建的探测可执行文件装入本地未签名 .app，方便原生 UI 工具识别
pnpm exec tauri bundle --bundles app --config '{"bundle":{"active":true}}'
```

开发命令在终端输出 `NATIVE RISK PROBE enabled`、几何请求/即时 API 读值、焦点激活请求返回值及 `PROBE FAILURE`。这些返回值不是实际焦点或动画正确性的证明；配合现场观察和录屏。不要把日志附带的屏幕坐标解释为用户内容，也不要记录应用身份或输入。

仅 macOS 的 `native-probe` feature 编译 Rust 探测模块；默认 `pnpm dev` / `pnpm build` 不启用。显式模式在原骨架双窗口之外创建第三个独立的 `native-probe` 窗口，避免临时代码变成正式 Quick Panel。菜单控制不增加 webview IPC 或 capability；探测窗口没有命令权限。前端通过独立 URL 显示标记明确的探测内容，URL 本身不能启用原生能力。

不接触剪贴板、辅助功能或录屏 API，不新增系统权限、存储或业务运算。原应用句柄只保留在当前会话内存；关闭后释放；输入不传到 Rust、不写日志，重新打开清空。录屏由验收者自行操作系统工具完成，不是应用能力。

## 操作

macOS 将首个应用菜单显示为 **Nibble Local**；其中包含以下探测操作。

1. `⌘1` 安排三秒后打开；立即切到目标应用，并把指针放在目标显示器上。延迟结束时记录当时前台应用，然后将探测窗口放在指针显示器 work area 顶部居中。打开期间再执行不会覆盖原应用；等待期间重复按键保留第一次的三秒截止时间，不重新计时；关闭会取消待执行请求。`⌘1` 是本应用菜单快捷键，不是全局快捷键：必须先激活 Workbench，再按一次并等待三秒。
2. `⌘2` / `⌘3` / `⌘4` 请求 650×264 / 650×480 / 650×620 逻辑尺寸，分别代表搜索/历史/Base64 的尺寸负载，不执行工具业务。按目标屏幕比例转换为物理像素；优先保留左上角，在工作区放不下时限制尺寸并调整位置。聚焦首个输入。
3. `⌘W` / `Esc` 隐藏并仅向最初应用发送一次激活请求。原应用是 Nibble 自身时显式聚焦 Workbench；原应用已退出时只隐藏，不激活替代应用。探测对照项 `⌘⇧W` 隐藏但不恢复；外部点击失焦也隐藏并清除恢复目标。
4. 打开期间每秒检查窗口是否完整包含在任一现有显示器 work area 内；超出时重新约束到可用显示器。小屏通过滚动到达底部输入，重新唤起按指针位置定位。
5. 应用菜单 Quit / `⌘Q` 退出探测。普通骨架窗口与正式生命周期不在本探测中实现。

尺寸直接切换，没有 200ms 过渡。此选择用于隔离原生尺寸风险，不能算作产品动画合同的静默降级。reduced motion 场景仍需实机记录系统设置和实际结果，后续正式动画实现需另行验证。

## 候选平台方案与限制

- Tauri `transparent(true)` 与 `HudWindow` / Active 原生材质；CSS 仅加暖色薄遮罩，不能把浏览器渲染当作跨应用模糊的证据。
- 借用 Tauri 提供的 NSWindow 指针，在主线程设置 `MoveToActiveSpace | FullScreenAuxiliary`，配合 always-on-top。没有设置跨 Space 常驻，也没有把这些标志的名称当成全屏实测结论。若失败，保留事实并回到决策，不能静默改成所有 Space 常驻。
- 使用 `NSWorkspace.frontmostApplication` 保存原应用，关闭时调用 `activateWithOptions`。这是待验证候选；系统拒绝或请求成功但未实际恢复均应记录。其弃用状态与未来 macOS 激活策略变化是后续实现风险。
- `set_size` 后 `set_position` 是两个原生调用。几何算法保持顶边不等于原生合成过程中没有抖动，必须录屏检查。即时读回也可能早于窗口系统完成调整。
- 显示器恢复采用一秒轮询，尚未证明断屏后的实际时序、焦点或不同 DPI 间的行为，正式服务应基于实测选择事件/轮询策略。

锁定版本源码是此次 API 使用依据。Tauri [2.12.1 发布说明](https://v2.tauri.app/release/%40tauri-apps/api/v2.12.1/)说明透明/全屏不再需要 macOS private API feature；本探测在锁定的 Tauri 2.12.5 下编译通过，没有启用该 feature。此事实不保证材质表现。

## 探测证据与后续验证矩阵

2026-11-15 有限 smoke 环境：Intel x86_64，macOS 26.7.1（25G241），现有桌面会话。源码身份以本文件引入提交为准；没有将未提交工作树冒充独立 commit 证据。UI 工具绑定本地 `.app`，观察到初始化 Workbench、延迟唤起后的原生探测窗口、首个输入获焦，以及 264/620 两种高度切换；截图像素尺寸分别为 1300×528、1300×1240。关闭后观察到 Workbench。没有形成可提交短录屏，也未核实显示器硬件/Space 配置，故这些只记作有限观察。

| 场景 | 步骤与预期 | 正式验收状态 |
| --- | --- | --- |
| 顶部锚定 | 连续 ⌘2/3/4，录屏与坐标证明顶边稳定、内容可达、主输入获焦 | not verified；只观察到高度切换/输入焦点 |
| 跨应用恢复 | Finder、编辑器等分别唤起，再 ⌘W；原应用获焦；重复唤起不覆盖目标 | not verified；尝试 Finder 场景，缺全局焦点与动态证据 |
| 外部点击/原应用退出 | 点击别的应用不抢回；原应用退出后仍可关闭、不激活替代目标 | not verified |
| 当前 Space | 在不同桌面唤起，仅当前桌面显示，切换桌面不常驻 | not verified |
| 全屏应用 | 在原生全屏应用中延迟唤起，可见可输入，关闭返回目标 | not verified |
| 材质 | 在明暗、文字和彩色背景前检查真正背景模糊、透明与可读性 | 延期至正式 UI；维护者反馈开启/关闭无明显差异，视觉未通过 |
| 多显示器/DPI | 指针切屏，每次唤起对应屏幕且尺寸一致 | not verified；维护者综合反馈正常，未附逐项物理多屏证据 |
| 断屏 | 面板位于外屏时拔线，恢复到剩余屏可见范围且可关闭 | not verified；有几何单测及综合反馈，缺逐项实机记录 |
| 小屏 | 小工作区打开三种尺寸，滚到底部输入，可关闭 | not verified；有尺寸约束单测及综合反馈，缺逐项实机记录 |
| 快速关闭重开 | 安排多次延迟打开、关闭，再开；过期请求不重开，输入为空 | not verified |
| Reduced motion | 开启系统减少动态效果后重复切换；无空间/交叉淡入动画 | not verified；综合反馈正常，探测本身无动画，正式动画仍需验证 |
| Apple Silicon | 在可用 Apple Silicon macOS 上重复所有场景 | not verified；维护者声明有可用 Apple Silicon 环境，未附逐项环境/证据 |

下表保留证据缺项，交接后由窗口协调和整体验收任务使用最终实现补齐。维护者的综合正常反馈不自动补齐每条证据。实机记录使用以下模板及实际 commit；动态场景附短录屏后才把相应项改为通过。缺项可继续开发，但不能宣布整体原生体验通过。

```text
场景：
Commit（git rev-parse HEAD）：
macOS（sw_vers）/ 架构（uname -m）：
显示器型号、分辨率、缩放、排列/Space/全屏配置：
Reduce motion 状态：
步骤：
预期：
实际：
结果：passed / failed / not verified
证据文件或链接（动态场景短录屏，勿含私人内容）：
限制与后续：
```

## 自动化、依赖与回退

先建立三条几何测试，`cargo test --lib --offline` 因缺少 `fit` 实现出现三条 E0425，随后实现并通过。覆盖负坐标 Retina 工作区顶部保持、断屏后小工作区约束、指针屏顶部居中。它们不证明真实窗口时序。现有元信息与前端合同测试继续通过。

本地已运行：`pnpm format:check`、`pnpm lint`、`pnpm typecheck`（含生成漂移）、`pnpm test`（3 前端 + 4 Rust）、`cargo test --manifest-path src-tauri/Cargo.toml --features native-probe --offline --lib`、探测 feature 的全 targets clippy、`pnpm build:probe`。最终改动后又完成格式检查、ESLint、探测 feature clippy/test 及探测构建；默认构建亦通过。初始实现的[远端 macOS CI 已通过](https://github.com/NibbleOrg/Nibble-desktop/actions/runs/43724543159/job/132843531697)（合并 PR 37）；后续修复的远端结果须独立核对。

新增可选直接依赖为 crates.io 的 `objc2 = 0.6.5`（MIT）、`objc2-app-kit = 0.3.2`（Zlib OR Apache-2.0 OR MIT）。二者版本此前已在 Cargo.lock 的 Tauri 传递依赖中，仅提升为探测 feature 的显式依赖；关闭 app-kit default features，只选择使用的绑定，没有增加解析包。原生指针借用隔离在一个有安全说明的同步主线程调用。没有声称完成漏洞或公开分发许可审计。

CI 增加探测 feature clippy/test/build；这些仍不能验证实机。回退可删除 feature、可选依赖、两个 Rust 探测文件、独立 React 探测页、脚本和 CI 对应步骤，并恢复默认入口；无数据格式或用户设置迁移。维护者需审阅窗口/AppKit 候选方案，正式实现另行抽取经过实测的约束，不直接复用整个探测模块。

## 验收期间发现的唤起缺陷

维护者报告：从 Workbench 按 `⌘1` 常需两次以上并等待较久；Esc 后难以重新唤起。因此不能依据先前有限 smoke 宣称此场景已通过。

修复前，原生 UI 回放曾连续两次观察到单次 `⌘1` 后等待 3.5 秒仍显示 Workbench；临时边界日志还记录到获焦后失焦并关闭。该观察本身不足以判断所有失焦的来源，未将它当成所有平台均相同的根因。

确定性复现提取到实际菜单及原生回调共用的 `probe_schedule`：重复请求重设 token 会使第一次截止失效；隐藏后的旧失焦回调也会无条件调用 close，取消新请求。对应回归测试先失败，再通过。修复使等待中的重试不延后截止，并仅在收到失焦事件时窗口仍可见才处理外部关闭；单测保留可见窗口失焦时调用 close 的路径。关闭前先取走恢复目标，避免 hide 产生的事件抢先清除它；返回 Nibble 自身时明确选择 Workbench 键窗口。

复现命令：`cargo test --manifest-path src-tauri/Cargo.toml --lib probe_schedule --offline`。覆盖重试不延后、旧截止不取消新请求、Esc 的隐藏事件不取消重开、实际外部失焦仍关闭。原生失焦事件用于关闭判断，不用可能滞后的焦点查询结果覆盖事件。它们不替代 Apple Silicon 原生重测。临时 `[DEBUG-28-open]` 日志及临时文件写入代码已移除；保留内容无关的“已安排/保留首次截止”提示，便于验收时区分未收到按键和仍在等待。

最终候选的 Intel 原生 UI 回放观察到：单次 `⌘1` 等待 3.5 秒后出现探测窗口；Esc 后立即重复 `⌘1`，同样能重新出现；首个输入获焦。外部点击后 AX 树仍报告探测窗口，当前未能据此可靠核对真实显隐，故外部点击的原生结果仍 not verified，不能引用单测作为通过证据。维护者随后复测回复“可以”；缺少逐项 OS/配置与短录屏，保留该反馈而不提升整体原生验收状态。

本次修复检查通过：format:check、lint、typecheck/绑定漂移、3 前端 + 8 Rust 测试、探测 feature clippy/test、默认构建及探测构建。新增 `objc2-app-kit/libc` feature 用于已有 `processIdentifier` 绑定；Cargo.lock 仅增加至既有 libc 包的依赖边，无新增版本或包。依赖许可与回退沿用上文；修复可回退本次提交，无持久数据变更。
