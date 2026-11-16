import { describe, expect, it } from "vitest";
import { calendarDays, weekStart } from "./calendar";

describe("Gregorian calendar", () => {
  it("includes leap day and adjacent dates in exactly six weeks", () => {
    const days = calendarDays(2024, 1, 1);
    expect(days).toHaveLength(42);
    expect(days[0].getDay()).toBe(1);
    expect(days.filter((d) => d.getMonth() === 1)).toHaveLength(29);
    expect(days[41].getMonth()).toBe(2);
  });
  it("uses regional week starts and Gregorian dates even for other calendars", () => {
    expect(weekStart("en-US")).toBe(0);
    expect(weekStart("en-GB")).toBe(1);
    expect(weekStart("ar-SA")).toBe(0);
    expect(calendarDays(2026, 0, 0)[0].getFullYear()).toBe(2025);
  });
});
