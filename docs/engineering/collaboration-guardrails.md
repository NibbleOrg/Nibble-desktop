# 协作设施与 main 保护

Status: Current
Owner: 项目维护者
Last verified: 2026-11-16 — main 规则集已启用，并回读核对生效范围及当前身份不可绕过

对应[建立协作模板与 main 保护](https://github.com/NibbleOrg/Nibble-desktop/issues/29)。审批规范见[工程工作流](../agents/workflow.md)，验收规范见[质量](../agents/quality.md)。本页区分配置文件、远端启用及验证结果。

## 当前状态

2026-11-16 完成维护者批准核对、创建及回读，目标为 `NibbleOrg/Nibble-desktop`：

| 项目 | 实际结果 |
| --- | --- |
| 仓库 | public；默认分支 main |
| 当前身份 | API 返回 admin/maintain/push/triage/pull 均为 true |
| 传统 branch protection | 启用前返回 404；本次采用 ruleset，不使用传统保护 |
| 生效 ruleset | main-pr-checks，ID 27338759，enforcement=active，仅 main，空 bypass |
| 已成功的实际检查 | 合并提交 243adc62b9b357f5c05e349957174de245489f39 的 check-run 名为 macos，conclusion=success |
| 检查来源 | github-actions；App ID 15368 |
| 已合并交付 | 工作/bug issue forms、PR 模板、main ruleset JSON；PR 40 已合并且 CI 成功 |
| 模板默认分支内容 | gh API 已确认两类表单、config 及 PR 模板存在；已解析内容与标签 |
| 浏览器展示 | not verified；独立浏览器入口要求登录，未创建测试议题或 PR |
| 远端保护启用 | main effective-rules 返回四类规则，source ruleset_id 均为 27338759 |
| 当前身份绕过能力 | API 返回 current_user_can_bypass=never |

main 现已受规则集保护。启用依据是维护者本人在 [PR 40 的批准评论](https://github.com/NibbleOrg/Nibble-desktop/pull/40#issuecomment-6903104159)明确批准提交 `e0c1d5d4b039bcd4997fcabb334c2a9ad3c54f76`；启用前核对载荷与该提交一致，Agent 没有代写批准。此前的空规则集核查保留为启用前事实，不能再表示当前状态。

## 模板入口

- `.github/ISSUE_TEMPLATE/work.yml`：目的/用户价值、验收、非目标、依赖、验证、文档、风险回退。
- `.github/ISSUE_TEMPLATE/bug.yml`：上述范围加版本/环境、复现、预期/实际及影响；日志和媒体须去除秘密和私人内容。
- `.github/pull_request_template.md`：Summary / Evidence / Merge Danger，关联议题、验收对应、依赖状态、命令/场景结果、风险/恢复与文档影响。
- 普通表单初始使用 `needs-triage`，充分定义后按分流规则调整；保留 Blank issue 供 Wayfinder 地图/决策使用其专用结构及标签。

CR 与 CI 可并行，开始时机及结论范围见[质量规范](../agents/quality.md#cr-与-ci-并行)；这不改变下述 main 合并保护。模板不代表人类批准，也不自动关闭普通实施 issue。未完成验收保留 draft，未运行写 not verified。字段完整性仍需分流和评审；这些模板不是运行时或安全边界。

## 已启用的保护配置

`.github/rulesets/main.json` 是可审阅的 REST 创建载荷；GitHub 不会因该文件被合并而自动安装规则。它采用 active enforcement、只匹配 `refs/heads/main`，没有排除项或绕过身份：

- 更新 main 须经过 PR，禁止强推及删除；其他分支不在本规则作用范围。
- 必需检查为实际 API 确认过的 `macos`，限定来源 GitHub Actions App 15368；要求基于最新 main 运行检查。
- 合并须解决 review conversations。共享 GitHub 身份期间 required approvals 为 0，不强制独立 review、CODEOWNERS 或最后一次 push 的独立批准。人类提交级风险审批仍按工作流由维护者本人记录；规则集不验证该评论的真实性。
- 没有配置 admin、组织管理员、团队、用户或 App 的 bypass。具有仓库管理权限的身份仍能修改保护本身，此治理权限不等于获准绕过工作流。

字段依据 GitHub [repository rules REST 文档](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28)。`integration_id` 限定检查来源，strict policy 要求与最新基线一起测试。本次已在创建后回读并核对服务端接受结果；后续更新仍须独立核对。

## 后续更新与核对流程

1. 本任务 PR 的适用检查通过；维护者按工作流亲自记录绑定最终完整 SHA 的治理批准。Agent 引用证据，不代写批准。若有新提交，重新核对批准。
2. 启用前重新读取 main protection、含上级的 rulesets 及实际成功 check-run；发现新规则、检查改名或来源变化时，先更新审阅配置，避免覆盖或重复叠加。
3. 确认本次获批准的载荷后，由获授权的维护者或 Agent 创建规则。下面是本次空规则集状态下已执行的创建命令；当前已有规则集，后续更新使用已确认的规则 ID，不能重复创建：

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

已核对仓库权限、维护者批准、成功 check-run 名称/来源、创建返回及 main 生效规则。规则服务端创建时间为 2026-11-16 07:24:14 +08:00，操作者为当前获授权 gh 身份；[规则页面](https://github.com/NibbleOrg/Nibble-desktop/rules/27338759)。[原始规则集回读](evidence/main-ruleset-2026-11-16.json)和[main 生效规则回读](evidence/main-effective-rules-2026-11-16.json)保存 API 结果，核对后确认 active、main 唯一范围、无 bypass、四类规则及 macos/App 15368。没有向 main 试验性推送、强推或删除，因此实际拒绝操作未单独试验；以 GitHub 生效规则回读为配置验收证据。

模板合入后的默认分支文件由 gh API 确认存在，之前的 YAML/JSON、字段、标签与格式检查通过；浏览器可见表单展示仍 not verified，不把文件存在写成 UI 实测。PR 40 的 macOS CI 成功。当前文档补录不改变保护载荷、审批政策或应用行为。

可复现的本地语法/格式入口：

```sh
pnpm format:check
ruby -ryaml -e 'ARGV.each { |path| YAML.safe_load(File.read(path)); puts path }' .github/ISSUE_TEMPLATE/*.yml
python3 -m json.tool .github/rulesets/main.json > /dev/null
```

本次另核对了表单 ID 唯一、必填文本字段、已有 needs-triage 标签、main 唯一目标、空 bypass、四类规则及 macos/App 来源，五份变更文档的本地链接和 diff whitespace 检查通过。运行时代码未变更；本轮本地 lint/typecheck/test/build 为 not verified，远端 CI 仍按既有流程执行。
