/** Dates are local noon so DST transitions do not shift the displayed day. */
export function calendarDays(
  year: number,
  month: number,
  firstWeekday: number,
) {
  const first = new Date(year, month, 1, 12);
  const offset = (first.getDay() - firstWeekday + 7) % 7;
  return Array.from(
    { length: 42 },
    (_, index) => new Date(year, month, 1 - offset + index, 12),
  );
}
export function weekStart(locale: string) {
  const region = new Intl.Locale(locale) as Intl.Locale & {
    getWeekInfo?: () => { firstDay: number };
    weekInfo?: { firstDay: number };
  };
  return (
    (region.getWeekInfo?.() ?? region.weekInfo ?? { firstDay: 1 }).firstDay % 7
  );
}
export function sameDay(a: Date, b: Date) {
  return (
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  );
}
