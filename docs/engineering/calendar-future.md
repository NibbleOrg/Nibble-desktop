# Calendar 未来扩展契约

Status: Current
Owner: 项目维护者
Last verified: 2026-11-13 — v1 规划契约；生产实现与实机行为未验证

本页全部事件与 Provider 内容属于未来范围，v1 不实现事件存储、授权、导入按钮或同步界面。v1 的公历月份浏览见[产品契约](../product/contract.md)。未来集成采用只读、来源权威、单向导入；不会向系统日历写回。以下保留已确认英文契约全文，作为规范整理期间的完整参考，后续中文编辑不得遗漏边界。

## Resolution

**Decision:** Calendar remains display-only in v1. Future calendar integration is a one-way, source-authoritative import into Nibble. Nibble may project imported events locally, but never writes app-originated events, edits, moves, or deletions back to a system calendar.

### Domain model and date ownership

- `Calendar` is a read-only Workbench view and query surface, not a date repository.
- The viewed month is transient UI state. Opening the app starts at the current month; Return to Today selects the current month.
- The domain consumes an explicit half-open date range. Empty grid cells never become domain objects.
- The month grid is Gregorian. OS locale controls labels and the first day of the week. V1 exposes no locale, timezone, week-start, or calendar-system override.
- The current OS timezone determines where events fall in the local grid. A Calendar Provider timezone affects event interpretation only.

### Minimal Calendar Event shape

```text
CalendarEvent {
  id                 // local opaque identity
  title
  timing:
    AllDay(startDate, endDateExclusive)
    | Timed(startsAt, endsAt, sourceTimeZone?)
  sourceRef          // External Event Reference
  sourceLabel        // Provider and Source display names
}
```

- All-day events use local dates and an exclusive end date.
- Timed events use absolute instants and preserve the source timezone when available.
- A multi-day event is one time range spanning multiple days, not multiple events.
- V1 does not model descriptions, locations, reminders, attendees, recurrence, or event status.
- A system timezone change does not move all-day dates or change timed-event instants; it changes their local rendering on the grid.

### Provider and Source model

- `Calendar Provider` is a platform integration that supplies one or more `Calendar Source` values.
- `Calendar Source` is one specific external calendar.
- `Calendar Source Label` is the human-readable Provider and Source naming used for display and filtering. It is not an identity or deduplication key.
- The UI may filter by Source and group Source choices by Provider. Renaming a Source updates its label without creating a new event.
- Each Provider integration is explicitly enabled by the user. Permission is requested only after enablement, and Sources are individually selected. The default is not to enable every available calendar.
- Provider integrations are read-only in the current boundary. There is no create, edit, move, delete, or write-back capability.

### Identity, deduplication, and filtering

- Every normalized event has a stable local opaque `id`.
- The external identity is `(provider, calendarSourceId, externalEventId, occurrenceId?)`. Occurrence identity is reserved for a future recurrence capability; recurrence is not implemented.
- Raw platform identifiers are never used as the internal primary key.
- Repeated imports upsert by external identity rather than appending records.
- Events with identical title and time from different Sources remain distinct. There is no content hash, title/time matching, fuzzy matching, or cross-Source deduplication.
- If an external identifier changes, reconciliation treats the old event as removed and the new event as a separate arrival.

### Manual range import

- A small icon button in the current month view manually triggers synchronization. No polling, system-change watcher, or automatic background import is required.
- The button is labelled for the current month, while the actual query covers the complete visible six-week month grid. Range matching uses event overlap.
- Enabling a Provider or Source does not automatically fetch data; the user triggers the first import with the button.
- Only one synchronization may run at a time. The button is disabled while it runs and indicates the bound range.
- The range is captured when synchronization starts. Navigating to another month does not retarget the active import and does not automatically start another import.
- The UI can show never-synced, syncing, last-sync time, partial failure, and stale states. Their visual treatment belongs to the Workbench interaction and prototype tickets.

### Authoritative reconciliation and deletion

- Each enabled Source is reconciled independently and returns a complete, partial, or failed result.
- On a complete Source snapshot, returned events are upserted and existing projections for that Source in the queried range that are absent from the result are deleted.
- An event moved inside the range is updated in place. An event moved outside the range is removed from its old projection and can be imported when the destination range is synchronized.
- Deletions outside the queried range are never inferred.
- A partial or failed Source result may apply valid returned upserts but must not perform deletion.
- If any returned event for a Source cannot be normalized, the Source result becomes partial to prevent deletion of the skipped event old projection.
- Provider-cancelled events are excluded from the complete event set and therefore disappear from the projection on a later complete import. Tentative and confirmed events import normally.
- Explicit provider cancellation, complete omission, and complete snapshot absence all converge on the same source-authoritative deletion behavior.

### Failure and Source lifecycle

- A transient permission error, Provider outage, timeout, or read failure preserves existing projections, marks them stale, and performs no deletion. Reauthorization followed by a current-month synchronization recalibrates them.
- User-disabled imports and Sources confirmed absent after successful Source discovery remove that Source local projection.
- One failing Source does not roll back another Source that completed successfully.
- Provider or Source failure never blocks application startup.
- Permission states must distinguish unknown, not determined, denied, restricted, and authorized.
- Nibble may remove or hide local projections, but doing so is not a system-calendar write.

### Adapter boundary

- The Calendar domain consumes only normalized Provider, Source, Calendar Event, capability, authorization, warning, and error types.
- EventKit objects, Windows platform objects, and platform identifiers remain inside platform adapters.
- React and Quick Panel do not call platform calendar APIs and do not branch on platform-specific event types.
- Future macOS and Windows adapters must map into this contract without changing the domain model. Their concrete platform API choices and module placement remain with the application architecture and Windows compatibility tickets.
- These adapters are first-party platform integrations, not a third-party plugin system.

### Out of scope

- V1 event display, import UI, event CRUD, recurrence expansion, reminders, multi-timezone event behavior, and system-calendar writes.
- Bidirectional synchronization or conflict merging. A future write capability requires a separate write contract, ownership model, and conflict policy.
- Cross-Source content deduplication.
- Concrete EventKit or Windows API selection in this ticket.

### 后续交付边界

上述事件、Provider、导入状态与按钮全部属于未来扩展。#6 和 #14 仅完成 v1 无事件月历，不承担导入界面的验收。未来启动日历集成时须新建实施议题，补齐界面、存储、适配与验证；本页不承诺其版本或日期。


来源：[#13](https://github.com/NibbleOrg/Nibble-desktop/issues/13)
