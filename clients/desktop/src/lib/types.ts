export type ConnectionPhase =
  | "disconnected"
  | "starting"
  | "connected"
  | "stopping"
  | "failed";

export type CapabilityState = "available" | "manual" | "planned" | "unsupported";

export interface PlatformCapabilities {
  os: string;
  arch: string;
  proxyMode: CapabilityState;
  lanSharing: CapabilityState;
  systemProxy: CapabilityState;
  vpnMode: CapabilityState;
  androidVpn: CapabilityState;
  notes: string[];
}

export interface ProxyEndpoint {
  host: string;
  port: number;
  enabled: boolean;
}

export interface TrajectoryProfile {
  id: string;
  name: string;
  domain: string;
  accessKey: string;
  accessKeySaved: boolean;
  resolvers: string[];
  resolverFile?: string;
  resolverSocksProxy?: string;
  resolverTransport: "auto" | "udp" | "tcp";
  transportMode: "secure" | "velocity" | "resilient" | "frontier";
  socks: ProxyEndpoint;
  http: ProxyEndpoint;
  dnsMaxPayload: number;
  resolverCohortSize?: number;
  resolverAdmissionMin: number;
  pollIntervalMs: number;
  allowLanWithoutAuth: boolean;
  admissionReport: boolean;
}

export interface RuntimeSnapshot {
  phase: ConnectionPhase;
  activeProfileId?: string;
  activeProfileName?: string;
  pid?: number;
  startedAt?: string;
  socksEndpoint?: string;
  httpEndpoint?: string;
  binaryPath?: string;
  statusDetail?: string;
  lastError?: string;
  logLines: string[];
  capabilities: PlatformCapabilities;
}

export interface ProfileStoreSnapshot {
  profiles: TrajectoryProfile[];
  selectedProfileId?: string;
}

export interface DesktopApi {
  loadSnapshot(): Promise<RuntimeSnapshot>;
  loadProfiles(): Promise<ProfileStoreSnapshot>;
  saveProfile(profile: TrajectoryProfile): Promise<ProfileStoreSnapshot>;
  deleteProfile(profileId: string): Promise<ProfileStoreSnapshot>;
  setSelectedProfile(profileId: string): Promise<ProfileStoreSnapshot>;
  connect(profileId: string): Promise<RuntimeSnapshot>;
  disconnect(): Promise<RuntimeSnapshot>;
  enableSystemProxy(profileId: string): Promise<RuntimeSnapshot>;
  disableSystemProxy(): Promise<RuntimeSnapshot>;
  markFrontendReady(visibleText?: string, visualReport?: string): Promise<void>;
  markSmokeStateReady(): Promise<void>;
  smokeUiFlowEnabled(): Promise<boolean>;
  markSmokeUiFlowReady(
    connectedText?: string,
    connectedVisualReport?: string,
    disconnectedText?: string,
    disconnectedVisualReport?: string,
  ): Promise<void>;
  markSmokeFrontendError(message: string): Promise<void>;
}
