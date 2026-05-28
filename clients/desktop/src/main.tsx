import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { markSmokeFrontendErrorBestEffort } from "./lib/api";
import "./styles.css";

declare global {
  interface Window {
    __TRAJECTORY_DESKTOP_FRONTEND_BOOTSTRAP__?: string;
    __TRAJECTORY_DESKTOP_FRONTEND_RENDERED__?: string;
  }
}

window.__TRAJECTORY_DESKTOP_FRONTEND_BOOTSTRAP__ = new Date().toISOString();

const formatStartupError = (error: unknown) => {
  if (error instanceof Error) {
    return `${error.name}: ${error.message}\n${error.stack ?? ""}`;
  }
  return String(error);
};

const reportStartupError = (error: unknown) => {
  void markSmokeFrontendErrorBestEffort(formatStartupError(error));
};

window.addEventListener("error", (event) => {
  reportStartupError(event.error ?? event.message);
});

window.addEventListener("unhandledrejection", (event) => {
  reportStartupError(event.reason);
});

async function bootstrap() {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
  window.__TRAJECTORY_DESKTOP_FRONTEND_RENDERED__ = new Date().toISOString();
}

void bootstrap().catch(reportStartupError);
