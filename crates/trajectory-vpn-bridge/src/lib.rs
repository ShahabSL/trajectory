#![cfg_attr(not(target_os = "android"), allow(improper_ctypes_definitions))]

use jni::{
    errors::LogErrorAndDefault,
    objects::{JClass, JString},
    sys::{jboolean, jint},
    Env, EnvUnowned,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Mutex,
};
use tun2proxy::{general_run_async, ArgDns, ArgProxy, ArgVerbosity, Args, CancellationToken};

static RUNNING: Mutex<Option<CancellationToken>> = Mutex::new(None);

/// Runs the TUN-to-SOCKS bridge on an Android `VpnService` file descriptor.
///
/// # Safety
///
/// Android calls this through JNI. `dns_server` must be a valid Java string
/// for the duration of the call, and `tun_fd` must be a detached TUN file
/// descriptor that this bridge is allowed to close when it stops.
#[no_mangle]
pub unsafe extern "system" fn Java_app_trajectory_android_TrajectoryVpnBridge_run(
    mut env: EnvUnowned<'_>,
    _class: JClass<'_>,
    tun_fd: jint,
    socks_port: jint,
    dns_server: JString<'_>,
    mtu: jint,
    max_sessions: jint,
    ipv6_enabled: jboolean,
) -> jint {
    env.with_env(|env: &mut Env| -> Result<jint, jni::errors::Error> {
        let dns_server = dns_server.try_to_string(env)?;
        Ok(run_bridge(
            tun_fd,
            socks_port,
            &dns_server,
            mtu,
            max_sessions,
            ipv6_enabled,
        ))
    })
    .resolve::<LogErrorAndDefault>()
}

/// Stops the active TUN-to-SOCKS bridge, if one is running.
///
/// # Safety
///
/// Android calls this through JNI. The caller must ensure the containing VM is
/// still alive while the JNI frame is active.
#[no_mangle]
pub unsafe extern "system" fn Java_app_trajectory_android_TrajectoryVpnBridge_stop(
    _env: EnvUnowned<'_>,
    _class: JClass<'_>,
) -> jint {
    stop_bridge()
}

fn run_bridge(
    tun_fd: i32,
    socks_port: i32,
    dns_server: &str,
    mtu: i32,
    max_sessions: i32,
    ipv6_enabled: bool,
) -> i32 {
    let Ok(args) = build_args(
        tun_fd,
        socks_port,
        dns_server,
        mtu,
        max_sessions,
        ipv6_enabled,
    ) else {
        return -10;
    };

    let token = CancellationToken::new();
    {
        let Ok(mut running) = RUNNING.lock() else {
            return -11;
        };
        if running.is_some() {
            return -12;
        }
        *running = Some(token.clone());
    }

    let mtu = args.mtu;
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("trajectory-vpn")
        .build()
        .map_err(|_| -13)
        .and_then(|runtime| {
            runtime
                .block_on(general_run_async(args, mtu, false, token))
                .map(|_| 0)
                .map_err(|_| -14)
        });

    if let Ok(mut running) = RUNNING.lock() {
        running.take();
    }

    result.unwrap_or_else(|code| code)
}

fn stop_bridge() -> i32 {
    let Ok(mut running) = RUNNING.lock() else {
        return -1;
    };
    if let Some(token) = running.take() {
        token.cancel();
        0
    } else {
        -2
    }
}

fn build_args(
    tun_fd: i32,
    socks_port: i32,
    dns_server: &str,
    mtu: i32,
    max_sessions: i32,
    ipv6_enabled: bool,
) -> Result<Args, String> {
    if tun_fd < 0 {
        return Err("invalid TUN file descriptor".to_string());
    }
    if !(1..=65535).contains(&socks_port) {
        return Err("invalid SOCKS port".to_string());
    }
    if !(576..=9000).contains(&mtu) {
        return Err("invalid VPN MTU".to_string());
    }
    if !(16..=20000).contains(&max_sessions) {
        return Err("invalid max session count".to_string());
    }
    let dns_addr = dns_server
        .parse::<IpAddr>()
        .map_err(|_| "invalid DNS server address".to_string())?;
    let proxy_addr = SocketAddr::from(([127, 0, 0, 1], socks_port as u16));
    let proxy = ArgProxy::try_from(format!("socks5://{proxy_addr}").as_str())
        .map_err(|error| format!("invalid proxy address: {error}"))?;

    let mut args = Args {
        proxy,
        dns: ArgDns::Virtual,
        dns_addr,
        mtu: mtu as u16,
        max_sessions: max_sessions as usize,
        ipv6_enabled,
        verbosity: ArgVerbosity::Warn,
        setup: false,
        ..Args::default()
    };
    #[cfg(unix)]
    {
        args.tun_fd = Some(tun_fd);
        args.close_fd_on_drop = Some(true);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_args_use_virtual_dns_by_default() {
        let args = build_args(7, 7000, "1.1.1.1", 1500, 512, false).unwrap();
        assert_eq!(args.dns, ArgDns::Virtual);
        assert_eq!(args.dns_addr, "1.1.1.1".parse::<IpAddr>().unwrap());
        assert_eq!(
            args.proxy.addr,
            "127.0.0.1:7000".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(args.mtu, 1500);
        assert_eq!(args.max_sessions, 512);
        assert!(!args.ipv6_enabled);
        #[cfg(unix)]
        {
            assert_eq!(args.tun_fd, Some(7));
            assert_eq!(args.close_fd_on_drop, Some(true));
        }
    }

    #[test]
    fn bridge_args_reject_bad_inputs() {
        assert!(build_args(-1, 7000, "1.1.1.1", 1500, 512, false).is_err());
        assert!(build_args(7, 0, "1.1.1.1", 1500, 512, false).is_err());
        assert!(build_args(7, 7000, "not-ip", 1500, 512, false).is_err());
        assert!(build_args(7, 7000, "1.1.1.1", 100, 512, false).is_err());
        assert!(build_args(7, 7000, "1.1.1.1", 1500, 1, false).is_err());
    }
}
