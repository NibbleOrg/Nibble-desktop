# 验证、评审与完成

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

PR 适用检查为格式、lint、类型、单元测试和构建；行为变更提供 test-first red/green，bug 先建立失败复现。跨 UI、IPC、持久化需契约/集成验证，平台功能需实机 smoke，发布按[运维门禁](../operations/release.md)执行。

实现尚不存在，当前无生产检查命令或 CI；ready issue 必须给出具体命令、合同及风险覆盖，建立模块时声明命令与矩阵。未运行写 not verified，记录系统和步骤，不能表示通过。外部故障记录原因、替代证据和后续任务，必需检查不能因故障被称为通过。

评审分别判断规格符合性和工程标准。发现标 Blocking 或 Non-blocking；正确性、规格、安全、隐私、数据完整性和发布风险未解决则阻塞，风格偏好为非阻塞。

完成须验收满足、证据可复现、必要测试和相关规范文档更新、术语/必要 ADR 更新、无秘密或私有数据、明确发布影响；普通 issue 不能仅凭 Agent 自评关闭，需 PR 合并证据。


来源：[#4](https://github.com/NibbleOrg/Nibble-desktop/issues/4)、[#8](https://github.com/NibbleOrg/Nibble-desktop/issues/8)、[#10](https://github.com/NibbleOrg/Nibble-desktop/issues/10)
