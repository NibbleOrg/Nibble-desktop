import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./surfaces/App";
import "./tokens.css";
const surface =
  new URLSearchParams(location.search).get("surface") === "quick-panel"
    ? "quick-panel"
    : "workbench";
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App surface={surface} />
  </StrictMode>,
);
