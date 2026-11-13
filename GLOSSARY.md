# Nibble Desktop

Nibble is a local-first personal desktop application with two interaction surfaces over one shared capability and data model.

## Language

**Capability**:
An ability Nibble provides, defined independently of the surfaces that expose it.
_Avoid_: plugin, page

**Workbench**:
The full-context surface for Nibble capabilities whose complete workflow requires sustained navigation, management, inspection, or substantial space.
_Avoid_: personal workspace, dashboard, main window

**Quick Panel**:
The lightweight, summonable surface for immediately using a capability or a deliberately limited projection of a larger capability with minimal interruption.
_Avoid_: personal tool interface, uTools interface, launcher, utility page

**Clipboard Entry**:
One captured plain-text value in Clipboard History.
_Avoid_: clip, clipboard record, history item

**Calendar**:
The Workbench month view used to browse dates and any imported Calendar Events.
_Avoid_: schedule, calendar widget, date dashboard

**Calendar Event**:
A dated item represented by Calendar.
_Avoid_: appointment, activity, calendar item

**Calendar Provider**:
A platform integration that supplies one or more Calendar Sources.
_Avoid_: calendar account, cloud sync service

**Calendar Source**:
One external calendar exposed by a Calendar Provider.
_Avoid_: provider, calendar account

**Calendar Source Label**:
The human-readable Calendar Provider and Calendar Source names shown with an event and used for filtering.
_Avoid_: source identity, duplicate key

**Calendar Import**:
The one-way reconciliation of Calendar Events from Calendar Sources into Nibble's local projection.
_Avoid_: calendar sync, export

**External Event Reference**:
A provider-scoped identity that associates a Calendar Event with an external event occurrence.
_Avoid_: external ID, calendar UID
