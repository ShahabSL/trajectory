import {
  Activity,
  AlertTriangle,
  Bug,
  CircleStop,
  Copy,
  DatabaseZap,
  FileDown,
  Globe2,
  KeyRound,
  ListRestart,
  Lock,
  Network,
  Play,
  Plus,
  RadioTower,
  Save,
  Settings,
  Shield,
  SlidersHorizontal,
  TerminalSquare,
  Trash2,
  Wifi,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import logoUrl from "./assets/trajectory-logo.png";
import { desktopApi } from "./lib/api";
import { createDefaultProfile, scrubProfile } from "./lib/profile-store";
import type {
  CapabilityState,
  ConnectionPhase,
  ProfileStoreSnapshot,
  RuntimeSnapshot,
  TrajectoryProfile,
} from "./lib/types";

type TabId =
  | "status"
  | "profiles"
  | "proxy"
  | "resolvers"
  | "diagnostics"
  | "vpn"
  | "settings";

const tabs: Array<{ id: TabId; label: string; icon: typeof Activity }> = [
  { id: "status", label: "Status", icon: Activity },
  { id: "profiles", label: "Profiles", icon: KeyRound },
  { id: "proxy", label: "Proxy", icon: Network },
  { id: "resolvers", label: "Resolvers", icon: RadioTower },
  { id: "diagnostics", label: "Diagnostics", icon: Bug },
  { id: "vpn", label: "VPN", icon: Shield },
  { id: "settings", label: "Settings", icon: Settings },
];

const phaseLabels: Record<ConnectionPhase, string> = {
  disconnected: "Disconnected",
  starting: "Starting",
  connected: "Connected",
  stopping: "Stopping",
  failed: "Failed",
};

const phaseClass: Record<ConnectionPhase, string> = {
  disconnected: "neutral",
  starting: "working",
  connected: "good",
  stopping: "working",
  failed: "bad",
};

const emptySnapshot: RuntimeSnapshot = {
  phase: "disconnected",
  logLines: [],
  capabilities: {
    os: "unknown",
    arch: "unknown",
    proxyMode: "available",
    lanSharing: "manual",
    systemProxy: "planned",
    vpnMode: "planned",
    androidVpn: "planned",
    notes: [],
  },
};

export default function App() {
  const [profiles, setProfiles] = useState<TrajectoryProfile[]>([]);
  const [selectedProfileId, setSelectedProfileId] = useState<string>("");
  const [draft, setDraft] = useState<TrajectoryProfile>(() => createDefaultProfile());
  const [activeTab, setActiveTab] = useState<TabId>("status");
  const [snapshot, setSnapshot] = useState<RuntimeSnapshot>(emptySnapshot);
  const [uiError, setUiError] = useState<string | undefined>();

  const selectedProfile = useMemo(
    () => profiles.find((profile) => profile.id === selectedProfileId) ?? profiles[0],
    [profiles, selectedProfileId],
  );

  useEffect(() => {
    let alive = true;
    const loadInitialState = async () => {
      try {
        const [profileState, runtimeState] = await Promise.all([
          desktopApi.loadProfiles(),
          desktopApi.loadSnapshot(),
        ]);
        if (alive) {
          applyProfileState(profileState);
          setSnapshot(runtimeState);
        }
      } catch (error) {
        if (alive) {
          setUiError(String(error));
        }
      }
    };

    const refreshRuntime = async () => {
      try {
        const next = await desktopApi.loadSnapshot();
        if (alive) {
          setSnapshot(next);
        }
      } catch (error) {
        if (alive) {
          setUiError(String(error));
        }
      }
    };

    loadInitialState();
    const interval = window.setInterval(refreshRuntime, 1500);
    return () => {
      alive = false;
      window.clearInterval(interval);
    };
  }, []);

  useEffect(() => {
    if (selectedProfile) {
      setDraft(selectedProfile);
    }
  }, [selectedProfile]);

  const applyProfileState = (state: ProfileStoreSnapshot, preferredDraftId?: string) => {
    const nextProfiles = state.profiles.length > 0 ? state.profiles : [createDefaultProfile()];
    const nextSelected = state.selectedProfileId ?? preferredDraftId ?? nextProfiles[0].id;
    const nextDraft =
      nextProfiles.find((profile) => profile.id === preferredDraftId) ??
      nextProfiles.find((profile) => profile.id === nextSelected) ??
      nextProfiles[0];
    setProfiles(nextProfiles);
    setSelectedProfileId(nextSelected);
    setDraft(nextDraft);
  };

  const updateDraft = (patch: Partial<TrajectoryProfile>) => {
    setDraft((current) => ({ ...current, ...patch }));
  };

  const saveDraft = async () => {
    const state = await desktopApi.saveProfile(draft);
    applyProfileState(state, draft.id);
    setUiError(undefined);
    return draft.id;
  };

  const addProfile = () => {
    const next = {
      ...createDefaultProfile(),
      name: `Profile ${profiles.length + 1}`,
    };
    setProfiles((current) => [...current, next]);
    setSelectedProfileId(next.id);
    setDraft(next);
    setActiveTab("profiles");
  };

  const deleteProfile = async () => {
    if (profiles.length === 1) {
      setUiError("Keep at least one profile.");
      return;
    }
    try {
      const state = await desktopApi.deleteProfile(draft.id);
      applyProfileState(state);
      setUiError(undefined);
    } catch (error) {
      setUiError(String(error));
    }
  };

  const connect = async () => {
    try {
      const savedProfileId = await saveDraft();
      setUiError(undefined);
      const next = await desktopApi.connect(savedProfileId);
      setSnapshot(next);
    } catch (error) {
      setUiError(String(error));
    }
  };

  const disconnect = async () => {
    try {
      setUiError(undefined);
      const next = await desktopApi.disconnect();
      setSnapshot(next);
    } catch (error) {
      setUiError(String(error));
    }
  };

  const profileWarnings = validateProfile(draft);
  const running = snapshot.phase === "connected" || snapshot.phase === "starting";

  return (
    <main className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <img src={logoUrl} alt="" />
          <div>
            <strong>Trajectory</strong>
            <span>DNS transport client</span>
          </div>
        </div>

        <nav className="nav-list" aria-label="Primary">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                className={activeTab === tab.id ? "nav-item active" : "nav-item"}
                onClick={() => setActiveTab(tab.id)}
              >
                <Icon size={17} />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </nav>

        <div className="sidebar-footer">
          <StatusPill phase={snapshot.phase} />
          <span>{snapshot.capabilities.os} / {snapshot.capabilities.arch}</span>
        </div>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <p className="eyeline">Active profile</p>
            <h1>{selectedProfile?.name ?? "No profile"}</h1>
          </div>
          <div className="topbar-actions">
            <select
              value={selectedProfileId}
              onChange={(event) => {
                const id = event.target.value;
                const profile = profiles.find((item) => item.id === id);
                if (profile) {
                  void desktopApi
                    .setSelectedProfile(id)
                    .catch((error) => setUiError(String(error)));
                  setSelectedProfileId(id);
                  setDraft(profile);
                }
              }}
              aria-label="Select profile"
            >
              {profiles.map((profile) => (
                <option key={profile.id} value={profile.id}>
                  {profile.name}
                </option>
              ))}
            </select>
            {running ? (
              <button className="button danger" onClick={disconnect}>
                <CircleStop size={17} />
                Disconnect
              </button>
            ) : (
              <button
                className="button primary"
                onClick={connect}
                disabled={profileWarnings.length > 0}
              >
                <Play size={17} />
                Connect
              </button>
            )}
          </div>
        </header>

        {(uiError || snapshot.lastError || profileWarnings.length > 0) && (
          <div className="notice">
            <AlertTriangle size={18} />
            <div>
              {uiError || snapshot.lastError || profileWarnings[0]}
              {profileWarnings.length > 1 && (
                <span className="muted"> {profileWarnings.length - 1} more profile issue(s).</span>
              )}
            </div>
          </div>
        )}

        {activeTab === "status" && (
          <StatusView
            snapshot={snapshot}
            profile={draft}
            onCopy={copyText}
          />
        )}
        {activeTab === "profiles" && (
          <ProfilesView
            draft={draft}
            profiles={profiles}
            updateDraft={updateDraft}
            saveDraft={saveDraft}
            addProfile={addProfile}
            deleteProfile={deleteProfile}
          />
        )}
        {activeTab === "proxy" && (
          <ProxyView
            draft={draft}
            updateDraft={updateDraft}
            snapshot={snapshot}
            onCopy={copyText}
            onSetSystemProxy={async () => {
              try {
                const savedProfileId = await saveDraft();
                setSnapshot(await desktopApi.enableSystemProxy(savedProfileId));
              } catch (error) {
                setUiError(String(error));
              }
            }}
            onRestoreSystemProxy={async () => {
              try {
                setSnapshot(await desktopApi.disableSystemProxy());
              } catch (error) {
                setUiError(String(error));
              }
            }}
          />
        )}
        {activeTab === "resolvers" && (
          <ResolversView draft={draft} updateDraft={updateDraft} />
        )}
        {activeTab === "diagnostics" && (
          <DiagnosticsView snapshot={snapshot} profile={draft} />
        )}
        {activeTab === "vpn" && (
          <VpnView capabilities={snapshot.capabilities} />
        )}
        {activeTab === "settings" && (
          <SettingsView draft={draft} updateDraft={updateDraft} />
        )}
      </section>
    </main>
  );
}

function StatusView({
  snapshot,
  profile,
  onCopy,
}: {
  snapshot: RuntimeSnapshot;
  profile: TrajectoryProfile;
  onCopy: (value: string) => void;
}) {
  const socks = profile.socks.enabled ? `${profile.socks.host}:${profile.socks.port}` : "disabled";
  const http = profile.http.enabled ? `${profile.http.host}:${profile.http.port}` : "disabled";
  return (
    <div className="content-grid">
      <section className="panel hero-panel">
        <div className="hero-copy">
          <StatusPill phase={snapshot.phase} />
          <h2>{phaseLabels[snapshot.phase]}</h2>
          <p>{snapshot.statusDetail ?? "Waiting for runtime status."}</p>
        </div>
        <div className="metric-grid">
          <Metric label="SOCKS" value={snapshot.socksEndpoint ?? socks} copy onCopy={onCopy} />
          <Metric label="HTTP" value={snapshot.httpEndpoint ?? http} copy onCopy={onCopy} />
          <Metric label="PID" value={snapshot.pid ? String(snapshot.pid) : "none" } />
          <Metric label="Binary" value={snapshot.binaryPath ?? "auto"} />
        </div>
      </section>

      <section className="panel">
        <SectionHeader icon={Wifi} title="Connect Apps" />
        <div className="endpoint-list">
          <EndpointRow
            label="SOCKS5"
            value={socks}
            command={`curl --socks5-hostname ${socks} https://ifconfig.me`}
            onCopy={onCopy}
          />
          <EndpointRow
            label="HTTP"
            value={http}
            command={`curl -x http://${http} https://example.com`}
            onCopy={onCopy}
          />
          <EndpointRow
            label="Browser"
            value="Manual proxy"
            command={`Set HTTP proxy to ${http} and SOCKS proxy to ${socks}`}
            onCopy={onCopy}
          />
        </div>
      </section>

      <section className="panel">
        <SectionHeader icon={Globe2} title="Tunnel Target" />
        <dl className="definition-grid">
          <dt>Domain</dt>
          <dd>{profile.domain || "not configured"}</dd>
          <dt>Resolver count</dt>
          <dd>{profile.resolvers.length}</dd>
          <dt>Resolver gate</dt>
          <dd>{profile.resolverSocksProxy || "direct DNS"}</dd>
          <dt>Resolver transport</dt>
          <dd>{profile.resolverTransport}</dd>
          <dt>Mode</dt>
          <dd>{profile.transportMode}</dd>
          <dt>DNS payload</dt>
          <dd>{profile.dnsMaxPayload} bytes</dd>
        </dl>
      </section>
    </div>
  );
}

function ProfilesView({
  draft,
  profiles,
  updateDraft,
  saveDraft,
  addProfile,
  deleteProfile,
}: {
  draft: TrajectoryProfile;
  profiles: TrajectoryProfile[];
  updateDraft: (patch: Partial<TrajectoryProfile>) => void;
  saveDraft: () => Promise<string>;
  addProfile: () => void;
  deleteProfile: () => Promise<void>;
}) {
  return (
    <div className="content-grid two-column">
      <section className="panel">
        <SectionHeader icon={KeyRound} title="Profile" />
        <div className="form-grid">
          <label>
            Name
            <input value={draft.name} onChange={(event) => updateDraft({ name: event.target.value })} />
          </label>
          <label>
            Tunnel domain
            <input
              value={draft.domain}
              placeholder="t.example.com"
              onChange={(event) => updateDraft({ domain: event.target.value.trim() })}
            />
          </label>
          <label className="wide">
            Access key
            <input
              value={draft.accessKey}
              type="password"
              autoComplete="off"
              placeholder={draft.accessKeySaved ? "Stored in OS credential store" : "Paste the client access key"}
              onChange={(event) => updateDraft({ accessKey: event.target.value.trim() })}
            />
          </label>
          <p className="field-note wide">
              {draft.accessKeySaved
                ? "Saved keys are owned by the Rust backend and OS credential store. Leave blank to keep the saved key."
                : "In Tauri mode the key is stored by the Rust backend on save, not persisted in WebView storage."}
          </p>
        </div>
        <div className="button-row">
          <button className="button primary" onClick={saveDraft}>
            <Save size={16} />
            Save
          </button>
          <button className="button" onClick={addProfile}>
            <Plus size={16} />
            New
          </button>
          <button className="button ghost" onClick={deleteProfile} disabled={profiles.length === 1}>
            <Trash2 size={16} />
            Delete
          </button>
        </div>
      </section>

      <section className="panel">
        <SectionHeader icon={FileDown} title="Safe Export" />
        <pre className="profile-preview">{JSON.stringify(scrubProfile(draft), null, 2)}</pre>
      </section>
    </div>
  );
}

function ProxyView({
  draft,
  updateDraft,
  snapshot,
  onCopy,
  onSetSystemProxy,
  onRestoreSystemProxy,
}: {
  draft: TrajectoryProfile;
  updateDraft: (patch: Partial<TrajectoryProfile>) => void;
  snapshot: RuntimeSnapshot;
  onCopy: (value: string) => void;
  onSetSystemProxy: () => Promise<void>;
  onRestoreSystemProxy: () => Promise<void>;
}) {
  const updateSocks = (patch: Partial<TrajectoryProfile["socks"]>) =>
    updateDraft({ socks: { ...draft.socks, ...patch } });
  const updateHttp = (patch: Partial<TrajectoryProfile["http"]>) =>
    updateDraft({ http: { ...draft.http, ...patch } });

  return (
    <div className="content-grid two-column">
      <section className="panel">
        <SectionHeader icon={Network} title="Local Listeners" />
        <div className="listener-grid">
          <ListenerEditor title="SOCKS5" endpoint={draft.socks} onChange={updateSocks} locked />
          <ListenerEditor title="HTTP" endpoint={draft.http} onChange={updateHttp} />
        </div>
      </section>

      <section className="panel">
        <SectionHeader icon={Lock} title="LAN Sharing Gate" />
        <p className="panel-copy">
          LAN binding works by changing the listener host to `0.0.0.0`. Keep it local unless you
          control the network. Built-in per-client LAN auth is still a platform-helper task.
        </p>
        <label className="check-row">
          <input
            type="checkbox"
            checked={draft.allowLanWithoutAuth}
            onChange={(event) => updateDraft({ allowLanWithoutAuth: event.target.checked })}
          />
          Allow non-localhost binding without local proxy auth
        </label>
        <EndpointRow
          label="SOCKS endpoint"
          value={`${draft.socks.host}:${draft.socks.port}`}
          command={`${draft.socks.host}:${draft.socks.port}`}
          onCopy={onCopy}
        />
        <EndpointRow
          label="HTTP endpoint"
          value={`${draft.http.host}:${draft.http.port}`}
          command={`${draft.http.host}:${draft.http.port}`}
          onCopy={onCopy}
        />
      </section>

      <section className="panel">
        <SectionHeader icon={Globe2} title="System Proxy" />
        <p className="panel-copy">
          Manual proxy is safest. System proxy integration applies OS user proxy settings for the
          connected profile and can clear the settings it applies.
        </p>
        <div className="button-row">
          <button
            className="button primary"
            onClick={() => void onSetSystemProxy()}
            disabled={snapshot.phase !== "connected"}
          >
            <Globe2 size={16} />
            Set System Proxy
          </button>
          <button className="button" onClick={() => void onRestoreSystemProxy()}>
            <ListRestart size={16} />
            Clear System Proxy
          </button>
        </div>
      </section>
    </div>
  );
}

function ResolversView({
  draft,
  updateDraft,
}: {
  draft: TrajectoryProfile;
  updateDraft: (patch: Partial<TrajectoryProfile>) => void;
}) {
  return (
    <div className="content-grid">
      <section className="panel">
        <SectionHeader icon={RadioTower} title="Resolver Set" />
        <textarea
          className="resolver-box"
          value={draft.resolvers.join("\n")}
          spellCheck={false}
          onChange={(event) =>
            updateDraft({
              resolvers: event.target.value
                .split(/\r?\n/)
                .map((line) => line.trim())
                .filter(Boolean),
            })
          }
        />
      </section>
      <section className="panel">
        <SectionHeader icon={DatabaseZap} title="Admission Control" />
        <div className="form-grid">
          <label>
            Resolver file
            <input
              value={draft.resolverFile ?? ""}
              placeholder="/path/to/dnses.txt"
              onChange={(event) => updateDraft({ resolverFile: event.target.value || undefined })}
            />
          </label>
          <label>
            Resolver SOCKS gate
            <input
              value={draft.resolverSocksProxy ?? ""}
              placeholder="127.0.0.1:11092"
              onChange={(event) => updateDraft({ resolverSocksProxy: event.target.value || undefined })}
            />
          </label>
          <label>
            Cohort size
            <input
              type="number"
              min={1}
              value={draft.resolverCohortSize ?? ""}
              onChange={(event) =>
                updateDraft({
                  resolverCohortSize: event.target.value ? Number(event.target.value) : undefined,
                })
              }
            />
          </label>
          <label>
            Minimum admitted
            <input
              type="number"
              min={1}
              value={draft.resolverAdmissionMin}
              onChange={(event) => updateDraft({ resolverAdmissionMin: Number(event.target.value) })}
            />
          </label>
        </div>
      </section>
    </div>
  );
}

function DiagnosticsView({
  snapshot,
  profile,
}: {
  snapshot: RuntimeSnapshot;
  profile: TrajectoryProfile;
}) {
  return (
    <div className="content-grid">
      <section className="panel">
        <SectionHeader icon={TerminalSquare} title="Runtime Log" />
        <pre className="log-view">
          {snapshot.logLines.length > 0
            ? snapshot.logLines.join("\n")
            : "No runtime output yet."}
        </pre>
      </section>
      <section className="panel">
        <SectionHeader icon={ListRestart} title="Diagnostics" />
        <dl className="definition-grid">
          <dt>Admission report</dt>
          <dd>{profile.admissionReport ? "enabled" : "disabled"}</dd>
          <dt>Poll interval</dt>
          <dd>{profile.pollIntervalMs} ms</dd>
          <dt>Started at</dt>
          <dd>{snapshot.startedAt ?? "not running"}</dd>
          <dt>Last error</dt>
          <dd>{snapshot.lastError ?? "none"}</dd>
        </dl>
      </section>
    </div>
  );
}

function VpnView({ capabilities }: { capabilities: RuntimeSnapshot["capabilities"] }) {
  return (
    <div className="content-grid">
      <section className="panel">
        <SectionHeader icon={Shield} title="VPN Mode" />
        <p className="panel-copy">
          Proxy mode is live. Whole-device VPN requires platform-native packet adapters:
          Android `VpnService`, macOS Network Extension, Windows Wintun, and Linux TUN.
          Those are tracked as platform work so the UI does not claim unsafe coverage.
        </p>
        <div className="capability-grid">
          <Capability label="Desktop proxy" state={capabilities.proxyMode} />
          <Capability label="LAN proxy" state={capabilities.lanSharing} />
          <Capability label="System proxy" state={capabilities.systemProxy} />
          <Capability label="Whole-device VPN" state={capabilities.vpnMode} />
          <Capability label="Android VPN" state={capabilities.androidVpn} />
        </div>
      </section>
    </div>
  );
}

function SettingsView({
  draft,
  updateDraft,
}: {
  draft: TrajectoryProfile;
  updateDraft: (patch: Partial<TrajectoryProfile>) => void;
}) {
  return (
    <div className="content-grid two-column">
      <section className="panel">
        <SectionHeader icon={SlidersHorizontal} title="Transport Knobs" />
        <div className="form-grid">
          <label>
            Client mode
            <select
              value={draft.transportMode}
              onChange={(event) =>
                updateDraft({
                  transportMode: event.target.value as TrajectoryProfile["transportMode"],
                })
              }
            >
              <option value="secure">Secure</option>
              <option value="velocity">Velocity</option>
              <option value="resilient">Resilient</option>
              <option value="frontier">Frontier</option>
            </select>
          </label>
          <label>
            Resolver transport
            <select
              value={draft.resolverTransport}
              onChange={(event) =>
                updateDraft({
                  resolverTransport: event.target.value as TrajectoryProfile["resolverTransport"],
                })
              }
            >
              <option value="auto">Auto</option>
              <option value="udp">UDP only</option>
              <option value="tcp">TCP only</option>
            </select>
          </label>
          <label>
            DNS max payload
            <input
              type="number"
              min={512}
              max={4096}
              value={draft.dnsMaxPayload}
              onChange={(event) => updateDraft({ dnsMaxPayload: Number(event.target.value) })}
            />
          </label>
          <label>
            Failure poll interval
            <input
              type="number"
              min={1}
              value={draft.pollIntervalMs}
              onChange={(event) => updateDraft({ pollIntervalMs: Number(event.target.value) })}
            />
          </label>
        </div>
        <label className="check-row">
          <input
            type="checkbox"
            checked={draft.admissionReport}
            onChange={(event) => updateDraft({ admissionReport: event.target.checked })}
          />
          Write resolver admission report while connecting
        </label>
      </section>
    </div>
  );
}

function ListenerEditor({
  title,
  endpoint,
  onChange,
  locked,
}: {
  title: string;
  endpoint: { enabled: boolean; host: string; port: number };
  onChange: (patch: Partial<typeof endpoint>) => void;
  locked?: boolean;
}) {
  return (
    <div className="listener-card">
      <div className="listener-title">
        <strong>{title}</strong>
        <label className="switch">
          <input
            type="checkbox"
            checked={endpoint.enabled}
            disabled={locked}
            onChange={(event) => onChange({ enabled: event.target.checked })}
          />
          <span />
        </label>
      </div>
      <label>
        Host
        <input value={endpoint.host} onChange={(event) => onChange({ host: event.target.value })} />
      </label>
      <label>
        Port
        <input
          type="number"
          min={1}
          max={65535}
          value={endpoint.port}
          onChange={(event) => onChange({ port: Number(event.target.value) })}
        />
      </label>
    </div>
  );
}

function SectionHeader({ icon: Icon, title }: { icon: typeof Activity; title: string }) {
  return (
    <div className="section-header">
      <Icon size={18} />
      <h2>{title}</h2>
    </div>
  );
}

function StatusPill({ phase }: { phase: ConnectionPhase }) {
  return <span className={`status-pill ${phaseClass[phase]}`}>{phaseLabels[phase]}</span>;
}

function Metric({
  label,
  value,
  copy,
  onCopy,
}: {
  label: string;
  value: string;
  copy?: boolean;
  onCopy?: (value: string) => void;
}) {
  return (
    <div className="metric">
      <span>{label}</span>
      <strong title={value}>{value}</strong>
      {copy && onCopy && value !== "disabled" && (
        <button className="icon-button" aria-label={`Copy ${label}`} onClick={() => onCopy(value)}>
          <Copy size={15} />
        </button>
      )}
    </div>
  );
}

function EndpointRow({
  label,
  value,
  command,
  onCopy,
}: {
  label: string;
  value: string;
  command: string;
  onCopy: (value: string) => void;
}) {
  return (
    <div className="endpoint-row">
      <div>
        <strong>{label}</strong>
        <span>{value}</span>
      </div>
      <code>{command}</code>
      <button className="icon-button" aria-label={`Copy ${label}`} onClick={() => onCopy(command)}>
        <Copy size={15} />
      </button>
    </div>
  );
}

function Capability({ label, state }: { label: string; state: CapabilityState }) {
  return (
    <div className="capability">
      <span className={`cap-dot ${state}`} />
      <strong>{label}</strong>
      <span>{state}</span>
    </div>
  );
}

function validateProfile(profile: TrajectoryProfile) {
  const warnings: string[] = [];
  if (!profile.domain) warnings.push("Set the authoritative tunnel domain before connecting.");
  if (!profile.accessKey && !profile.accessKeySaved) {
    warnings.push("Paste the client access key before connecting.");
  }
  if (!profile.socks.enabled) {
    warnings.push("SOCKS5 listener is required by the current client runtime.");
  }
  if (!profile.resolverFile && profile.resolvers.length === 0) {
    warnings.push("Add resolvers or a resolver file.");
  }
  if (!["secure", "velocity", "resilient", "frontier"].includes(profile.transportMode)) {
    warnings.push("Choose a valid client mode.");
  }
  if (!["auto", "udp", "tcp"].includes(profile.resolverTransport)) {
    warnings.push("Choose a valid resolver transport.");
  }
  for (const endpoint of [profile.socks, profile.http]) {
    if (endpoint.enabled && endpoint.host !== "127.0.0.1" && !profile.allowLanWithoutAuth) {
      warnings.push("LAN binding requires explicit confirmation in Proxy.");
    }
  }
  return warnings;
}

function copyText(value: string) {
  void navigator.clipboard?.writeText(value);
}
