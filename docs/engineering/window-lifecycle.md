# 窗口生命周期服务

Status: Draft
Owner: 项目维护者
Last verified: 2026-11-16 — 候选实现与自动化核对；原生实机验收 not verified

对应 [#30](https://github.com/NibbleOrg/Nibble-desktop/issues/30)，依据[原生决议](https://github.com/NibbleOrg/Nibble-desktop/issues/21#issuecomment-6854118028)及[探测交接](native-window-probe.md)。本页记录候选实现接口及验收缺项，不表示窗口体验已通过。产品合同仍见[交互](../product/interaction.md)。

后续合并基线的检查结果及原生待补清单见[2026-11-17 验收进度](evidence/lifecycle-acceptance-2026-11-17.md)；下文候选时期的记录保留为历史证据。

## 范围与入口

默认 `pnpm dev` / `pnpm build` 使用配置中的 Workbench 与 Quick Panel 两个窗口。启动显示 Home；Workbench 关闭事件被拦截为隐藏，不卸载 React。托盘提供打开 Workbench、打开 Quick Panel、演示暂停、设置及退出；菜单 ⌘Q 调用完整退出。Dock reopen 与官方单实例插件的第二实例回调只激活原有 Workbench，不重新加载其 URL。路由可在同次运行保留；实际月历及月份保留由 [#31](https://github.com/NibbleOrg/Nibble-desktop/issues/31)接入验证，本票不伪造月历验收。

默认全局 ⌥Space 开关同一个 Quick Panel。快捷键插件安装及注册均作为可降级服务初始化：失败在 Workbench 显示阶段、系统返回原因和托盘恢复路径，不静默改键，也不使托盘启动失败。配置、注册替换、持久化/回滚与重启验证由 [#33](https://github.com/NibbleOrg/Nibble-desktop/issues/33)交付；当前没有保存设置入口。托盘演示暂停只改变内存中的 `demoPaused`，不控制真实捕获，完整退出重置。后续工具 UI 可用受限命令消费这一状态。

当前 React 只有窗口服务所需的最小壳、空搜索及三种尺寸入口；工具页面明确标记业务尚未实现，不执行捕获、剪贴板读写、转换或工具成功流程。[#32](https://github.com/NibbleOrg/Nibble-desktop/issues/32)替换正式工具演示界面，承接参考层、工具键盘及材质视觉调优。

显式 `native-probe` feature 继续运行隔离的第三窗口探测，不安装正式生命周期服务及全局快捷键。共享的纯几何算法改名为 `window_geometry`，没有复用探测的临时菜单和三秒等待作为正式入口。探测不能用于替代本页的正式服务验收。

## 模块和线程

- `src-tauri/src/panel_session.rs`：会话/切换票据、关闭顺序；无 Tauri/AppKit 依赖。关闭使票据失效，重复切换不能延后当前截止时间。
- `src-tauri/src/window_geometry.rs`：物理像素工作区约束，正式服务和隔离探测共用。
- `src-tauri/src/lifecycle.rs`：macOS 主线程适配；保留本会话最初的 `NSRunningApplication`，管理托盘、原生显隐、尺寸及焦点。菜单、同步命令、窗口事件在主线程执行；插件回调和计时线程通过 `run_on_main_thread` 投递，不跨线程持有 AppKit 对象。
- `src-tauri/src/lifecycle_commands.rs`：从真实调用窗口取得身份，校验会话/路由规则；不接受客户端自报窗口名。
- `src/ipc/useLifecycle.ts` / `lifecycle-sync.ts`：先订阅通知再读快照，读期间收到较新版本时继续读；销毁后忽略迟到响应，窗口重新获焦时重读。React 按 session 挂载临时状态，关闭重开不能恢复旧搜索和输入。

新会话先记录前台应用，再定位/显示/聚焦；会话内已有打开请求不覆盖原应用。关闭先取走恢复目标并使会话及切换失效，再隐藏；返回外部应用时仅在隐藏调用期间临时禁止 Workbench 成为焦点回退窗口，调用结束即恢复其可聚焦状态，隐藏或抑制失败也执行恢复；Esc/⌘W/全局快捷键恢复仍存在的原应用一次，目标退出不激活替代应用。原应用为 Nibble 时明确显示并聚焦 Workbench。外部点击不恢复，设置跳转关闭面板后激活 Workbench。隐藏失败或系统拒绝恢复保留可见故障类别，不循环抢焦点。

原生失焦事件作为外部点击依据，只处理仍有活动会话且仍可见的窗口，不用可能滞后的 Tauri 焦点查询覆盖失焦事件。隐藏后的失焦不取消隐藏状态。快速隐藏/重开时 OS 事件与新激活事件的真实顺序仍需实机验证，不能仅凭票据单测保证原生事件来源可识别。

## IPC 和最小权限

类型由 Rust/Specta 生成到 `src/ipc/bindings.ts`，包括 `View`、`LifecycleError` 及快照。所有 session/request 均为 Rust 分配的非零递增 u32；零、旧值和并发切换被拒绝或忽略，无客户端选择任意窗口、坐标或任意脚本的入口。

| 命令 | 调用窗口 | 用途 |
|---|---|---|
| `lifecycle_snapshot` | 两窗口 | 读取版本、会话、view、transition、导航请求、快捷键注册状态/失败原因、故障、减少动态及演示暂停；不含草稿或原应用身份 |
| `panel_close(session)` | Quick Panel | 仅关闭当前会话并请求恢复焦点 |
| `panel_transition(session, view)` | Quick Panel | 搜索与单个工具之间切换；禁止工具直接互跳；返回权威快照 |
| `panel_transition_complete(session, request)` | Quick Panel | 完成匹配当前会话及切换的动画，旧通知无副作用 |
| `panel_settings(session)` | Quick Panel | 关闭当前面板并请求 Settings → Quick Panel |
| `panel_toggle_demo_pause(session)` | Quick Panel | 与托盘共享的临时演示暂停；不执行捕获 |
| `settings_acknowledged` | Workbench | 消费设置导航请求 |

`lifecycle-version` 通知只含递增版本。两个 capability 只授予对应命令及 `core:event:allow-listen/unlisten`，不授予前端 emit、通用窗口控制或插件快捷键注册权限。ACL 外，命令还独立核对实际窗口 label；探测窗口不获这些 capability。没有文件、剪贴板、shell、录屏或辅助功能权限。`NSAccessibility` 编译 feature 仅用于读取系统减少动态偏好，不申请辅助功能授权。

稳定故障类别为 Rust `LifecycleError`；普通底层窗口错误不把原始对象或内容送到 UI。默认快捷键固定为 ⌥Space，其插件初始化/注册错误只在内存快照中显示系统原因，不记录用户输入、原应用身份或持久保存。

## 尺寸、动画与恢复

每次唤起按指针屏幕工作区顶部居中。搜索/历史/Base64 的逻辑高度为 264/480/620，宽度 650；仅为正式 UI 接入前的尺寸负载。所有尺寸限制在工作区内，小屏通过文档滚动到达控件。

Rust 按单调时钟做 200ms 尺寸插值，React 处理内容淡入及主控件焦点；几何尽量保持左上角，工作区容不下时约束位置。减少动态时直接调整尺寸，禁用内容淡入。动画通知匹配 `(session, request)` 且原生尺寸已完成时才解锁；750ms watchdog 保证 CSS 动画、完成通知或前端异常不会无限锁住。重复切换请求被拒绝，不刷新截止；关闭始终可用。计时线程仅在切换期间约 16ms 唤醒，空闲每秒检查显示器；活动面板不在任何现存工作区内时重新约束。没有将这种候选轮询策略当成已验证的断屏时序。

正式服务在 macOS 主线程用单次同步 `NSWindow.setFrame:display:` 应用完整矩形，转换以当前原生 frame 和物理位置/缩放为基准。锁定版本 Tao 的 `set_size`/`set_position` 会再次异步投递，不能当作显示前定位的完成屏障；先行探测仍保留其原有两调用实现。同步应用 frame 仍不证明合成阶段无抖动；当前 Space/全屏采用 `MoveToActiveSpace | FullScreenAuxiliary`，不设置所有 Space 常驻。实机不符合合同必须保留证据并修复或重新决策，不能静默放宽。

## 自动化和实机证据

本地环境：Intel x86_64 / macOS 26.7.1 (25G241)。工作树候选检查不冒充 Apple Silicon 或已合并提交的原生证据。最终 SHA 与 CI 结果以本票 PR head 为准。

先红后绿记录：

- `cargo test --manifest-path src-tauri/Cargo.toml --lib panel_session --offline`：先因缺 `Session` 出现四处 E0433；实现后四项通过，覆盖重开拒绝旧请求、重复请求、旧完成及超时票据不能完成新切换。
- `cargo test --manifest-path src-tauri/Cargo.toml --lib close_tests --offline`：先因缺 `close_with_restore` 出现 E0425；实现后四项通过，覆盖先取恢复目标、隐藏失败不恢复、恢复失败仍保持关闭、外点/无目标不抢焦点。调用的就是原生适配共用的关闭顺序函数；不代表系统已实际恢复焦点。
- `pnpm exec vitest run src/ipc/lifecycle-sync.test.ts`：先因模块缺失失败；实现后覆盖快照读取期间通知竞态和销毁后迟到响应。
- 前端补充覆盖新会话清空/聚焦、切换期间 Esc 可关闭及销毁后取消完成通知；Rust 校验调用窗口身份矩阵。已有几何/探测回归保留。

本地适用检查由 `pnpm format:check`、`pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build` 运行；隔离模式另外运行 feature clippy/test 和 `pnpm build:probe`。本候选上述检查均通过：8 项前端、17 项 Rust 测试，默认及探测原生构建成功；feature clippy/test 通过。另完成变更文档本地链接、`git diff --check` 及直接依赖 metadata 来源/许可核对。远端 CI 按 PR 实际记录，不能用构建替代下列原生门禁。

本轮 Computer Use 未获 Nibble Local 操作授权，随后工具不可用；没有通过替代输入工具绕过，也未取得候选 UI 截图/短录屏。所有正式服务原生场景均为 **not verified**，PR 保持 draft。可在最终候选 SHA 的 Apple Silicon 实机上执行：

```sh
git rev-parse HEAD
sw_vers
uname -m
pnpm install --frozen-lockfile
pnpm dev
```

记录显示器型号、缩放/排列、Space/全屏与减少动态状态；动态项目附无私人内容的短录屏，模板沿用[探测证据模板](native-window-probe.md#探测证据与后续验证矩阵)。不要再使用 `pnpm dev:probe` 检验正式服务。

| 场景及步骤 | 预期 | 状态 |
|---|---|---|
| 启动、切设置、关 Workbench、托盘打开；再次启动同一应用 | 启动 Home；隐藏后后台继续，路由保留；无第二组窗口/托盘；月份在 #31 接入后补测 | not verified |
| 外部测试应用中 ⌥Space，输入，再次 ⌥Space 关闭，再打开 | 首次搜索获焦、关闭恢复原应用、新会话为空；无须等待三秒 | not verified |
| Esc/⌘W、切换动画中关闭、立即重开、多次重试 | 不卡住，旧完成/失焦不关闭新会话，最初原应用不被 Nibble 覆盖 | not verified |
| 外部点击、原应用退出、⌘, 跳设置 | 外点不抢回；目标退出仍关闭；设置激活 Workbench，不回原应用 | not verified |
| 不同 Space、全屏测试应用分别唤起/关闭 | 仅当前 Space；可见可输入；不跨 Space 常驻 | not verified |
| 不同屏幕/DPI 上指针唤起，打开后拔除外屏、重接再开 | 对应指针屏幕；断屏约束到剩余可见工作区；控制可达 | not verified |
| 连续搜索/两工具/返回，小工作区，减少动态开/关 | 顶边稳定、滚动可达、切换后主控件获焦；减少动态无空间/淡入动画 | not verified |
| 制造默认快捷键冲突，再启动 | 明确系统原因；Workbench 与托盘仍可用；修改恢复由 #33 补测 | not verified |
| 托盘暂停/恢复、⌘Q、完整重启 | 仅临时演示暂停，退出结束服务和托盘；重启回 Home/默认演示状态 | not verified |
| 正式内容与明暗文字/图案背景的材质对照 | 由 #32 优化并验证真实模糊/可读性，#34 收齐组合证据 | not verified |

## 依赖、审阅与回退

按已确认[工具链决议](https://github.com/NibbleOrg/Nibble-desktop/issues/23#issuecomment-6854590288)使用官方 Rust 插件；不增加 JS 插件 SDK 或前端插件权限。直接新增 crates.io `tauri-plugin-single-instance = 2.5.5`、`tauri-plugin-global-shortcut = 2.4.1`，均 Apache-2.0 OR MIT，提交完整 Cargo.lock。已有 `objc2 = 0.6.5`（MIT）、`objc2-app-kit = 0.3.2`（Zlib OR Apache-2.0 OR MIT）由探测可选依赖改为 macOS 正式适配依赖；Tauri 打开 `tray-icon`。锁文件新增的跨平台传递依赖含 Linux/Windows 专用包，不视为这些平台已实现或获得支持。Rust 漏洞库扫描和公开分发许可审计 not verified。

API 依据为锁定版本源码及官方[单实例](https://v2.tauri.app/plugin/single-instance/)、[全局快捷键](https://v2.tauri.app/plugin/global-shortcut/)、[托盘](https://v2.tauri.app/learn/system-tray/)文档；源码核对确认单实例应先安装，快捷键插件自己的 manager 初始化也可能失败，因此本候选在托盘建立后动态安装并捕获该失败。

本变更涉及架构、IPC/Capability、原生窗口和依赖，须按[工作流](../agents/workflow.md)取得维护者本人绑定最终 SHA 的批准，不能由 Agent 代写。完成原生验收、适用 CI 及审批前不合并、不关闭 #30。不新增 ADR：沿用已确认的 Rust 核心协调与平台隔离决策，没有改变产品或权限承诺。

回退恢复本票之前的 Rust/前端入口、两端权限及 Cargo 清单/锁文件。纯内存生命周期，无数据迁移或删除、无设置文件写入；完整退出释放默认快捷键及单实例资源。正式服务回退后仍可显式使用先行探测，但不能声称产品生命周期可用。本次无公开发布、签名或更新交付。

## Esc 关闭闪现回归（#30 后续候选）

维护者在 PR 43 合并后报告：Workbench 保持打开、Finder 在前台，从 Finder 唤起 Quick Panel，再 Esc 返回时出现居中/闪现。原生操作接口不可用，采用只记录静态窗口事件、会话号、矩形和焦点标志的本地临时跟踪；未记录输入或原应用身份，日志未上传。

两项独立问题得到记录：

- 打开位置：原生 frame 在 `panel.geometry.ready` 和显示/获焦时仍为 `(1179,1048,650,264)`，稍后才成为顶部 `(1179,1374,650,264)`（AppKit points）。源码核对 Tao 0.37.2 的尺寸/位置 setter 使用 `DispatchQueue::main().exec_async`；即使主线程调用也会排队。主线程同步设置完整 frame 后，维护者反馈打开不再闪现，关闭仍闪现。
- 关闭焦点：隐藏返回时 Workbench 原生 `isKeyWindow=true`，随后再切回外部应用；没有主界面重新打开或恢复目标误记为本应用的事件。候选仅在返回外部应用的隐藏调用期间禁用 Workbench 的 `focusable`，无论隐藏/抑制成功或失败都恢复；设置跳转、返回本应用与外部点击保持既有意图。维护者明确确认 Finder 在前台的路径关闭不再闪现；同轮跟踪中，外部起点会话的 `panel.hide.done` 时 Workbench 保持 `key=false`，原应用激活请求被接受。此为本轮反馈及事件证据，没有短录屏，仍不扩大为完整原生门禁通过。

先红后绿：新增坐标转换测试最初因缺 `native_frame` 出现 E0425；实现后覆盖 Retina 实测位置差与负坐标屏幕缩放顶边。实际事件校验先观察到“定位完成”和隐藏前 frame 不一致，再以原生读回比较验证同步生效。焦点回退三条测试先失败，随后使用与原生适配共用的 `hide_without_fallback` 覆盖关闭时不能选中 Workbench、隐藏失败恢复可聚焦状态、抑制失败仍隐藏并清理。

仅改变外部恢复的调用顺序以及仅禁用 Quick Panel 的 AppKit 自动显隐动画，均经维护者复测仍闪现，已撤回；它们的模型单测或编译结果不能当作修复证据。最终候选不保留这些无效改动。未新增 IPC、权限或依赖；其余原生门禁仍按上表保留。临时跟踪已在确认结果后从实现移除。新增 5 项回归，当前合计 8 前端与 22 Rust 测试；最终移除跟踪后的检查结果以修复 PR 为准。
