import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import { beforeEach, expect, it, vi } from "vitest";
import { Calendar } from "./CalendarView";
const read = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ isTauri: () => true }));
vi.mock("../ipc/bindings", () => ({ commands: { calendarPreferences: read } }));
beforeEach(() => read.mockReset());
it("honors the OS week override even when the locale normally starts Sunday", async () => {
  read.mockResolvedValue({ locale: "en-US", firstWeekday: 1 });
  render(<Calendar reducedMotion={false} />);
  await waitFor(() => expect(read).toHaveBeenCalled());
  expect(screen.getAllByRole("cell")[0].querySelector("time")).toHaveAttribute(
    "aria-label",
    expect.stringContaining("Monday"),
  );
  read.mockResolvedValue({ locale: "en-US", firstWeekday: 0 });
  fireEvent.focus(window);
  await waitFor(() =>
    expect(
      screen.getAllByRole("cell")[0].querySelector("time"),
    ).toHaveAttribute("aria-label", expect.stringContaining("Sunday")),
  );
});

it("rejects stale reads and keeps the last confirmed settings after an invalid response", async () => {
  let finishOld!: (value: { locale: string; firstWeekday: number }) => void;
  read.mockReturnValueOnce(
    new Promise((resolve) => {
      finishOld = resolve;
    }),
  );
  render(<Calendar reducedMotion={false} />);
  read.mockResolvedValue({ locale: "en-US", firstWeekday: 1 });
  fireEvent.focus(window);
  await waitFor(() =>
    expect(
      screen.getAllByRole("cell")[0].querySelector("time"),
    ).toHaveAttribute("aria-label", expect.stringContaining("Monday")),
  );
  await act(async () => finishOld({ locale: "en-US", firstWeekday: 0 }));
  expect(screen.getAllByRole("cell")[0].querySelector("time")).toHaveAttribute(
    "aria-label",
    expect.stringContaining("Monday"),
  );
  read.mockResolvedValue({ locale: "en-US", firstWeekday: 8 });
  await act(async () => {
    fireEvent.focus(window);
  });
  expect(screen.getByRole("alert")).toBeVisible();
  expect(screen.getAllByRole("cell")[0].querySelector("time")).toHaveAttribute(
    "aria-label",
    expect.stringContaining("Monday"),
  );
});
