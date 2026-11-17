import { useEffect, useState } from "react";
import { calendarDays, sameDay } from "../calendar";
import { useCalendarPreferences } from "../ipc/useCalendarPreferences";
import styles from "./Workbench.module.css";

export function Calendar({ reducedMotion }: { reducedMotion: boolean }) {
  const [now, setNow] = useState(() => new Date());
  const [month, setMonth] = useState(
    () => new Date(now.getFullYear(), now.getMonth(), 1, 12),
  );
  const [outgoing, setOutgoing] = useState<Date | null>(null);
  useEffect(() => {
    const timer = window.setTimeout(() => setOutgoing(null), 200);
    return () => window.clearTimeout(timer);
  }, [month]);
  const [direction, setDirection] = useState(1);
  const { preferences, failed } = useCalendarPreferences();
  useEffect(() => {
    const refresh = () => {
      setNow(new Date());
    };
    const timer = window.setInterval(refresh, 30_000);
    window.addEventListener("focus", refresh);
    window.addEventListener("languagechange", refresh);
    return () => {
      window.clearInterval(timer);
      window.removeEventListener("focus", refresh);
      window.removeEventListener("languagechange", refresh);
    };
  }, []);
  const locale = preferences?.locale;
  const format = (options: Intl.DateTimeFormatOptions) =>
    new Intl.DateTimeFormat(locale, { ...options, calendar: "gregory" });
  const first = preferences?.firstWeekday ?? 0;
  const days = calendarDays(month.getFullYear(), month.getMonth(), first);
  const monthLabel = format({ year: "numeric", month: "long" }).format(month);
  function renderGrid(displayMonth: Date, isOutgoing: boolean) {
    const days = calendarDays(
      displayMonth.getFullYear(),
      displayMonth.getMonth(),
      first,
    );
    return (
      <div
        key={`${displayMonth.getFullYear()}-${displayMonth.getMonth()}`}
        className={`${styles.days} ${isOutgoing ? styles.outgoing : ""}`}
        aria-hidden={isOutgoing || undefined}
        role="table"
        aria-label={monthLabel}
      >
        {Array.from({ length: 6 }, (_, row) => (
          <div role="row" key={row}>
            {days.slice(row * 7, row * 7 + 7).map((day) => (
              <div
                role="cell"
                key={day.toISOString()}
                className={
                  day.getMonth() !== displayMonth.getMonth()
                    ? styles.adjacent
                    : undefined
                }
              >
                <time
                  dateTime={`${day.getFullYear()}-${String(day.getMonth() + 1).padStart(2, "0")}-${String(day.getDate()).padStart(2, "0")}`}
                  aria-label={format({ dateStyle: "full" }).format(day)}
                  aria-current={
                    !isOutgoing && sameDay(day, now) ? "date" : undefined
                  }
                >
                  {format({ day: "numeric" }).format(day)}
                </time>
              </div>
            ))}
          </div>
        ))}
      </div>
    );
  }
  function move(delta: number) {
    setOutgoing(month);
    setDirection(delta);
    setMonth(new Date(month.getFullYear(), month.getMonth() + delta, 1, 12));
  }
  if (!preferences)
    return (
      <section aria-label="月历">
        <p role={failed ? "alert" : "status"}>
          {failed
            ? "系统日期偏好读取失败，请重新激活窗口重试。"
            : "正在读取系统日期偏好…"}
        </p>
      </section>
    );
  return (
    <section
      className={styles.calendar}
      aria-label="月历"
      data-reduced-motion={reducedMotion}
    >
      {failed && (
        <p role="alert">
          系统日期偏好刷新失败，当前显示上次读取的设置；请重新激活窗口重试。
        </p>
      )}
      <header className={styles.calendarHeader}>
        <div>
          <p className={styles.eyebrow}>CALENDAR</p>
          <h1 aria-live="polite">{monthLabel}</h1>
        </div>
        <div className={styles.controls}>
          <button aria-label="上个月" onClick={() => move(-1)}>
            ‹
          </button>
          <button
            onClick={() => {
              const today = new Date();
              if (
                today.getFullYear() !== month.getFullYear() ||
                today.getMonth() !== month.getMonth()
              )
                setOutgoing(month);
              setDirection(today.getTime() < month.getTime() ? -1 : 1);
              setNow(today);
              setMonth(new Date(today.getFullYear(), today.getMonth(), 1, 12));
            }}
          >
            今天
          </button>
          <button aria-label="下个月" onClick={() => move(1)}>
            ›
          </button>
        </div>
      </header>
      <div className={styles.weekdays} aria-hidden="true">
        {days.slice(0, 7).map((day) => (
          <span key={day.getDay()}>
            {format({ weekday: "short" }).format(day)}
          </span>
        ))}
      </div>
      <div
        className={`${styles.gridFrame} ${direction < 0 ? styles.previous : ""}`}
      >
        {renderGrid(month, false)}
        {outgoing && !reducedMotion && renderGrid(outgoing, true)}
      </div>
      <p className={styles.empty}>本月暂无日历事件</p>
    </section>
  );
}
