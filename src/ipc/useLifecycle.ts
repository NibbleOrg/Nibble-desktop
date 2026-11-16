import { useEffect, useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { commands, type Snapshot_Serialize } from "./bindings";
import { createVersionReader } from "./lifecycle-sync";
export function useLifecycle() {
  const [snapshot, setSnapshot] = useState<Snapshot_Serialize | null>(null);
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    if (!isTauri()) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    const reader = createVersionReader(
      commands.lifecycleSnapshot,
      (s) => {
        setSnapshot(s);
        setFailed(false);
      },
      () => setFailed(true),
    );
    // Subscribe first; a notification racing with read raises the minimum version.
    void listen<number>("lifecycle-version", ({ payload }) => {
      void reader.refresh(payload);
    })
      .then((stop) => {
        if (disposed) {
          stop();
          return;
        }
        unlisten = stop;
        void reader.refresh(0);
      })
      .catch(() => {
        if (!disposed) setFailed(true);
      });
    const reconnect = () => {
      void reader.refresh(0);
    };
    window.addEventListener("focus", reconnect);
    return () => {
      disposed = true;
      reader.dispose();
      unlisten?.();
      window.removeEventListener("focus", reconnect);
    };
  }, []);
  return { snapshot, failed };
}
