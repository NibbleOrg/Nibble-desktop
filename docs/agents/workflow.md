# 工程工作流

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

main 不直接推送工作。Agent 分支 `codex/<issue-number>-<short-slug>`，一分支一 issue 一逻辑变更；`research/<slug>` 为不合并的研究分支。

提交及 PR 标题：`<type>(<scope>): <imperative summary>`。type 为 feat、fix、refactor、test、docs、build、ci、chore、perf、revert；破坏变更加 ! 及 footer，模块 scope 在建立实现后稳定采用。squash 使用 PR 标题，main 留任务关联记录。

PR 链接 issue，说明行为、非目标、验证命令和结果、风险/回滚、文档影响；UI 附截图或录像，数据和发布附恢复信息。验收及验证完成前保留 draft。非阻塞建议及无关清理另行处理。

治理、架构、IPC/Capability、隐私/数据、破坏性迁移、安全、签名/更新及公开承诺需要人类批准；依赖变更审查来源、许可、供应链风险和回滚，不确定风险交维护者。普通 UI、内部重构、测试和文档在 CI 通过后可合并。

共享 GitHub 身份期间，由维护者亲自写 `Human approval: approved @ <full-commit-sha> - <scope>`；Agent 不得代写或推断。批准绑定提交，新提交使旧批准失效，实质变更重新验证。存在第二独立维护者后改用正式 review。

无绕过 issue、适用 CI、隐私及人类批准的紧急路径；事故使用精简 issue/PR。技术设施尚未启用，见[覆盖与后续](../decision-coverage.md)。


来源：[#4](https://github.com/NibbleOrg/Nibble-desktop/issues/4)
