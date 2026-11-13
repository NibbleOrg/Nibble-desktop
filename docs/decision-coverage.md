# 决议覆盖与实施交接

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

## 覆盖矩阵

| Issue | 文档落点 |
|---|---|
| #1 | 本索引、文档入口与一致性记录 |
| #2 | 产品契约、交互；旧条款替代关系见一致性记录 |
| #3 | 架构平台边界、发布；研究证据仍见原 issue 链接 |
| #4 | 六份 Agent 指南、根 AGENTS、ADR 索引 |
| #5 | 交互与视觉、架构 |
| #6 | 产品契约、交互 |
| #7 | 隐私与数据 |
| #8 | 架构、ADR-0001 |
| #9 | 交互与视觉、原型引用 |
| #10 | macOS 发布与运维 |
| #11 | 项目说明、文档索引与权威指南 |
| #12 | Windows v4 规划 |
| #13 | Calendar 未来扩展契约 |
| #14 | 交互与视觉、原型引用 |

来源：各 issue 最终决议；[统一冲突口径](product/decision-reconciliation.md)经维护者确认。

## 尚待实施与决策

- 最小工程、检查命令及 macOS CI 配置已建立，验证证据见[初始化](engineering/bootstrap.md)；完整业务与平台验收尚未完成。
- issue/PR 模板、GitHub main ruleset/branch protection 尚未启用；代码和 CI 首次存在时建立治理 issue，要求 PR 和适用 CI，禁止直接推送。不声称正式 review 已被强制。
- macOS 真实权限、焦点、快捷键、窗口材质、功耗、安装与更新恢复均 not verified。
- 应用图标、实际本地数据路径、用户卸载步骤和真实发布下载入口待实施。
- Windows v4 前决策清单见[Windows 规划](operations/windows-v4.md)。
- Calendar 未来事件合同已留档，v1 不建设导入/事件系统。

## 本轮核对与完成条件

文档及 Agent 指南已生成；Calendar 英文详细合同保留于未来扩展页，中文主文档区分当前范围。内部链接与 diff 检查由本次工作执行。未执行任何生产或平台测试。

2026-11-13：维护者在对话中明确 review 通过，规范文档转 Current，ADR-0001 转 Accepted。23 份文档的内部链接、元数据唯一性、代码块和 diff 检查通过。当前不声称已提交、合并或批准到某个 commit；GitHub 提交级审批仍按工程工作流执行。
