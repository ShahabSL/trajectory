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

const hasTauriRuntime = () =>
  typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);

async function invokeCommand<T>(command: string, args?: Record<string, unknown>) {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

const mockSnapshot: RuntimeSnapshot = {
  phase: "disconnected",
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

const mockApi: DesktopApi = {
  async loadSnapshot() {
    return mockSnapshot;
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
    return {
      ...mockSnapshot,
      phase: "connected",
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
  },
  async disconnect() {
    return mockSnapshot;
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
    }
  : mockApi;
