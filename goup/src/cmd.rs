use std::collections::HashMap;

use colored::Colorize;
use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use which::which;

use goup_misc::Toolchain;
use goup_misc::ToolchainFilter;
use goup_misc::Version;
use goup_misc::op;
use goup_misc::{Dir, consts};

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(windows)]
    println!("set {key}={value}");
    #[cfg(unix)]
    println!("export {key}={value}");
}

pub fn cmd_env() -> anyhow::Result<()> {
    print_env(
        consts::GOUP_HOME,
        &Dir::goup_home().unwrap_or_default().to_string_lossy(),
    );
    print_env(consts::GOUP_GO_HOST, &consts::go_host());
    print_env(
        consts::GOUP_GO_DOWNLOAD_BASE_URL,
        &consts::go_download_base_url(),
    );

    Ok(())
}

pub fn cmd_list() -> anyhow::Result<()> {
    let vers = op::list_go_version()?;
    if vers.is_empty() {
        println!("No Go is installed by goup.");
        if let Ok(go_bin) = which("go") {
            println!(" Using system Go {}.", go_bin.to_string_lossy());
        }
    } else {
        #[cfg(windows)]
        colored::control::set_virtual_terminal(true).unwrap();

        for v in vers {
            if v.active {
                println!("* {}", v.version.to_string().green());
            } else {
                println!("  {}", v.version);
            };
        }
    }

    Ok(())
}

pub fn cmd_search(filter: Option<String>, host: String) -> anyhow::Result<()> {
    let filter = filter.and_then(|s| s.parse().ok());
    let remote_versions = op::list_upstream_go_versions_filter(&host, filter)?;

    let local_versions = op::list_go_version()?;
    let mut v_a_map = HashMap::<String, bool>::new();
    for v in local_versions {
        v_a_map.insert(v.version.to_string(), v.active);
    }

    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    for v in remote_versions {
        if !v_a_map.contains_key(&v) {
            println!("  {v}");
            continue;
        }
        if v_a_map[&v] {
            println!("* {}", v.green());
        } else {
            println!("  {}", v.green());
        }
    }

    Ok(())
}

pub fn cmd_install(toolchain: String, host: &str) -> anyhow::Result<()> {
    let version = match toolchain.parse()? {
        Toolchain::Stable => op::get_upstream_latest_go_version(host)?,
        Toolchain::Unstable => {
            let version =
                op::list_upstream_go_versions_filter(host, Some(ToolchainFilter::Unstable))?;
            let version = version
                .last()
                .ok_or_else(|| anyhow::anyhow!("failed get latest unstable version"))?;
            version.to_string()
        }
        Toolchain::Beta => {
            let version = op::list_upstream_go_versions_filter(host, Some(ToolchainFilter::Beta))?;
            let version = version
                .last()
                .ok_or_else(|| anyhow::anyhow!("failed get latest beta version"))?;
            version.to_string()
        }
        Toolchain::Version(ver_req) => op::match_version_req(host, &ver_req)?,
        Toolchain::Nightly => {
            anyhow::bail!("gotip is no supported");
        }
    };

    let version = Version::normalize(&version);
    goup_downloader::install_go_version(&version)
}

pub fn cmd_remove(version: Vec<String>) -> anyhow::Result<()> {
    if !version.is_empty() {
        return op::remove_go_versions(&version);
    }

    let vers = op::list_go_version()?;
    if vers.is_empty() {
        anyhow::bail!("No go is installed");
    }
    let items: Vec<String> = vers.iter().map(|v| v.version.to_string()).collect();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select multiple version")
        .items(&items)
        .interact()?;
    if selection.is_empty() {
        anyhow::bail!("No item selected");
    }

    let vers: Vec<String> = selection.into_iter().map(|i| items[i].clone()).collect();
    op::remove_go_versions(&vers)
}

pub fn cmd_set(version: Option<String>) -> anyhow::Result<()> {
    if let Some(version) = version {
        return op::set_go_version(&version);
    }

    let vers = op::list_go_version()?;
    if vers.is_empty() {
        anyhow::bail!("Not any go is installed, Install it with `goup install`.");
    }

    let mut items = vec![];
    let mut pos = 0;
    for (i, v) in vers.iter().enumerate() {
        items.push(v.version.to_string());
        if v.active {
            pos = i;
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .items(&items)
        .default(pos)
        .interact()?;
    op::set_go_version(&items[selection])
}

pub fn cmd_clean(yes: bool) -> anyhow::Result<()> {
    let confirmation = yes
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to clean cache file?")
            .interact()?;
    if confirmation {
        op::remove_cache()?;
    } else {
        println!("Cancelled");
    }
    Ok(())
}
