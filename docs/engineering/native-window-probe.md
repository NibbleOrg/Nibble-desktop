# 原生窗口风险探测

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — Intel 原生材质结构检查；视觉模糊及完整原生验收 not verified

对应[验证原生窗口关键风险](https://github.com/NibbleOrg/Nibble-desktop/issues/28)，验收依据[确定原生窗口体验与阶段验收](https://github.com/NibbleOrg/Nibble-desktop/issues/21#issuecomment-6854118028)。这是可复现的临时风险探测，不能作为完整窗口服务或通过验收的替代品。

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

## 实机记录与待验证矩阵

2026-11-15 有限 smoke 环境：Intel x86_64，macOS 26.7.1（25G241），现有桌面会话。源码身份以本文件引入提交为准；没有将未提交工作树冒充独立 commit 证据。UI 工具绑定本地 `.app`，观察到初始化 Workbench、延迟唤起后的原生探测窗口、首个输入获焦，以及 264/620 两种高度切换；截图像素尺寸分别为 1300×528、1300×1240。关闭后观察到 Workbench。没有形成可提交短录屏，也未核实显示器硬件/Space 配置，故这些只记作有限观察。

| 场景 | 步骤与预期 | 正式验收状态 |
| --- | --- | --- |
| 顶部锚定 | 连续 ⌘2/3/4，录屏与坐标证明顶边稳定、内容可达、主输入获焦 | not verified；只观察到高度切换/输入焦点 |
| 跨应用恢复 | Finder、编辑器等分别唤起，再 ⌘W；原应用获焦；重复唤起不覆盖目标 | not verified；尝试 Finder 场景，缺全局焦点与动态证据 |
| 外部点击/原应用退出 | 点击别的应用不抢回；原应用退出后仍可关闭、不激活替代目标 | not verified |
| 当前 Space | 在不同桌面唤起，仅当前桌面显示，切换桌面不常驻 | not verified |
| 全屏应用 | 在原生全屏应用中延迟唤起，可见可输入，关闭返回目标 | not verified |
| 材质 | 在明暗、文字和彩色背景前检查真正背景模糊、透明与可读性 | not verified；只观察到浅色探测窗口 |
| 多显示器/DPI | 指针切屏，每次唤起对应屏幕且尺寸一致 | not verified；缺物理多屏场景 |
| 断屏 | 面板位于外屏时拔线，恢复到剩余屏可见范围且可关闭 | not verified；只有几何单测 |
| 小屏 | 小工作区打开三种尺寸，滚到底部输入，可关闭 | not verified；只有尺寸约束单测 |
| 快速关闭重开 | 安排多次延迟打开、关闭，再开；过期请求不重开，输入为空 | not verified |
| Reduced motion | 开启系统减少动态效果后重复切换；无空间/交叉淡入动画 | not verified；探测本身无动画 |
| Apple Silicon | 在可用 Apple Silicon macOS 上重复所有场景 | not verified；当前主机是 Intel |

每条实机记录复制以下模板，必须使用实际 commit；动态场景附短录屏后才能把相应项改为通过。缺设备可继续开发，但不能宣布整体原生体验通过。

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

最终候选的 Intel 原生 UI 回放观察到：单次 `⌘1` 等待 3.5 秒后出现探测窗口；Esc 后立即重复 `⌘1`，同样能重新出现；首个输入获焦。外部点击后 AX 树仍报告探测窗口，当前未能据此可靠核对真实显隐，故外部点击的原生结果仍 not verified，不能引用单测作为通过证据。Apple Silicon 上的原始报告需维护者复测；无短录屏，不提升整体原生验收状态。

本次修复检查通过：format:check、lint、typecheck/绑定漂移、3 前端 + 8 Rust 测试、探测 feature clippy/test、默认构建及探测构建。新增 `objc2-app-kit/libc` feature 用于已有 `processIdentifier` 绑定；Cargo.lock 仅增加至既有 libc 包的依赖边，无新增版本或包。依赖许可与回退沿用上文；修复可回退本次提交，无持久数据变更。

## 材质缺失报告与对照诊断

维护者在 `b21ffcd3d5e912d04c38011e928aa472de9ed1fd` 复测后反馈其他本轮项目没问题，但背景看不出模糊。缺少明确纹理背景的对照和逐场景录屏，材质项不能升级为通过；也未据简短反馈补写设备/OS 等环境事实。

本轮没有复现出可判定的视觉失败，未完成视觉症状的自动红/绿环路，**没有宣称模糊已修复**。Intel macOS 26.7.1 的原生只读检查显示：窗口 `opaque=false`，视图树包含一个非零尺寸 Active / BehindWindow 的 HudWindow 材质，原生 WebView 视图非 opaque，系统 `reduce_transparency=false`。这些不能证明 CSS 绘制、跨应用合成及屏幕实际模糊正确，也不能替代 Apple Silicon 上的报告。

### 同一窗口 A/B

运行 `pnpm dev:probe`，用 ⌘1 打开，将有文字和图案的测试窗口放在后面：

- `⌘6`：移除原生材质，保留当前 CSS 及窗口尺寸。
- `⌘5`：恢复默认 HudWindow / Active 原生材质，保持 CSS 相同。
- `⌘7`：在启动终端输出原生材质数量、窗口 opaque、视图 opaque 计数和系统减少透明度状态，不记录应用身份、窗口标题、输入、屏幕内容或坐标。

请比较同一位置的文字/图案清晰度和颜色混合，记录“有明显差异 / 无差异”，保留两种状态的截图，结束时按 ⌘5 恢复默认材质。此为探测对照，不改变默认产品材质或降低既定模糊合同。没有自动上传图像或日志。若外部点击使面板隐藏，重新唤起后再用菜单切换，不点击后面的窗口。

如果两种状态都看起来相同，将 ⌘5 后的 ⌘7 输出附给诊断。不得仅因视图存在就断言视觉通过；不得把纯色背景或单独窗口捕获的浅色结果当作跨应用模糊证据。

### 可自动退出的原生结构检查

在已安装固定依赖的 macOS 检出中执行：

```sh
pnpm build:web
cargo run --manifest-path src-tauri/Cargo.toml --features native-probe,tauri/custom-protocol --locked --bin nibble-desktop -- --check-probe-material
cargo run --manifest-path src-tauri/Cargo.toml --features native-probe,tauri/custom-protocol --locked --bin nibble-desktop -- --check-probe-material-off
```

这两个命令使用同一个真实窗口构建路径，隐藏 Workbench，等待原生主循环后只读检查本应用的视图树并自动退出：开启时预期 1 个指定材质视图，关闭时预期 0 个；不透明窗口/材质结构不符返回 2，匹配返回 0。视图 opaque 计数仅为诊断信息，局部不透明控件不等于整张网页遮住材质。退出 0 仅表示结构检查匹配，**不是视觉模糊验收通过**。

本机两次命令均退出 0，开启输出 `effect_count: 1, active_hud_behind_window: 1`，关闭输出二者均为 0；两次 `window_opaque: false, opaque_content_views: 0, reduce_transparency: false`。未复现用户机器上的视觉缺失，不能据此指定根因或作视觉修复承诺。

系统布尔值来自 AppKit 的只读 [accessibilityDisplayShouldReduceTransparency](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducetransparency)，不修改用户设置。[NSVisualEffectView 文档](https://developer.apple.com/documentation/appkit/nsvisualeffectview)说明外观还取决于材质、混合模式和系统设置。`NSVisualEffectView` / `NSAccessibility` Cargo feature 只启用现有锁定 objc2-app-kit 的所需绑定，无新增包、版本或运行时权限。默认未启用 native-probe 的启动不执行检查或显示材质对照菜单。
