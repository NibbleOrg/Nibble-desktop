import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { NativeProbe } from "./surfaces/NativeProbe";
import { App } from "./surfaces/App";
import "./tokens.css";
const probe =
  new URLSearchParams(location.search).get("surface") === "native-probe";
if (probe) document.documentElement.dataset.nativeProbe = "true";
const surface =
  new URLSearchParams(location.search).get("surface") === "quick-panel"
    ? "quick-panel"
    : "workbench";
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    {probe ? <NativeProbe /> : <App surface={surface} />}
  </StrictMode>,
);
