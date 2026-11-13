# v1 决策一致性核对

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；未验证生产实现或平台行为

本记录服务于 [总蓝图 #1](https://github.com/NibbleOrg/Nibble-desktop/issues/1) 的文档整理。维护者已在对话中确认以下口径。文档已经维护者审阅通过，当前规范正文承载有效契约，issue 保留历史；本记录不代表功能已实现。

## 已有明确替代关系

| 历史条款 | 当前有效契约与来源 |
|---|---|
| #2 Quick Panel 无搜索 | #5 工具名称及别名搜索，不搜索 Clipboard Entry 内容 |
| #2 Workbench 管理历史及使用 Base64 | #6 两项能力只在 Quick Panel，Workbench 仅 Calendar Home 和应用 Settings |
| #2 单条删除 | #7 无单条删除，Settings 提供需确认的清空全部 |
| #2 Home 预留未来模块 | #6、#14 无可见占位模块，只有受限 Calendar |
| #7 不兼容历史可自动重置 | #10、#8 经用户确认后重建，拒绝时保留原数据 |
| #10 早期暂缓及系统版本待定 | #10 最终决议继续发布规划，Apple Silicon、macOS 14+ Preview |
| #14 字标尚待品牌决定 | #11 沿用 Nibble 和原型字标及视觉方向，应用图标为实施交付项 |

## 本轮确认的统一口径

### 捕获前提

只有用户已授权、未暂停且服务健康时，运行中的应用才自动捕获剪贴板。关闭 Workbench 不改变这些前提。首次未授权时不读取并保存历史；恢复捕获只处理后续变化，不补采暂停内容。#1、#2、#5 的“运行时自动捕获”均应结合 #7 的前提理解。

### Calendar 当前与未来范围

#13 中 Calendar Event、Provider、Source、手动导入和对账规则属于未来扩展契约。v1 只实现日期浏览和必要模块边界，不提前建设事件存储、授权或同步界面。术语保留，但用户文档和工程文档必须标明未来范围，不能呈现为现有功能。

### 导出边界

不提供用户业务数据的备份、导入导出或迁移功能。诊断包是明确例外：由用户主动导出、可预览，不含用户内容或密钥，由用户决定是否附到 issue，不自动上传。

### 本轮与实施交付

本轮交付规范文档、索引、必要 ADR、术语及 Agent 指南。CI、issue/PR 模板、分支保护及生产代码属于后续实施任务，须明确记录尚未启用，不能声称技术强制规则已经生效。#11 对 Agent 指南实际落地的要求细化 #4 的后续交付安排。

### 文档权威切换

整理期间规范文档为 Draft，以已确认的决议为依据。完成决议覆盖、冲突、引用和链接核对后，统一转为 Current，再关闭 #1。Last verified 表示对 v1 规划契约的核对，不代表生产实现或实机验证。

## 非冲突的交付状态

- 根 AGENTS.md 指向的六份 docs/agents/ 指南已补齐并经维护者审阅通过。
- README 已补充定位、状态与文档入口；实际安装下载链接待生产发布。
- Wayfinder 议题使用 wayfinder:*；普通工作 issue 使用 #4 的五种分流标签，二者不混用。
- Windows 是 v4 的 Windows 11 x64 目标，无日期承诺；具体最低版本、安装器、代码签名、默认快捷键和新增能力矩阵是未来决策。

## 来源

所有 issue 均位于 https://github.com/NibbleOrg/Nibble-desktop/issues/ 下：

- [#2 产品](https://github.com/NibbleOrg/Nibble-desktop/issues/2)
- [#4 治理](https://github.com/NibbleOrg/Nibble-desktop/issues/4)
- [#5 生命周期](https://github.com/NibbleOrg/Nibble-desktop/issues/5)
- [#6 Workbench](https://github.com/NibbleOrg/Nibble-desktop/issues/6)
- [#7 数据隐私](https://github.com/NibbleOrg/Nibble-desktop/issues/7)
- [#8 架构](https://github.com/NibbleOrg/Nibble-desktop/issues/8)
- [#9 Quick Panel 视觉](https://github.com/NibbleOrg/Nibble-desktop/issues/9)
- [#10 发布](https://github.com/NibbleOrg/Nibble-desktop/issues/10)
- [#11 文档与叙事](https://github.com/NibbleOrg/Nibble-desktop/issues/11)
- [#12 Windows](https://github.com/NibbleOrg/Nibble-desktop/issues/12)
- [#13 Calendar](https://github.com/NibbleOrg/Nibble-desktop/issues/13)
- [#14 Workbench 视觉](https://github.com/NibbleOrg/Nibble-desktop/issues/14)
