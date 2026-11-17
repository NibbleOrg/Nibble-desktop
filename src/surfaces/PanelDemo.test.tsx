import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { DemoTools } from "./PanelDemo";
const close = vi.fn();
const pause = vi.fn();
afterEach(() => {
  vi.useRealTimers();
  vi.clearAllMocks();
});
it("only displays preset results and clears them on arbitrary edits", () => {
  render(
    <DemoTools
      view="base64"
      paused={false}
      onPause={pause}
      onComplete={close}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "编码样例" }));
  expect(screen.getByLabelText("演示结果")).toHaveValue("SGVsbG8=");
  fireEvent.change(screen.getByLabelText("输入"), {
    target: { value: "private draft" },
  });
  expect(screen.getByLabelText("演示结果")).toHaveValue("");
  expect(screen.getByText("本阶段未启用转换")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "模拟完成" })).toBeDisabled();
});
it("shows perceptible simulated feedback before closing and cancels stale completion", () => {
  vi.useFakeTimers();
  const { unmount } = render(
    <DemoTools
      view="history"
      paused={false}
      onPause={pause}
      onComplete={close}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "模拟完成" }));
  expect(screen.getByRole("status")).toHaveTextContent(
    "演示操作完成，未写入剪贴板",
  );
  expect(close).not.toHaveBeenCalled();
  unmount();
  act(() => vi.advanceTimersByTime(2000));
  expect(close).not.toHaveBeenCalled();
});
it("closes the current tool after feedback and exposes failures without success", () => {
  vi.useFakeTimers();
  render(
    <DemoTools
      view="history"
      paused={false}
      onPause={pause}
      onComplete={close}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "模拟完成" }));
  act(() => vi.advanceTimersByTime(1000));
  expect(close).toHaveBeenCalledOnce();
});

it("keeps history failure, empty, permission, and paused states distinct", () => {
  const { rerender } = render(
    <DemoTools
      view="history"
      paused={false}
      onPause={pause}
      onComplete={close}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "开发场景" }));
  for (const scenario of ["failure", "unavailable"]) {
    fireEvent.change(screen.getByRole("combobox", { name: "历史演示场景" }), {
      target: { value: scenario },
    });
    fireEvent.click(screen.getByRole("button", { name: "模拟完成" }));
    expect(screen.getByRole("alert")).toBeInTheDocument();
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
    expect(close).not.toHaveBeenCalled();
  }
  for (const scenario of ["empty", "permission"]) {
    fireEvent.change(screen.getByRole("combobox", { name: "历史演示场景" }), {
      target: { value: scenario },
    });
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "模拟完成" })).toBeDisabled();
  }
  rerender(
    <DemoTools
      view="history"
      paused={true}
      onPause={pause}
      onComplete={close}
    />,
  );
  expect(screen.getByText("演示已暂停")).toBeInTheDocument();
  fireEvent.keyDown(screen.getByLabelText("历史演示"), {
    key: "p",
    metaKey: true,
  });
  expect(pause).toHaveBeenCalledOnce();
});
it("shows decode, URL-safe and invalid presets and never converts edited input", () => {
  render(
    <DemoTools
      view="base64"
      paused={false}
      onPause={pause}
      onComplete={close}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "解码样例" }));
  expect(screen.getByLabelText("演示结果")).toHaveValue("Hello");
  fireEvent.click(screen.getByRole("button", { name: "URL-safe 样例" }));
  expect(screen.getByLabelText("演示结果")).toHaveValue("Pz8_");
  fireEvent.click(screen.getByRole("button", { name: "无效样例" }));
  expect(screen.getByRole("alert")).toHaveTextContent("预设无效输入");
  expect(screen.getByLabelText("演示结果")).toHaveValue("");
  fireEvent.change(screen.getByLabelText("输入"), {
    target: { value: "SGVsbG8=" },
  });
  fireEvent.change(screen.getByRole("combobox", { name: "方向" }), {
    target: { value: "encode" },
  });
  expect(screen.getByLabelText("演示结果")).toHaveValue("");
});
