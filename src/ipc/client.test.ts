import { expect, it, vi } from "vitest";
const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));
import { readBootstrapInfo } from "./client";
it("uses the generated command and propagates transport failures", async () => {
  invoke.mockResolvedValueOnce({ version: "0.1.0", stage: "bootstrap" });
  await expect(readBootstrapInfo()).resolves.toEqual({
    version: "0.1.0",
    stage: "bootstrap",
  });
  expect(invoke.mock.calls[0][0]).toBe("bootstrap_info");
  invoke.mockRejectedValueOnce(new Error("transport unavailable"));
  await expect(readBootstrapInfo()).rejects.toThrow("transport unavailable");
});
