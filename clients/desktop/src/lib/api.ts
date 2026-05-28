import {
  loadProfiles,
  loadSelectedProfileId,
  saveProfiles,
  saveSelectedProfileId,
} from "./profile-store";
import type {
  DesktopApi,
  ProfileStoreSnapshot,
  RuntimeSnapshot,
  TrajectoryProfile,
} from "./types";

class NoTauriRuntimeError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NoTauriRuntimeError";
  }
}

let tauriCore: Promise<typeof import("@tauri-apps/api/core")> | undefined;

const loadTauriCore = () => {
  tauriCore ??= import("@tauri-apps/api/core");
  return tauriCore;
};

function isMissingTauriRuntime(error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  return (
    message.includes("__TAURI_INTERNALS__") ||
    message.includes("__TAURI__") ||
    /Cannot read properties of undefined/i.test(message) ||
    /is not defined/i.test(message) ||
    /is not a function/i.test(message)
  );
}

async function invokeCommand<T>(command: string, args?: Record<string, unknown>) {
  try {
    const { invoke } = await loadTauriCore();
    return await invoke<T>(command, args);
  } catch (error) {
    if (isMissingTauriRuntime(error)) {
      throw new NoTauriRuntimeError(String(error));
    }
    throw error;
  }
}

async function invokeOrMock<T>(
  command: string,
  args: Record<string, unknown> | undefined,
  fallback: () => Promise<T>,
) {
  try {
    return await invokeCommand<T>(command, args);
  } catch (error) {
    if (error instanceof NoTauriRuntimeError) {
      return fallback();
    }
    throw error;
  }
}

const mockSnapshot: RuntimeSnapshot = {
  phase: "disconnected",
  statusDetail: "Browser preview does not control a real trajectory-client process.",
  logLines: [
    "browser preview mode: open this app with Tauri to start trajectory-client",
  ],
  capabilities: {
    os: "browser",
    arch: "preview",
    proxyMode: "manual",
    lanSharing: "manual",
    systemProxy: "planned",
    vpnMode: "planned",
    androidVpn: "planned",
    notes: ["Preview mode renders UI only; Tauri mode controls the client binary."],
  },
};

let previewSnapshot = mockSnapshot;

const mockApi: DesktopApi = {
  async loadSnapshot() {
    return previewSnapshot;
  },
  async loadProfiles() {
    const profiles = loadProfiles();
    return {
      profiles,
      selectedProfileId: loadSelectedProfileId(profiles),
    };
  },
  async saveProfile(profile: TrajectoryProfile) {
    const profiles = loadProfiles();
    const next = profiles.some((item) => item.id === profile.id)
      ? profiles.map((item) => (item.id === profile.id ? profile : item))
      : [...profiles, profile];
    saveProfiles(next);
    saveSelectedProfileId(profile.id);
    return { profiles: next, selectedProfileId: profile.id };
  },
  async deleteProfile(profileId: string) {
    const remaining = loadProfiles().filter((profile) => profile.id !== profileId);
    const profiles = remaining.length > 0 ? remaining : loadProfiles();
    const selectedProfileId = profiles[0]?.id;
    saveProfiles(profiles);
    if (selectedProfileId) {
      saveSelectedProfileId(selectedProfileId);
    }
    return { profiles, selectedProfileId };
  },
  async setSelectedProfile(profileId: string) {
    const profiles = loadProfiles();
    saveSelectedProfileId(profileId);
    return { profiles, selectedProfileId: profileId };
  },
  async connect(profileId: string) {
    const profiles = loadProfiles();
    const profile = profiles.find((item) => item.id === profileId) ?? profiles[0];
    previewSnapshot = {
      ...mockSnapshot,
      phase: "starting",
      statusDetail: "Browser preview simulated a start request; no real listeners are running.",
      activeProfileId: profile.id,
      activeProfileName: profile.name,
      socksEndpoint: profile.socks.enabled
        ? `${profile.socks.host}:${profile.socks.port}`
        : undefined,
      httpEndpoint: profile.http.enabled
        ? `${profile.http.host}:${profile.http.port}`
        : undefined,
      startedAt: new Date().toISOString(),
      logLines: [
        "browser preview mode: simulated connection",
        `profile ${profile.name} would start against ${profile.domain || "<missing domain>"}`,
      ],
    };
    return previewSnapshot;
  },
  async disconnect() {
    previewSnapshot = mockSnapshot;
    return previewSnapshot;
  },
  async enableSystemProxy() {
    return {
      ...mockSnapshot,
      logLines: [...mockSnapshot.logLines, "browser preview mode: system proxy not changed"],
    };
  },
  async disableSystemProxy() {
    return {
      ...mockSnapshot,
      logLines: [...mockSnapshot.logLines, "browser preview mode: system proxy not changed"],
    };
  },
  async markFrontendReady() {},
  async markSmokeStateReady() {},
  async smokeUiFlowEnabled() {
    return false;
  },
  async markSmokeUiFlowReady() {},
  async markSmokeFrontendError() {},
};

