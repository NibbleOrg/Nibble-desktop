# Workbench 深色外壳与月历候选

Status: Draft
Owner: 项目维护者
Last verified: 2026-11-17 — 自动化、原生构建与浏览器窄窗核对；原生交互 not verified

对应[实现 Workbench 深色外壳与真实月历](https://github.com/NibbleOrg/Nibble-desktop/issues/31)。产品合同见[交互与视觉](../product/interaction.md)，使用入口见[用户指南](../product/user-guide.md)。本页为实现证据，不替代原生验收。

## 实现与边界

`src/calendar.ts` 只生成本地公历日期，使用本地正午避开夏令时午夜偏移。`CalendarView.tsx` 在 macOS 桌面从受限的 `calendar_preferences` 命令读取 Foundation 当前地区与用户周首日覆盖，固定用公历格式化；浏览器预览才使用 Intl locale 默认周首日。原合并版只读 WebKit 默认 locale，实机验收发现它忽略系统周首日覆盖，因此追加本次修复。每次窗口获焦、语言变化和每 30 秒刷新当前日期；浏览中的月份不会随午夜跳转。“今天”即时读取系统日期。

`App.tsx` 保留同一个 Calendar 实例，切换设置仅隐藏内容；月份没有持久化。沿用窗口服务隐藏不卸载的约定，完整重启重新创建 Home 当前月。现有设置导航请求及确认命令保持不变；原外壳没有新增 IPC；本次日期偏好修复新增的只读命令与权限见下文。Quick Panel 业务保持原有骨架。

`Workbench.module.css` 提供 Ink rail、底部设置、日期网格、200ms 路由淡入与月份方向交叉淡入；CSS 系统减少动态偏好与原生 snapshot.reducedMotion 均禁用动画。退出月份的视觉层对辅助技术隐藏，200ms 后清除；快速切换会替换退出层并取消旧计时，视觉精调仍待原生验收。小窗口允许内容区域独立滚动，导航轨道保持可达。没有事件、导入、工具编辑或未来模块占位。

## 验证证据

本机 Intel x86_64 / macOS；精确工具链与启动命令见[初始化](bootstrap.md)。日期单测先运行因缺 `calendar` 模块失败，实现后通过。

- `pnpm format:check`、`pnpm lint`、`pnpm typecheck`：通过（含 Rust fmt/clippy、生成绑定漂移）。
- `pnpm test`：10 项前端及 22 项 Rust 测试通过；追加原生设置导航与 reduced motion 参数测试后，`pnpm exec vitest run` 共 11 项通过。
- `pnpm build`：Tauri release 可执行文件构建成功，无公开安装包。
- 自动化覆盖闰年、相邻月、42 格、US/GB/SA 周首日、月份前后/今天、设置往返保留、重新挂载重置、跨午夜更新、原生设置请求确认与减少动态参数。
- 浏览器默认地区 en-US 的真实日期显示及 380×580 窄窗截图核对：六行日期、月份导航、今天及底部设置均可见。浏览器无原生窗口服务，不能验证原生显隐与 IPC。

![380×580 浏览器窄窗](evidence/workbench-calendar-narrow.jpg)

## 合并后原生核对

已在合并提交 `5bd3aa2` 的 Intel macOS 本地打包应用验证月份浏览、设置往返状态保留、关闭请求后重新激活状态保留及完整退出重启重置，具体边界、截图与剩余缺项见[原生验收记录](evidence/workbench-native-2026-11-17.md)。PR 已合并；原生完整矩阵仍未完成，不能把此前 draft 状态描述当作当前 PR 状态。

## 待实机验收

以下为原候选矩阵；已部分验证的项目以合并后记录为准，其余仍 **not verified**，不自评关闭议题：

1. `pnpm dev` 启动 Home 当前月，浏览后设置往返、关闭隐藏、托盘/Dock 重开保留路由和月份；⌘Q 后重启重置。
2. 从 Quick Panel 用 ⌘, 导航到 Settings → Quick Panel，返回 Home 保留月份；原生协调集成场景。
3. 系统不同地区、周首日覆盖、时区变化和午夜唤醒；至少核对周一与周日起始地区、非公历默认地区仍显示公历。
4. 原生窄窗口滚动、键盘焦点可见、底部设置可达；200ms 路由/月份运动及系统减少动态开关下无位移/淡入。
5. Apple Silicon 上的真实窗口截图/短录屏。远端 CI、独立 CR 尚待 PR 记录，均 not verified。

回退恢复本票前端、窗口布局代码与文档文件即可；无持久数据、迁移、原生权限或发布变更。

## 等距启动窗口（本地追加候选）

维护者在本地体验后确认：Workbench 四边留白像素一致，窗口约占显示器可用面积 80%，不采用宽高各 80%。可用区域扣除菜单栏和 Dock，窗口外框包含标题栏；阴影不计入。启动时以窗口所在显示器为准，无法取得时使用主显示器；宽高为 W、H，求等距留白 m 使 `(W−2m)(H−2m)=0.8WH`，取整到物理像素，因此面积有舍入误差。用户随后可自行缩放/移动，隐藏重开不会强制恢复初始大小；完整重启重新计算。不新增 IPC、权限或依赖。

实现位于 `window_geometry::workbench_frame`，由 `lifecycle::setup` 调用并复用同步 AppKit 外框设置；获取显示器/窗口失败沿用现有窗口故障报告，保留配置的默认窗口。先运行 `cargo test --manifest-path src-tauri/Cargo.toml --lib workbench_tests --locked` 因缺函数失败，实现后横屏、竖屏、负原点、Retina 尺寸及小区域四组数据通过等距和面积断言。开发进程已重新编译启动；原生窗口读取工具超时，维护者随后反馈“UI没问题”，记录为当前机器上的 UI 体验确认，不等于绑定提交的风险审批。多显示器、再次显示、系统减少动态及完整原生矩阵仍 **not verified**。追加后本地 11 项前端、23 项 Rust 测试及 format/lint/typecheck 通过；最终原生构建结果见 PR。

## 系统日期偏好修复候选

`calendar_preferences` 无输入，只返回 `locale`（语言/文字/地区组成的 BCP 47 标签）及 `firstWeekday`（Sunday=0），不读取事件、文件、剪贴板或任何日历账户。只授予 Workbench 的 capability，并由命令再次校验真实窗口 label，Quick Panel/探测窗口均不允许调用。NSLocale / NSCalendar 的 autoupdating current 对象包含用户偏好；地区变更存在 Foundation 进程缓存，本机需要完整重启应用才能完整更新地区格式。单独覆盖周首日可在重新激活后更新。不许诺运行中地区变化即时生效。

前端独立 hook 在初始加载、focus、languagechange、每 30 秒读取；请求序号拒绝迟到的旧结果，卸载后不写状态。初次读取失败就地报错，不把浏览器默认值伪装成系统偏好；刷新失败保留最后确认值并说明状态。Rust 生成 IPC 类型，前端再校验 locale 与 0–6 整数范围。

直接声明已由现有依赖锁定的 crates.io `objc2-foundation = 0.3.2`，启用 NSLocale/NSCalendar/NSString；锁文件只增加本包依赖边，没有新增包版本。来源/接口以本地锁定 crate 源码核对，许可 MIT，完整漏洞扫描 not verified。没有新增系统权限或公开发布。此项涉及 IPC/Capability 与依赖，需按工作流取得维护者本人绑定修复最终 SHA 的审批，原 PR 审批不覆盖追加修复。

自动化先红：`pnpm exec vitest run src/surfaces/CalendarPreferences.test.tsx` 发现原组件完全不读取系统偏好；实现后覆盖系统周首日覆盖/恢复、focus 刷新、旧响应拒绝及无效响应保留。原生复测与恢复记录见[验收记录](evidence/workbench-native-2026-11-17.md)。回退仅恢复本次命令、权限、hook 与 Cargo 依赖边；无持久数据或迁移。
