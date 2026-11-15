# 协作设施与 main 保护

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — GitHub 只读核查；保护配置待维护者批准及启用

对应[建立协作模板与 main 保护](https://github.com/NibbleOrg/Nibble-desktop/issues/29)。审批规范见[工程工作流](../agents/workflow.md)，验收规范见[质量](../agents/quality.md)。本页区分配置文件、远端启用及验证结果。

## 当前状态

2026-11-16 使用当前 gh 身份只读查询 `NibbleOrg/Nibble-desktop`：

| 项目 | 实际结果 |
| --- | --- |
| 仓库 | public；默认分支 main |
| 当前身份 | API 返回 admin/maintain/push/triage/pull 均为 true |
| 传统 branch protection | GET branches/main/protection 返回 404 / Branch not protected |
| 仓库及上级 rulesets | GET rulesets?includes_parents=true 返回空数组 |
| 已成功的实际检查 | commit b21ffcd3d5e912d04c38011e928aa472de9ed1fd 的 check-run 名为 macos，conclusion=success |
| 检查来源 | github-actions；App ID 15368 |
| 本分支交付 | 工作/bug issue forms、PR 模板、候选 main ruleset JSON |
| 远端模板展示 | not verified；待模板合入默认分支后核对 GitHub 新建 issue/PR 界面 |
| 远端保护启用 | not verified；当前没有执行创建或更新 |

main 当前仍未受保护，不能将本分支 JSON 或已有 CI 当作保护已生效。当前身份具有管理权限，不等于维护者已批准本次治理变更。

## 模板入口

- `.github/ISSUE_TEMPLATE/work.yml`：目的/用户价值、验收、非目标、依赖、验证、文档、风险回退。
- `.github/ISSUE_TEMPLATE/bug.yml`：上述范围加版本/环境、复现、预期/实际及影响；日志和媒体须去除秘密和私人内容。
- `.github/pull_request_template.md`：Summary / Evidence / Merge Danger，关联议题、验收对应、依赖状态、命令/场景结果、风险/恢复与文档影响。
- 普通表单初始使用 `needs-triage`，充分定义后按分流规则调整；保留 Blank issue 供 Wayfinder 地图/决策使用其专用结构及标签。

模板不代表人类批准，也不自动关闭普通实施 issue。未完成验收保留 draft，未运行写 not verified。字段完整性仍需分流和评审；这些模板不是运行时或安全边界。

## 候选保护配置

`.github/rulesets/main.json` 是可审阅的 REST 创建载荷；GitHub 不会因该文件被合并而自动安装规则。它采用 active enforcement、只匹配 `refs/heads/main`，没有排除项或绕过身份：

- 更新 main 须经过 PR，禁止强推及删除；其他分支不在本规则作用范围。
- 必需检查为实际 API 确认过的 `macos`，限定来源 GitHub Actions App 15368；要求基于最新 main 运行检查。
- 合并须解决 review conversations。共享 GitHub 身份期间 required approvals 为 0，不强制独立 review、CODEOWNERS 或最后一次 push 的独立批准。人类提交级风险审批仍按工作流由维护者本人记录；规则集不验证该评论的真实性。
- 没有配置 admin、组织管理员、团队、用户或 App 的 bypass。具有仓库管理权限的身份仍能修改保护本身，此治理权限不等于获准绕过工作流。

字段依据 GitHub [repository rules REST 文档](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28)。`integration_id` 限定检查来源，strict policy 要求与最新基线一起测试。最终服务端接受与执行情况须在启用后回读，当前 not verified。

## 启用与完成条件

1. 本任务 PR 的适用检查通过；维护者按工作流亲自记录绑定最终完整 SHA 的治理批准。Agent 引用证据，不代写批准。若有新提交，重新核对批准。
2. 启用前重新读取 main protection、含上级的 rulesets 及实际成功 check-run；发现新规则、检查改名或来源变化时，先更新审阅配置，避免覆盖或重复叠加。
3. 确认本次获批准的载荷后，由获授权的维护者或 Agent 创建规则。下面是已知空规则集情况下的精确目标命令，当前尚未执行：

   ```sh
   gh api --method POST repos/NibbleOrg/Nibble-desktop/rulesets \
     -H 'X-GitHub-Api-Version: 2022-11-28' \
     --input .github/rulesets/main.json
   ```

4. 保存返回的规则 ID；GET `repos/NibbleOrg/Nibble-desktop/rulesets/<id>` 及 `repos/NibbleOrg/Nibble-desktop/rules/branches/main`，核对 active、main 条件、空 bypass、四种规则及 macos/App 15368。记录操作者、时间、审批与回读证据；没有回读不宣称已生效。
5. 用正常 PR 的检查/合并门禁状态核对适用性；不向 main 做试验性直接推送、强推或删除。默认分支合入模板后核对两类 issue 表单、初始标签与 PR 模板展示。远端适用身份/实际阻断与展示尚未验证的项目继续写 not verified。
6. 权限或套餐拒绝时保留 API 错误、配置和缺失控制，交维护者处理；不自动添加 bypass、降低必需检查或绕过人类批准。普通任务的关闭仍需 PR 合并及验收证据。

### 恢复

启用前可回退配置提交，远端不会受影响。启用后变更/撤销保护属于治理变更：先保存具体规则 ID 和原始回读 JSON，取得绑定提交的维护者批准，再恢复经批准的上一配置并回读；保留其他仓库/组织规则。没有无审批的紧急绕过路径。

## 本次验证范围

已核对实际仓库权限、空保护/规则集和成功 check-run 的名称/来源。新增模板/JSON 进入现有 format:check 范围。YAML/JSON 语法、字段唯一性、标签、配置目标与文档链接由本次任务检查；远端表单展示、规则创建和实际门禁尚待上述启用步骤。应用运行时没有变更，生产行为测试不适用于本次模板变更；远端现有 macOS CI 结果以本任务 PR checks 为准。

可复现的本地语法/格式入口：

```sh
pnpm format:check
ruby -ryaml -e 'ARGV.each { |path| YAML.safe_load(File.read(path)); puts path }' .github/ISSUE_TEMPLATE/*.yml
python3 -m json.tool .github/rulesets/main.json > /dev/null
```

本次另核对了表单 ID 唯一、必填文本字段、已有 needs-triage 标签、main 唯一目标、空 bypass、四类规则及 macos/App 来源，五份变更文档的本地链接和 diff whitespace 检查通过。运行时代码未变更；本轮本地 lint/typecheck/test/build 为 not verified，远端 CI 仍按既有流程执行。
