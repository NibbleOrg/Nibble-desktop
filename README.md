# Nibble

本地优先的个人桌面工具：Quick Panel 快速完成小任务，Workbench 承载需要更大空间的场景。

v1 规划文档已完成并经维护者审阅通过，仓库已有最小 Tauri/React 工程及浏览器原型；完整界面、业务、安装包与平台行为尚未验证。首发目标为 Apple Silicon / macOS 14+ Preview，Windows 11 x64 规划于 v4，无日期承诺。

v1 计划提供纯文本剪贴板历史、Base64 和只读月份浏览。能力范围与隐私规则见[文档索引](docs/README.md)，安装、更新及未公证限制见[发布规划](docs/operations/release.md)。公开下载入口以实际 Releases 为准。

- [Quick Panel 原型](prototypes/quick-panel-visual/README.md)
- [Workbench 原型](prototypes/workbench-visual/README.md)
- [项目定位与维护边界](docs/business/project.md)
- [Agent 指南](AGENTS.md)

免费、个人维护；许可见 [Apache-2.0](LICENSE)。

本地开发：`pnpm install --frozen-lockfile` 后执行 `pnpm dev`。版本和检查命令见[工程初始化](docs/engineering/bootstrap.md)。
