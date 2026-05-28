import {
  loadProfiles,
  loadSelectedProfileId,
  saveProfiles,
  saveSelectedProfileId,
} from "./profile-store";
import { invoke, isTauri } from "@tauri-apps/api/core";
import type {
  DesktopApi,
  ProfileStoreSnapshot,
  RuntimeSnapshot,
  TrajectoryProfile,
} from "./types";

const hasTauriRuntime = () => typeof window !== "undefined" && isTauri();

async function invokeCommand<T>(command: string, args?: Record<string, unknown>) {
  return invoke<T>(command, args);
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

export const desktopApi: DesktopApi = hasTauriRuntime()
  ? {
      loadSnapshot: () => invokeCommand<RuntimeSnapshot>("load_snapshot"),
      loadProfiles: () => invokeCommand<ProfileStoreSnapshot>("load_profiles"),
      saveProfile: (profile) =>
        invokeCommand<ProfileStoreSnapshot>("save_profile", { profile }),
      deleteProfile: (profileId) =>
        invokeCommand<ProfileStoreSnapshot>("delete_profile", { profileId }),
      setSelectedProfile: (profileId) =>
        invokeCommand<ProfileStoreSnapshot>("set_selected_profile", { profileId }),
      connect: (profileId) =>
        invokeCommand<RuntimeSnapshot>("connect_profile", { profileId }),
      disconnect: () => invokeCommand<RuntimeSnapshot>("disconnect_profile"),
      enableSystemProxy: (profileId) =>
        invokeCommand<RuntimeSnapshot>("enable_system_proxy", { profileId }),
      disableSystemProxy: () => invokeCommand<RuntimeSnapshot>("disable_system_proxy"),
      markFrontendReady: (visibleText, visualReport) =>
        invokeCommand<void>("mark_frontend_ready", { visibleText, visualReport }),
      markSmokeStateReady: () => invokeCommand<void>("mark_smoke_state_ready"),
      smokeUiFlowEnabled: () => invokeCommand<boolean>("smoke_ui_flow_enabled"),
      markSmokeUiFlowReady: (
        connectedText,
        connectedVisualReport,
        disconnectedText,
        disconnectedVisualReport,
      ) =>
        invokeCommand<void>("mark_smoke_ui_flow_ready", {
          connectedText,
          connectedVisualReport,
          disconnectedText,
          disconnectedVisualReport,
        }),
      markSmokeFrontendError: (message) =>
        invokeCommand<void>("mark_smoke_frontend_error", { message }),
    }
  : mockApi;
