import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { App } from "./App";
describe("bootstrap surfaces", () => {
  it.each(["workbench", "quick-panel"] as const)(
    "labels %s as unavailable business",
    (surface) => {
      render(<App surface={surface} />);
      expect(
        screen.getByRole("heading", {
          name: surface === "workbench" ? "Workbench" : "Quick Panel",
        }),
      ).toBeVisible();
      expect(screen.getByText(/尚未实现/)).toBeVisible();
      expect(
        screen.queryByRole("button", { name: /复制/ }),
      ).not.toBeInTheDocument();
    },
  );
});
