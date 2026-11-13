# v1 产品契约

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

Nibble 是单用户、本地优先的桌面应用，共享能力和数据，通过 Workbench 与 Quick Panel 两种界面提供服务。首版为 macOS Preview。

## 范围和成功路径

- Workbench 仅有首页与设置。首页仅显示受限月历：启动为当前月，可前后浏览和回到今天。无事件显示、系统日历访问、导入或提醒。
- Quick Panel 从空工具搜索开始，仅搜索 Clipboard History、Base64 的名称与别名。进入工具后完成使用，工具使用无需前往 Workbench。
- Clipboard History：授权后捕获纯文本，展示最近条目，选择后写入系统剪贴板并关闭面板，用户自行粘贴。数据规则见[隐私与数据](privacy.md)。
- Base64：纯文本 UTF-8 编解码，标准及 URL-safe 字符集，保留文本换行。无文件转换；无效输入或解码明确报错，无部分结果。有效结果可复制并关闭面板。
- 设置作用于整个应用，按通用、Workbench、Quick Panel、工具、关于组织，仅展示有实际配置的分组。工具配置按能力分组；快捷键注册和完整参考位于 Quick Panel 分组。

## 验收边界

验证从唤起、工具选择、操作、成功复制到焦点恢复的完整流程；失败保留当前输入并就地报告。验证设置修改跨窗口生效、关闭窗口后后台行为和完整退出。当前日期、地区标签与周起始日遵循系统；月历固定公历，v1 不提供地区、时区、周首日或历法覆盖设置，具体交互见[交互与视觉](interaction.md)。

不提供剪贴板正文搜索、置顶、单条删除、自动粘贴、图片/文件/富文本历史、多项合并、OCR、账号、云同步、AI、第三方插件、应用/文件/网页搜索。Home 不放置未来模块占位。两界面不要求功能对等。


来源：[#2](https://github.com/NibbleOrg/Nibble-desktop/issues/2)、[#5](https://github.com/NibbleOrg/Nibble-desktop/issues/5)、[#6](https://github.com/NibbleOrg/Nibble-desktop/issues/6)、[#7](https://github.com/NibbleOrg/Nibble-desktop/issues/7)、[#13](https://github.com/NibbleOrg/Nibble-desktop/issues/13)、[#14](https://github.com/NibbleOrg/Nibble-desktop/issues/14)
