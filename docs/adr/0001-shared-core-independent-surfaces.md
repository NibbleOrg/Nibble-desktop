# ADR-0001：共享核心与独立界面

Status: Accepted
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

架构选择及 ADR 文本已由维护者审阅通过。
Date: 2026-11-13

## Context

Workbench 和 Quick Panel 的使用深度不同，窗口可隐藏而后台捕获继续；若状态寄存在界面，窗口生命周期会影响数据和服务。

## Decision

Rust 核心持有共享服务和状态，React 两界面独立展示，通过明确契约协作，允许能力仅在单个界面出现。当前有效契约见[架构](../engineering/architecture.md)。

## Consequences

需要 IPC、版本化同步和窗口协调；可独立验证规则，平台适配不侵入共享规则。

## Alternatives

按窗口持有业务状态会重复规则并使后台依赖窗口；强制界面功能对等会引入不必要页面。

## References

[#5](https://github.com/NibbleOrg/Nibble-desktop/issues/5)、[#6](https://github.com/NibbleOrg/Nibble-desktop/issues/6)、[#8](https://github.com/NibbleOrg/Nibble-desktop/issues/8)
