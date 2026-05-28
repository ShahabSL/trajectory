import React from "react";
import ReactDOM from "react-dom/client";
import { markSmokeFrontendErrorBestEffort } from "./lib/api";
import "./styles.css";

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
  const { default: App } = await import("./App");
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}

void bootstrap().catch(reportStartupError);
