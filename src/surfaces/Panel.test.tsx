import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import { beforeEach, expect, it, vi } from "vitest";
import { commands, type Snapshot_Serialize } from "../ipc/bindings";
import { App } from "./App";
const model = vi.hoisted(() => ({
  snapshot: null as Snapshot_Serialize | null,
}));
vi.mock("../ipc/useLifecycle", () => ({
  useLifecycle: () => ({ snapshot: model.snapshot, failed: false }),
}));
vi.mock("../ipc/bindings", () => ({
  commands: {
    panelClose: vi.fn().mockResolvedValue(null),
    panelTransition: vi.fn().mockResolvedValue(null),
    panelTransitionComplete: vi.fn().mockResolvedValue(null),
    panelSettings: vi.fn().mockResolvedValue(null),
  },
}));
beforeEach(() => {
  vi.clearAllMocks();
  model.snapshot = {
    version: 1,
    session: 1,
    view: "search",
    transition: null,
    settingsRequested: false,
    shortcutAvailable: true,
    shortcutFailure: null,
    fault: null,
    reducedMotion: false,
    demoPaused: false,
  };
});
it("starts a reopened session with empty search and focus", async () => {
  const { rerender } = render(<App surface="quick-panel" />);
  fireEvent.change(screen.getByRole("combobox", { name: "搜索工具" }), {
    target: { value: "Base64" },
  });
  model.snapshot = { ...model.snapshot!, session: 2, version: 2 };
  rerender(<App surface="quick-panel" />);
  expect(screen.getByRole("combobox", { name: "搜索工具" })).toHaveValue("");
  await waitFor(() =>
    expect(screen.getByRole("combobox", { name: "搜索工具" })).toHaveFocus(),
  );
});
it("accepts escape during a transition while disabling repeated selection", () => {
  model.snapshot = { ...model.snapshot!, transition: 3 };
  render(<App surface="quick-panel" />);
  expect(screen.getByRole("option", { name: /Base64/ })).toBeDisabled();
  fireEvent.keyDown(window, { key: "Escape" });
  expect(commands.panelClose).toHaveBeenCalledWith(1);
});
it("cancels completion timers on close so they cannot complete a later session", () => {
  vi.useFakeTimers();
  try {
    model.snapshot = { ...model.snapshot!, transition: 3, reducedMotion: true };
    const { unmount } = render(<App surface="quick-panel" />);
    unmount();
    vi.advanceTimersByTime(1000);
    expect(commands.panelTransitionComplete).not.toHaveBeenCalled();
  } finally {
    vi.useRealTimers();
  }
});

it("filters aliases, handles empty results, and enters the keyboard selection", () => {
  render(<App surface="quick-panel" />);
  const search = screen.getByRole("combobox", { name: "搜索工具" });
  fireEvent.change(search, { target: { value: "未知工具" } });
  expect(screen.getByText("没有匹配的工具")).toBeInTheDocument();
  fireEvent.change(search, { target: { value: "剪贴板" } });
  expect(screen.getAllByRole("option")).toHaveLength(1);
  fireEvent.keyDown(search, { key: "Enter" });
  expect(commands.panelTransition).toHaveBeenCalledWith(1, "history");
});
it("closes the reference before the panel and restores its trigger focus", async () => {
  render(<App surface="quick-panel" />);
  const trigger = screen.getByRole("button", { name: "快捷键参考" });
  fireEvent.click(trigger);
  expect(screen.getByRole("dialog")).toBeInTheDocument();
  fireEvent.keyDown(window, { key: "Escape" });
  await act(async () => {});
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  expect(trigger).toHaveFocus();
  expect(commands.panelClose).not.toHaveBeenCalled();
  fireEvent.keyDown(window, { key: "Escape" });
  expect(commands.panelClose).toHaveBeenCalledWith(1);
});
it("ordinary builds offer no demo controls or pretend tool functionality", () => {
  model.snapshot = { ...model.snapshot!, view: "base64" };
  render(<App surface="quick-panel" />);
  expect(screen.getByText(/当前构建未提供此工具能力/)).toBeInTheDocument();
  expect(
    screen.queryByRole("button", { name: "开发场景" }),
  ).not.toBeInTheDocument();
  expect(
    screen.queryByRole("button", { name: "模拟完成" }),
  ).not.toBeInTheDocument();
});
