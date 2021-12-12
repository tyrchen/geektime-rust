use anyhow::Result;
use kv6::{
    ClientConfig, ClientTlsConfig, GeneralConfig, LogConfig, RotationConfig, ServerConfig,
    ServerTlsConfig, StorageConfig,
};
use std::fs;

fn main() -> Result<()> {
    const CA_CERT: &str = include_str!("../fixtures/ca.cert");
    const SERVER_CERT: &str = include_str!("../fixtures/server.cert");
    const SERVER_KEY: &str = include_str!("../fixtures/server.key");

    let general_config = GeneralConfig {
        addr: "127.0.0.1:9527".into(),
    };
    let server_config = ServerConfig {
        storage: StorageConfig::SledDb("/tmp/kv_server".into()),
        general: general_config.clone(),
        tls: ServerTlsConfig {
            cert: SERVER_CERT.into(),
            key: SERVER_KEY.into(),
            ca: None,
        },
        log: LogConfig {
            path: "/tmp/kv-log".into(),
            rotation: RotationConfig::Daily,
        },
    };

    fs::write(
        "fixtures/server.conf",
        toml::to_string_pretty(&server_config)?,
    )?;

    let client_config = ClientConfig {
        general: general_config,

        tls: ClientTlsConfig {
            identity: None,
            ca: Some(CA_CERT.into()),
            domain: "kvserver.acme.inc".into(),
        },
    };

    fs::write(
        "fixtures/client.conf",
        toml::to_string_pretty(&client_config)?,
    )?;

    Ok(())
}
