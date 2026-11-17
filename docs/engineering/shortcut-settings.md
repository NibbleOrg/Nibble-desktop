# 全局快捷键设置候选

Status: Draft
Owner: 项目维护者
Last verified: 2026-11-17 — 实施候选与自动化检查；原生保存、冲突和重启 not verified

对应[实现全局快捷键设置与可靠持久化](https://github.com/NibbleOrg/Nibble-desktop/issues/33)。默认 Alt+Space；Workbench Settings 可编辑真实快捷键、查看注册状态、完整参考及应用 Cargo 包版本。本阶段仅显示 Quick Panel 与关于，不提供捕获、保留或更新配置。工具内容继续是演示，不读写系统剪贴板。

## 格式、路径与恢复

Rust 使用 Tauri `app_config_dir()` 下的 `settings.json`，macOS 当前应用标识对应 `~/Library/Application Support/org.nibble.desktop.local/settings.json`。文件仅包含 `{"version":1,"shortcut":"Alt+Space"}`；实际保存使用快捷键库的规范化名称。无历史数据库、密钥、工具草稿或 UI 路由。缺文件使用默认；读取限制 4096 字节，拒绝未知字段、未知版本及无效快捷键。快捷键须包含 Alt、Control 或 Super 至少一个修饰键，以及一个合法主键；Shift 可组合。组合是否被系统占用由注册结果决定。

不可读、损坏或不兼容配置：尝试默认快捷键，明确显示恢复提示；启动不覆盖原文件。冲突只使快捷键不可用，Workbench 和托盘入口保留。用户可改组合并保存；“使用默认组合”只填入草稿，仍需保存。服务初始化失败须重启应用。

持久化使用同目录 `settings.json.tmp`、独占新建（Unix 权限 0600）、写入及文件 sync、rename 替换、父目录 sync。所有修改在主线程串行执行；残留临时文件不作为配置读取，写入前删除，不能跟随临时符号链接。断电发生在替换前保留旧文件，替换后可能为新文件；目录同步失败报告保存失败并尝试恢复。保证依赖本地文件系统的 rename/sync 语义，不承诺外部进程同时编辑配置时的事务隔离。

变更顺序：校验 → 注销旧注册（若有效）→ 注册新组合 → 原子保存 → 发布有效快照。注册失败尝试重新注册旧组合；保存失败移除新组合、重新注册旧组合并重新保存旧配置。恢复任一步失败返回 `recoveryFailed`，不显示成功；保留当前实际注册可用性，磁盘状态视为不确定，重新成功保存才能清除此状态。底层 macOS 库注销失败可能丢失句柄，故注销任一组合失败返回 `restartRequired`，禁止本进程继续修改，须完整退出重启清理系统注册；旧组合状态不确定时禁用其回调。回调只响应当前有效配置，遗留的新注册不能触发面板。全部失败均可通过托盘进入设置。

## 模块、IPC 与传播

- `shortcut_settings.rs`：平台无关的启动与变更事务，可替换 Backend。
- `settings_store.rs`：大小/格式/快捷键校验及同步文件替换。
- `settings.rs`：主线程适配、实际插件注册与应用目录、窗口身份校验。
- `Settings.tsx`：编辑草稿、真实反馈、参考与关于；失败保留草稿。

`settings_snapshot` 与 `settings_save(shortcut)` 只授权 Workbench；ACL 与运行时检查均拒绝 Quick Panel、探测及未知窗口。前端不获文件权限或插件注册权限。Rust 类型生成到 `bindings.ts`。错误仅为稳定类别，不透传文件路径、用户内容或底层错误文本。

设置状态更新复用 `lifecycle-version`：通知只带版本，两窗口先订阅再读取生命周期快照，Quick Panel 获知快捷键可用性；Workbench 设置读取须覆盖最新已知版本，旧响应不能覆盖新状态；重新聚焦重新读取。成功响应仅在实际注册和完整保存均成功时返回。重复保存当前有效组合仍执行持久化，以修复此前磁盘恢复故障。

新增直接依赖 `serde_json` 已是 Cargo.lock 内的传递依赖，本次不升级版本或新增解析包；本地 crate manifest 声明来源 crates.io、MIT OR Apache-2.0。依赖完整漏洞审计 not verified。回滚代码恢复固定快捷键路径，已有 JSON 留存且不再读取；不需历史迁移，不删除用户配置。

## 验证与剩余验收

自动化覆盖：注册及保存成功、注册拒绝、保存后回滚、旧注册恢复失败、旧注销失败、磁盘恢复失败、注销状态不确定时禁止重试并要求重启、原注册不可用时恢复、损坏启动回默认、启动冲突；真实文件替换及新 Store 读取、坏 JSON/版本/字段/组合、写入失败保留文件；窗口身份、生成命令参数、失败草稿与真实成功反馈、版本落后重读、StrictMode。

测试先记录 Rust 事务未实现的五项失败与前端缺少 Settings 模块的失败，再完成实现。检查结果以本 PR Evidence 为准。

原生 UI 工具能读取旧构建，但控制响应异常缓慢；尝试退出旧应用并绑定本次临时本地 `.app` 时失败，返回 NSCocoaErrorDomain 256 / NSOSStatusErrorDomain -600 `procNotFound`。没有以进程、打包成功或旧 UI 观察代替本次行为验收。本地 app 包只用于测试，不构成签名、分发或发布。

以下仍 **not verified**：本次设置页原生截图/窄窗视觉；在 Settings 保存新组合后立即从其他应用唤起/关闭；其他进程占用组合时的真实冲突和旧键恢复；真实文件系统写入/目录同步失败后的 UI 与注册恢复；完整退出、重新启动读取并注册新组合；损坏文件保留与托盘恢复的整体验证。文件及事务单测不代替跨进程和系统注册验收。前置窗口服务的原生缺项沿用其[证据记录](window-lifecycle.md)，不因此关闭前置或本任务。

后续实机步骤：保存 Control+Shift+Space，确认只有注册与保存成功才提示；从其他应用唤起并关闭；⌘Q 后重新启动确认组合和状态；以独立测试进程占用候选组合验证拒绝及旧组合仍可用；在备份配置后分别测试损坏文件和不可写目录，观察默认尝试、失败/恢复提示及托盘入口，最后恢复原配置与权限。不得在未备份真实配置时做故障注入。

## 独占注册后续候选

维护者在实施对话中明确要求“要求独占，其他应用已独占时保存失败并恢复旧键”。这替代先前默认依赖系统共享注册语义的候选；原有共享注册探测保留为历史证据，不视作新契约通过。产品行为见[交互契约](../product/interaction.md)。

`global-hotkey` 0.8.0 的 macOS RegisterEventHotKey 原本传 options=0；通过 Cargo 本地 patch 将该值改为 `kEventHotKeyExclusive`（1）。继续使用 Tauri 插件的同一事件与注册管理器，没有注册前探测/释放造成的竞态窗口。源码来自原锁定 crates.io 包，源归档 checksum、许可、精确差异和回滚步骤见 [补丁来源说明](../../src-tauri/vendor/global-hotkey/NIBBLE-PATCH.md)。媒体键使用共享 event tap，不满足独占，Rust 校验拒绝；未知媒体键旧配置按既有坏配置恢复规则尝试默认值。

原生回归例程 `src-tauri/examples/shortcut_exclusivity.rs` 在主线程创建管理器，以独立子进程占用 Control+Shift+F18，应用实际 Service 先注册 Control+Shift+F19，再改向被占用键。修复前断言失败（Ok 而不是 RegistrationFailed）；补丁后通过，旧注册及配置恢复，释放占用后可重新注册。存储适配为内存，避免修改用户配置；既有真实文件测试继续覆盖 JSON。执行命令：

```sh
cargo build --manifest-path src-tauri/Cargo.toml --locked --example shortcut_exclusivity
src-tauri/target/debug/examples/shortcut_exclusivity
```

该例程要求真实 macOS GUI 会话，沙箱不可用时不能把系统调用失败当作注册冲突通过；常规 cargo test 不自动占用用户快捷键。F18/F19 已被其他应用占用时先报告前置失败，不卸载其注册。子进程在完成或失败时退出释放测试注册。

已合并基线的设置保存、完整退出后读取、无效输入及恢复默认值已完成原生观察，详见[验收补充](https://github.com/NibbleOrg/Nibble-desktop/issues/33#issuecomment-6952224842)；本次补丁的 UI 冲突与恢复结果须另记，不复用旧构建的原生通过结论。

本次独占候选原生 UI 验收（macOS 26.7.1 / 25G241）：独立回归例程 `--hold` 占用候选键时，在新构建 Settings 保存 Control+Shift+F18，观察到系统拒绝提示、编辑草稿保留、当前有效值仍为已注册的 alt+Space，无保存成功提示。退出占用进程后，同一草稿重试成功，参考同步更新；随后恢复默认 alt+Space 并保存，辅助进程已结束。此证据覆盖真实系统拒绝及设置恢复、解除占用后重试；未扩大为文件系统故障注入、窄窗或从其他应用按键触发的直接观察。
