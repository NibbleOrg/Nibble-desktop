# 文档权威与维护

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

规范文档整理期间为 Draft，以已确认 issue 决议为依据；覆盖与链接核对后统一转 Current 并关闭总蓝图。issue 此后保留历史，当前正文为规范来源，代码表示实现事实。冲突显式解决，旧决议注明替代关系。

规范页包含 Status、Owner、Last verified；后者仅在实际核对后更新，注明相关版本/里程碑。规划核对不代表实机验证。被替代正文保留并链接替代页。

每项事实一个权威位置，其他页链接引用。行为改产品文档，架构/安全改工程，发布/诊断改运维，术语改根 GLOSSARY。README 为入口，docs/product、engineering、business、operations、agents、adr 分别承载对应内容，索引见[文档入口](../README.md)。

根 AGENTS 保持简短，模块仅在有独特命令、安全或领域不变量时新增局部 AGENTS，说明理由，叠加而不能削弱根规则。模块冲突停止相关实施并修正规则。

现阶段交付文档及 Agent 指南；模板、CI、branch protection 和生产代码是后续实施。不得将规范写成技术控制已经启用。


来源：[#4](https://github.com/NibbleOrg/Nibble-desktop/issues/4)、[#11](https://github.com/NibbleOrg/Nibble-desktop/issues/11)
