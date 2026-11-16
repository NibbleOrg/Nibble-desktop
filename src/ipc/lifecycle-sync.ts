/** Coalesce notifications; never publish a snapshot older than the known version. */
export function createVersionReader<T extends { version: number }>(
  read: () => Promise<T>,
  publish: (snapshot: T) => void,
  failed: () => void,
) {
  let known = 0;
  let disposed = false;
  let running: Promise<void> | undefined;
  return {
    refresh(version: number): Promise<void> {
      known = Math.max(known, version);
      if (running) return running;
      running = (async () => {
        try {
          do {
            const snapshot = await read();
            if (disposed) return;
            if (snapshot.version < known) continue;
            known = snapshot.version;
            publish(snapshot);
            return;
          } while (!disposed);
        } catch {
          if (!disposed) failed();
        } finally {
          running = undefined;
        }
      })();
      return running;
    },
    dispose() {
      disposed = true;
    },
  };
}
