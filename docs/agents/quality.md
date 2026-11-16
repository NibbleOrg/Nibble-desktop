# 验证、评审与完成

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — CR/CI 并行策略候选核对；生产实机验收未验证

PR 适用检查为格式、lint、类型、单元测试和构建；行为变更提供 test-first red/green，bug 先建立失败复现。跨 UI、IPC、持久化需契约/集成验证，平台功能需实机 smoke，发布按[运维门禁](../operations/release.md)执行。

工程骨架已提供检查命令与 macOS CI 配置，具体命令和验证范围见[初始化文档](../engineering/bootstrap.md)；CI 配置存在不等于远端运行已通过。ready issue 必须给出具体合同及风险覆盖。未运行写 not verified，记录系统和步骤，不能表示通过。外部故障记录原因、替代证据和后续任务，必需检查不能因故障被称为通过。

## CR 与 CI 并行

本地适用检查及变更说明齐备后即可对 draft PR 开始代码评审（CR），同时等待远端 CI；runner 排队或 CI 尚未完成不阻塞评审开始及代码发现的交付。无法本地执行的检查列明原因与 not verified，可评审可见内容，并明确结论覆盖范围。

评审报告分别列出代码发现、本地验证及远端 CI 状态，绑定所评审的提交。queued、in-progress、失败或未执行的 CI 如实记录，不表示验证通过；排队本身不是代码缺陷。评审完成后，Agent 可交付结论与未验证清单，不必仅为等待 runner 持续占用当前任务或反复重跑。需要跟踪运行完成时另按维护者要求执行。

CI 后续失败须判断原因并处理已知缺陷；runner 排队或基础设施未执行按 not verified 记录，合并是否要求远端检查成功由当前工作流门禁决定；修复产生实质变更时重新评审受影响部分，更新提交绑定的证据。CR 完成与可合并、任务完成分别判断；合并条件仍遵守[工作流](workflow.md)，本地必要工作或实质验收未完成的 PR 继续保留 draft；checks 门禁暂停生效后，仅远端 CI 尚未运行不构成 draft 或合并的独立阻塞。

评审分别判断规格符合性和工程标准。发现标 Blocking 或 Non-blocking；正确性、规格、安全、隐私、数据完整性和发布风险未解决则阻塞，风格偏好为非阻塞。

完成须验收满足、证据可复现、必要测试和相关规范文档更新、术语/必要 ADR 更新、无秘密或私有数据、明确发布影响；普通 issue 不能仅凭 Agent 自评关闭，需 PR 合并证据。


来源：[#4](https://github.com/NibbleOrg/Nibble-desktop/issues/4)、[#8](https://github.com/NibbleOrg/Nibble-desktop/issues/8)、[#10](https://github.com/NibbleOrg/Nibble-desktop/issues/10)
