import { StrictMode } from "react";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, expect, it, vi } from "vitest";
import { Settings } from "./Settings";
const initial = {
  version: 1,
  shortcut: "Alt+Space",
  available: true,
  error: null,
  startupRecovered: false,
  appVersion: "0.1.0",
};
const mocks = vi.hoisted(() => ({ read: vi.fn(), save: vi.fn() }));
vi.mock("../ipc/bindings", () => ({
  commands: { settingsSnapshot: mocks.read, settingsSave: mocks.save },
}));
beforeEach(() => {
  mocks.read.mockResolvedValue(initial);
  mocks.save.mockReset();
});
it("only confirms a successful native save and preserves a failed draft", async () => {
  render(<Settings version={1} />);
  const input = await screen.findByLabelText("全局快捷键");
  fireEvent.change(input, { target: { value: "Control+Space" } });
  mocks.save.mockRejectedValueOnce("saveFailed");
  fireEvent.click(screen.getByRole("button", { name: "保存快捷键" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("保存失败");
  expect(input).toHaveValue("Control+Space");
  expect(screen.queryByText("快捷键已生效并保存。")).not.toBeInTheDocument();
  mocks.save.mockResolvedValueOnce({
    ...initial,
    version: 2,
    shortcut: "Control+Space",
  });
  fireEvent.click(screen.getByRole("button", { name: "保存快捷键" }));
  expect(await screen.findByRole("status")).toHaveTextContent(
    "快捷键已生效并保存",
  );
});
it("shows real identity, recovery status and complete reference without future toggles", async () => {
  mocks.read.mockResolvedValueOnce({
    ...initial,
    available: false,
    startupRecovered: true,
    error: "registrationFailed",
  });
  render(<Settings version={1} />);
  await waitFor(() =>
    expect(screen.getByText(/版本 0.1.0/)).toBeInTheDocument(),
  );
  expect(screen.getByText(/已尝试默认快捷键/)).toBeInTheDocument();
  expect(screen.getByText(/⌘Return/)).toBeInTheDocument();
  expect(screen.queryByRole("switch")).not.toBeInTheDocument();
});
it("waits for a snapshot covering the notification and keeps an edited draft", async () => {
  mocks.read.mockResolvedValueOnce(initial);
  const view = render(<Settings version={1} />);
  const input = await screen.findByLabelText("全局快捷键");
  fireEvent.change(input, { target: { value: "Super+Space" } });
  mocks.read.mockResolvedValueOnce(initial).mockResolvedValueOnce({
    ...initial,
    version: 3,
    shortcut: "Control+Space",
  });
  view.rerender(<Settings version={3} />);
  await screen.findByText(/当前快捷键：Control\+Space/);
  expect(input).toHaveValue("Super+Space");
});
it("survives StrictMode effect reactivation", async () => {
  render(
    <StrictMode>
      <Settings version={1} />
    </StrictMode>,
  );
  expect(await screen.findByLabelText("全局快捷键")).toHaveValue("Alt+Space");
});
