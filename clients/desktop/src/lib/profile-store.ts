import type { TrajectoryProfile } from "./types";

const STORAGE_KEY = "trajectory.desktop.profiles.v1";
const SELECTED_KEY = "trajectory.desktop.selectedProfile.v1";

export function createDefaultProfile(): TrajectoryProfile {
  return {
    id: crypto.randomUUID(),
    name: "Local proxy",
    domain: "",
    accessKey: "",
    accessKeySaved: false,
    resolvers: ["1.1.1.1:53", "1.0.0.1:53", "8.8.8.8:53", "8.8.4.4:53"],
    socks: {
      host: "127.0.0.1",
      port: 7000,
      enabled: true,
    },
    http: {
      host: "127.0.0.1",
      port: 7001,
      enabled: true,
    },
    dnsMaxPayload: 1232,
    resolverAdmissionMin: 1,
    pollIntervalMs: 25,
    allowLanWithoutAuth: false,
    admissionReport: true,
  };
}

export function loadProfiles(): TrajectoryProfile[] {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    const initial = createDefaultProfile();
    saveProfiles([initial]);
    saveSelectedProfileId(initial.id);
    return [initial];
  }

  try {
    const parsed = JSON.parse(raw) as TrajectoryProfile[];
    if (!Array.isArray(parsed) || parsed.length === 0) {
      throw new Error("profile store is empty");
    }
    return parsed.map(normalizeProfile);
  } catch {
    const fallback = createDefaultProfile();
    saveProfiles([fallback]);
    saveSelectedProfileId(fallback.id);
    return [fallback];
  }
}

export function saveProfiles(profiles: TrajectoryProfile[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(profiles));
}

export function loadSelectedProfileId(profiles: TrajectoryProfile[]) {
  const stored = localStorage.getItem(SELECTED_KEY);
  if (stored && profiles.some((profile) => profile.id === stored)) {
    return stored;
  }
  return profiles[0]?.id;
}

export function saveSelectedProfileId(profileId: string) {
  localStorage.setItem(SELECTED_KEY, profileId);
}

export function scrubProfile(profile: TrajectoryProfile) {
  return {
    ...profile,
    accessKey: profile.accessKey || profile.accessKeySaved ? "stored in OS credential store" : "missing",
  };
}

function normalizeProfile(profile: TrajectoryProfile): TrajectoryProfile {
  const base = createDefaultProfile();
  return {
    ...base,
    ...profile,
    socks: { ...base.socks, ...profile.socks },
    http: { ...base.http, ...profile.http },
    resolvers: Array.isArray(profile.resolvers) ? profile.resolvers : base.resolvers,
  };
}
