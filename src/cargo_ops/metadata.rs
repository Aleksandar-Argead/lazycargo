use cargo_metadata::{DependencyKind, MetadataCommand};
use color_eyre::eyre::{Result, eyre};

use crate::models::dependency::Dependency;

pub fn load_dependencies() -> Result<Vec<Dependency>> {
    let metadata = MetadataCommand::new().exec()?;

    let root = metadata
        .root_package()
        .ok_or_else(|| eyre!("No root package found"))?;

    let mut deps = Vec::new();

    for dep in &root.dependencies {
        let resolved_pkg = metadata
            .packages
            .iter()
            .find(|p| p.name == dep.name && dep.req.matches(&p.version))
            .ok_or_else(|| eyre!("Resolved package not found for {}", dep.name))?;

        let locked_version = Some(resolved_pkg.version.to_string());

        let version = metadata
            .packages
            .iter()
            .filter(|p| p.name == dep.name)
            .max_by_key(|p| &p.version)
            .map(|p| p.version.to_string())
            .unwrap_or_else(|| "?".to_string());

        let kind = match dep.kind {
            DependencyKind::Normal => "normal",
            DependencyKind::Development => "dev",
            DependencyKind::Build => "build",
            _ => "unknown",
        }
        .to_string();

        let kind = dep
            .target
            .as_ref()
            .map_or(kind, |t| format!("target({})", t));

        let mut all_features: Vec<String> = resolved_pkg.features.keys().cloned().collect();
        all_features.sort();

        let mut enabled_sorted: Vec<String> = all_features
            .iter()
            .filter(|f| dep.features.contains(f))
            .cloned()
            .collect();
        enabled_sorted.sort();

        let mut disabled_sorted: Vec<String> = all_features
            .iter()
            .filter(|f| !dep.features.contains(f))
            .cloned()
            .collect();
        disabled_sorted.sort();

        deps.push(Dependency {
            name: dep.name.clone(),
            version,
            locked_version,
            kind,
            enabled_features: enabled_sorted,
            disabled_features: disabled_sorted,
            all_features,
        });
    }

    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}