export async function markSmokeFrontendErrorBestEffort(message: string) {
  try {
    await invokeCommand<void>("mark_smoke_frontend_error", { message });
  } catch {
    // Browser preview and early startup failures may not have Tauri IPC yet.
  }
}

export const desktopApi: DesktopApi = {
  loadSnapshot: () =>
    invokeOrMock<RuntimeSnapshot>("load_snapshot", undefined, mockApi.loadSnapshot),
  loadProfiles: () =>
    invokeOrMock<ProfileStoreSnapshot>("load_profiles", undefined, mockApi.loadProfiles),
  saveProfile: (profile) =>
    invokeOrMock<ProfileStoreSnapshot>("save_profile", { profile }, () =>
      mockApi.saveProfile(profile),
    ),
  deleteProfile: (profileId) =>
    invokeOrMock<ProfileStoreSnapshot>("delete_profile", { profileId }, () =>
      mockApi.deleteProfile(profileId),
    ),
  setSelectedProfile: (profileId) =>
    invokeOrMock<ProfileStoreSnapshot>("set_selected_profile", { profileId }, () =>
      mockApi.setSelectedProfile(profileId),
    ),
  connect: (profileId) =>
    invokeOrMock<RuntimeSnapshot>("connect_profile", { profileId }, () =>
      mockApi.connect(profileId),
    ),
  disconnect: () =>
    invokeOrMock<RuntimeSnapshot>("disconnect_profile", undefined, mockApi.disconnect),
  enableSystemProxy: (profileId) =>
    invokeOrMock<RuntimeSnapshot>("enable_system_proxy", { profileId }, () =>
      mockApi.enableSystemProxy(profileId),
    ),
  disableSystemProxy: () =>
    invokeOrMock<RuntimeSnapshot>(
      "disable_system_proxy",
      undefined,
      mockApi.disableSystemProxy,
    ),
  markFrontendReady: (visibleText, visualReport) =>
    invokeOrMock<void>("mark_frontend_ready", { visibleText, visualReport }, () =>
      mockApi.markFrontendReady(visibleText, visualReport),
    ),
  markSmokeStateReady: () =>
    invokeOrMock<void>("mark_smoke_state_ready", undefined, mockApi.markSmokeStateReady),
  smokeUiFlowEnabled: () =>
    invokeOrMock<boolean>("smoke_ui_flow_enabled", undefined, mockApi.smokeUiFlowEnabled),
  markSmokeUiFlowReady: (
    connectedText,
    connectedVisualReport,
    disconnectedText,
    disconnectedVisualReport,
  ) =>
    invokeOrMock<void>(
      "mark_smoke_ui_flow_ready",
      {
        connectedText,
        connectedVisualReport,
        disconnectedText,
        disconnectedVisualReport,
      },
      () =>
        mockApi.markSmokeUiFlowReady(
          connectedText,
          connectedVisualReport,
          disconnectedText,
          disconnectedVisualReport,
        ),
    ),
  markSmokeFrontendError: (message) =>
    invokeOrMock<void>("mark_smoke_frontend_error", { message }, () =>
      mockApi.markSmokeFrontendError(message),
    ),
};
