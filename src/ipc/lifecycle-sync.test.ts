import { describe, expect, it } from "vitest";
import { createVersionReader } from "./lifecycle-sync";
it("re-reads when a notification arrives during a snapshot read", async () => {
  let resolve!: (value: { version: number }) => void;
  let calls = 0;
  const published: number[] = [];
  const reader = createVersionReader(
    () =>
      ++calls === 1
        ? new Promise<{ version: number }>((r) => {
            resolve = r;
          })
        : Promise.resolve({ version: 2 }),
    (s) => published.push(s.version),
    () => {},
  );
  const running = reader.refresh(0);
  void reader.refresh(2);
  resolve({ version: 1 });
  await running;
  expect(published).toEqual([2]);
});
describe("disposing a window subscriber", () => {
  it("discards a late response", async () => {
    let resolve!: (value: { version: number }) => void;
    const published: number[] = [];
    const reader = createVersionReader(
      () =>
        new Promise<{ version: number }>((r) => {
          resolve = r;
        }),
      (s) => published.push(s.version),
      () => {},
    );
    const running = reader.refresh(0);
    reader.dispose();
    resolve({ version: 1 });
    await running;
    expect(published).toEqual([]);
  });
});
