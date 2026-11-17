import { fireEvent, render, screen } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { App } from "./App";
const lifecycle = vi.hoisted(() => ({
  snapshot: { settingsRequested: false, version: 1, reducedMotion: true },
  failed: false,
}));
vi.mock("../ipc/useLifecycle", () => ({ useLifecycle: () => lifecycle }));
vi.mock("../ipc/bindings", () => ({
  commands: {
    settingsAcknowledged: vi.fn().mockResolvedValue(undefined),
    settingsSnapshot: vi.fn().mockResolvedValue({
      version: 2,
      shortcut: "Alt+Space",
      available: true,
      error: null,
      startupRecovered: false,
      appVersion: "0.1.0",
    }),
  },
}));
import { commands } from "../ipc/bindings";
it("consumes native settings navigation and keeps the calendar mounted with reduced motion", () => {
  const view = render(<App surface="workbench" />);
  fireEvent.click(screen.getByRole("button", { name: "下个月" }));
  const month = screen.getByRole("table").getAttribute("aria-label");
  expect(screen.getByRole("region", { name: "月历" })).toHaveAttribute(
    "data-reduced-motion",
    "true",
  );
  lifecycle.snapshot = {
    settingsRequested: true,
    version: 2,
    reducedMotion: true,
  };
  view.rerender(<App surface="workbench" />);
  expect(screen.getByRole("heading", { name: "设置" })).toBeVisible();
  expect(commands.settingsAcknowledged).toHaveBeenCalledOnce();
  fireEvent.click(screen.getByRole("button", { name: "Home" }));
  expect(screen.getByRole("table")).toHaveAttribute("aria-label", month);
});
