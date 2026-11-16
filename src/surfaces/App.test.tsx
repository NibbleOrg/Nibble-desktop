import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, it, expect, vi } from "vitest";
import { App } from "./App";

afterEach(() => vi.useRealTimers());
describe("Workbench calendar", () => {
  it("browses months, retains navigation across routes, and resets on restart", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date(2024, 1, 15, 12));
    const view = render(<App surface="workbench" />);
    expect(screen.getAllByRole("cell")).toHaveLength(42);
    const initial = screen.getByRole("table").getAttribute("aria-label");
    expect(
      document.querySelector('time[aria-current="date"]'),
    ).toHaveTextContent("15");
    fireEvent.click(screen.getByRole("button", { name: "下个月" }));
    const next = screen.getByRole("table").getAttribute("aria-label");
    expect(next).not.toBe(initial);
    fireEvent.click(screen.getByRole("button", { name: "设置" }));
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Home" }));
    expect(screen.getByRole("table")).toHaveAttribute("aria-label", next);
    fireEvent.click(screen.getByRole("button", { name: "今天" }));
    expect(screen.getByRole("table")).toHaveAttribute("aria-label", initial);
    fireEvent.click(screen.getByRole("button", { name: "上个月" }));
    view.unmount();
    render(<App surface="workbench" />);
    expect(screen.getByRole("table")).toHaveAttribute("aria-label", initial);
    expect(
      screen.queryByRole("button", { name: /Quick Panel|工具/ }),
    ).not.toBeInTheDocument();
  });
  it("updates today after local midnight without changing the browsed month", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date(2024, 1, 15, 23, 59));
    render(<App surface="workbench" />);
    vi.setSystemTime(new Date(2024, 1, 16, 0, 1));
    fireEvent.focus(window);
    expect(
      document.querySelector('time[aria-current="date"]'),
    ).toHaveTextContent("16");
  });
});
