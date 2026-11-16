import { useEffect, useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { commands, type CalendarPreferences } from "./bindings";
import { weekStart } from "../calendar";

export function useCalendarPreferences() {
  const [preferences, setPreferences] = useState<CalendarPreferences | null>(
    () => {
      if (isTauri()) return null;
      const locale = new Intl.DateTimeFormat().resolvedOptions().locale;
      return { locale, firstWeekday: weekStart(locale) };
    },
  );
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    let generation = 0;
    let disposed = false;
    async function refresh() {
      const request = ++generation;
      try {
        const next = isTauri()
          ? await commands.calendarPreferences()
          : (() => {
              const locale = new Intl.DateTimeFormat().resolvedOptions().locale;
              return { locale, firstWeekday: weekStart(locale) };
            })();
        // Check the transport before letting an invalid locale break rendering.
        if (typeof next.locale !== "string" || !next.locale)
          throw new Error("Invalid calendar locale");
        new Intl.DateTimeFormat(next.locale);
        if (
          !Number.isInteger(next.firstWeekday) ||
          next.firstWeekday < 0 ||
          next.firstWeekday > 6
        )
          throw new Error("Invalid calendar preferences");
        if (!disposed && request === generation) {
          setPreferences(next);
          setFailed(false);
        }
      } catch {
        if (!disposed && request === generation) setFailed(true);
      }
    }
    void refresh();
    const timer = window.setInterval(() => void refresh(), 30_000);
    const onFocus = () => void refresh();
    window.addEventListener("focus", onFocus);
    window.addEventListener("languagechange", onFocus);
    return () => {
      disposed = true;
      window.clearInterval(timer);
      window.removeEventListener("focus", onFocus);
      window.removeEventListener("languagechange", onFocus);
    };
  }, []);
  return { preferences, failed };
}
