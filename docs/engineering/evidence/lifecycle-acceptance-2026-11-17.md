# 窗口协调验收进度

Status: Draft
Owner: 项目维护者
Last verified: 2026-11-17 — 合并基线自动化通过；本轮原生操作 not verified

对应[实现应用生命周期与 Quick Panel 窗口协调](https://github.com/NibbleOrg/Nibble-desktop/issues/30)。本页记录本轮证据；接口与历史修复见[窗口生命周期服务](../window-lifecycle.md)。这不是整体阶段通过记录。

## 基线与已运行检查

- 源码：`0bdab7f7cf9bd346073b2f2f0aa96352ebc6b145`，检查开始时工作区干净；本轮未修改业务代码。
- 环境：Intel x86_64，macOS 26.7.1（25G241）。
- `pnpm format:check`、`pnpm lint`、`pnpm typecheck`（含绑定漂移）通过。
- `pnpm test` 通过：13 项前端、23 项 Rust 测试。
- `pnpm build` 通过，生成本地 release 可执行文件；没有启动或观察该产物，不作为原生运行证据。
- 独立干净检出安装、隔离探测构建与本轮远端 CI：not verified。GitHub PR 列表查询连接失败，未据此判断 CI 或 PR 失败。
- 本轮没有可用的原生 UI 操作工具，没有新增截图或录屏；不以单测、构建或历史反馈替代实际场景结果。

## 原生验收待补

每条记录须包含实际源码 SHA、OS/架构、显示器/缩放/Space/全屏配置、减少动态状态、步骤、预期、实际及结果；动态场景附短录屏，避免私人内容。模板见[原生探测证据模板](../native-window-probe.md#探测证据与后续验证矩阵)。使用默认 `pnpm dev`，不要使用 `pnpm dev:probe`。

| 场景 | 操作与预期 | 本轮状态 |
| --- | --- | --- |
| 跨应用恢复及新会话 | Finder 与编辑器分别前台唤起；输入后用 Esc、⌘W、⌥Space 分别关闭；恢复最初原应用，打开/关闭不闪现，重新打开为空 | not verified |
| 快速切换与重开 | 切换两种工具尺寸，动画途中 Esc，立即重开并重复；旧完成/失焦不误关新面板，不永久锁住，顶边稳定 | not verified |
| 外部点击及目标退出 | 点击另一应用不抢回；原应用退出后关闭面板，不激活替代应用 | not verified |
| 设置转移 | 面板内 ⌘,；进入 Workbench 的 Quick Panel 设置，不回原应用 | 本轮 not verified；历史维护者确认见下文 |
| 驻留、单实例及完整退出 | 关闭 Workbench 后用托盘重开；第二次启动同一构建，无重复托盘/窗口，路由与月份保留；⌘Q 完整退出，再启动回 Home 当前月 | not verified；历史 Workbench 记录仅覆盖其中部分路径 |
| Space 与全屏 | 不同桌面、原生全屏应用中唤起；仅当前 Space 显示，可输入，关闭返回原应用 | not verified |
| 多屏、DPI、断屏及小屏 | 指针切屏唤起；面板在外屏时断开外屏；各尺寸控件可达，恢复到剩余可见工作区 | not verified |
| 减少动态 | 系统 Reduce motion 开启后切换面板尺寸；无空间及淡入动画，仍可关闭；测试后恢复系统设置 | not verified；Workbench 的减少动态反馈不覆盖面板 |
| 快捷键冲突与降级 | 制造默认键冲突后启动；显示真实失败原因，Workbench/托盘可用；不静默改键 | not verified；配置替换/保存恢复由快捷键持久化任务交付 |
| 临时演示暂停 | 托盘暂停/恢复与面板一致；完整退出重启恢复默认；不触碰真实剪贴板 | not verified；正式工具状态由 Quick Panel 演示任务交付 |

历史 [Workbench 原生记录](workbench-native-2026-11-17.md#维护者手动验收补充2026-11-17)已记录维护者确认面板进入设置并保留月份，以及 Workbench 减少动态通过；没有动态录屏，不能扩大为上述完整矩阵通过。

## 下游边界

- [Quick Panel 工具演示](https://github.com/NibbleOrg/Nibble-desktop/issues/32)尚未交付正式状态与材质对照；当前工具占位界面不作为该任务通过证据。
- [快捷键持久化](https://github.com/NibbleOrg/Nibble-desktop/issues/33)尚未交付修改、可靠保存与失败恢复；当前默认键注册不能替代验收。
- [整体验收与交接](https://github.com/NibbleOrg/Nibble-desktop/issues/34)须在最终组合提交重测集成场景并收齐证据。Apple Silicon、多屏、Space/全屏及动态证据缺项继续保留，不能宣称阶段通过。
- 本轮不关闭窗口协调任务、不改变产品承诺或降低验收门槛；未发现新的产品决策需要建立 Wayfinder 地图。

## 原生工具恢复后的续测

2026-11-17，在 `a78bceb` 上继续验收；相较上述 `0bdab7f` 仅增加本验收文档及链接，业务源码无变化。前文“没有可用工具”描述首轮状态，以下为后续实际尝试，不替代待补矩阵。

- 通过原生工具对旧 Nibble 执行 ⌘Q，返回 `App quit`。随后 `pnpm exec tauri build --bundles app` 通过，重新生成本仓库 `src-tauri/target/release/bundle/macos/Nibble Local.app`，再通过完整路径启动。
- 原生 AX 确认 Workbench 显示 Home、November 2026、今天为 17；页面 URL 为 `tauri://localhost/index.html?surface=workbench`。这是当前构建启动的有限证据，不是完整生命周期验收。
- 从 Workbench 发送工具键 `alt+space` 后，AX 及截图仍显示 Workbench；未观察到 Quick Panel。工具可能在读取时激活所绑定窗口，不能据此判定真实快捷键注册失败。
- 绑定 Finder 后再次发送 `alt+space`，返回 Quick Look 预览窗口的 AX（含 Exit Full Screen、Close、Share file），没有确认 Quick Panel。这里只记录工具实际结果，未确定工具输入、系统快捷键或产品行为中的原因，不把它记为通过或已复现的产品缺陷。
- 随后对 Finder 发送 Escape 并读取状态时，工具报 `com.apple.ScreenCaptureKit.SCStreamErrorDomain Code=-3811`（audio/video capture failure）。无法确认清理结果，未继续交互或通过其他输入技术绕过。
- 原生工具还出现长时间响应延迟。本轮没有取得跨应用焦点、关闭重开、Space/全屏、多屏或动态短录屏；这些项仍为 **not verified**。后续须由稳定工具或维护者逐场景补证据，不能用重建成功替代。

本轮仅补充上述有限观察；未修改系统偏好、快捷键配置或用户数据。原生工具截图已在验收对话显示，未新增仓库截图文件。
