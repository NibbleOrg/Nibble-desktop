# 工程工作流

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — main 规则集已启用并完成 API 回读；详见协作设施

main 不直接推送工作。Agent 分支 `codex/<issue-number>-<short-slug>`，一分支一 issue 一逻辑变更；`research/<slug>` 为不合并的研究分支。

提交及 PR 标题：`<type>(<scope>): <imperative summary>`。type 为 feat、fix、refactor、test、docs、build、ci、chore、perf、revert；破坏变更加 ! 及 footer，模块 scope 在建立实现后稳定采用。squash 使用 PR 标题，main 留任务关联记录。

PR 链接 issue，说明行为、非目标、验证命令和结果、风险/回滚、文档影响；UI 附截图或录像，数据和发布附恢复信息。本地必要工作或实质验收未完成时保留 draft；draft 可在远端 CI 完成前开展 CR，评审与 CI 状态按[质量规范](quality.md#cr-与-ci-并行)分别记录。非阻塞建议及无关清理另行处理。

治理、架构、IPC/Capability、隐私/数据、破坏性迁移、安全、签名/更新及公开承诺需要人类批准；依赖变更审查来源、许可、供应链风险和回滚，不确定风险交维护者。普通 UI、内部重构、测试和文档按当前合并门禁执行。

共享 GitHub 身份期间，由维护者亲自写 `Human approval: approved @ <full-commit-sha> - <scope>`；Agent 不得代写或推断。批准绑定提交，新提交使旧批准失效，实质变更重新验证。存在第二独立维护者后改用正式 review。

## 暂停远端 checks 合并门禁（#47 提案）

维护者要求暂时暂停 PR 必需 checks 阶段：应用经批准的规则集后，远端 CI 继续触发，但不再要求 `macos` passed 才能合并；暂停生效期间，仅远端 CI 排队或未执行不要求维持 draft。PR 仍须完成 CR、本地适用检查及验收说明，缺少的验证明确写 not verified；已知代码、规格、安全、隐私和数据正确性缺陷仍阻塞合并。适用的提交级人类审批保留，CR 完成不等同审批或任务完成。

本提案只移除 `required_status_checks`，保留 active/main 范围、空 bypass、PR、讨论解决、禁止强推及删除规则；不更改 CI workflow。应用前取得维护者本人绑定最终配置提交的批准，保存旧规则并回读生效结果；实际状态见[协作设施](../engineering/collaboration-guardrails.md)。恢复时按同样治理审批流程恢复此前 `macos` / GitHub Actions App 15368 / strict 配置。暂停没有自动到期时间，恢复须明确执行并回读。

无绕过 issue、当前适用合并门禁、隐私及人类批准的紧急路径；事故使用精简 issue/PR。模板及 main 保护的实际状态、配置和启用步骤见[协作设施](../engineering/collaboration-guardrails.md)。


来源：[#4](https://github.com/NibbleOrg/Nibble-desktop/issues/4)
