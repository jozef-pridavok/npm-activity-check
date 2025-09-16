use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::config::Config;
use crate::scoring::PackageScorer;
use crate::types::{NpmPackageInfo, PackageReport};

pub fn create_package_report(
    config: &Config,
    package_info: &NpmPackageInfo,
    latest_version: &str,
    last_publish_date: &DateTime<Utc>,
    weekly_downloads: u64,
    monthly_downloads: u64,
) -> PackageReport {
    let total_versions = package_info.versions.len();
    let maintainers_count = package_info.maintainers.as_ref()
        .and_then(|m| {
            if let Some(array) = m.as_array() {
                Some(array.len())
            } else {
                Some(1) // If it's not an array, assume it's a single maintainer
            }
        })
        .unwrap_or(0);
    
    let has_recent_activity = PackageScorer::has_recent_activity(last_publish_date, config.max_days);
    let package_alive = PackageScorer::is_package_alive(
        last_publish_date,
        total_versions,
        maintainers_count,
        weekly_downloads,
        monthly_downloads,
        config,
    );

    PackageReport {
        name: package_info.name.clone(),
        latest_version: latest_version.to_string(),
        total_versions,
        last_publish_date: *last_publish_date,
        downloads_last_week: weekly_downloads,
        downloads_last_month: monthly_downloads,
        maintainers_count,
        has_recent_activity,
        package_alive,
        description: package_info.description.clone(),
        homepage: package_info.homepage.clone(),
        repository_url: package_info.repository.as_ref()
            .and_then(|r| r.get("url"))
            .and_then(|url| url.as_str())
            .map(|s| s.to_string()),
        license: package_info.license.clone(),
        keywords: package_info.keywords.clone(),
    }
}

pub fn print_output(config: &Config, report: &PackageReport) -> Result<()> {
    let default_format = "default".to_string();
    let format = config.format.as_ref().unwrap_or(&default_format);

    match format.as_str() {
        "json" => print_json_output(report),
        "default" => print_default_output(report),
        field if field.starts_with("field:") => {
            let field_name = &field[6..]; // Remove "field:" prefix
            print_field_output(report, field_name)
        }
        _ => anyhow::bail!("Invalid output format: {}", format),
    }
}

fn print_json_output(report: &PackageReport) -> Result<()> {
    let json_output = serde_json::to_string_pretty(report)
        .context("Failed to serialize report to JSON")?;
    println!("{}", json_output);
    Ok(())
}

fn print_default_output(report: &PackageReport) -> Result<()> {
    println!("NPM Package: {}", report.name);
    println!("Latest Version: {}", report.latest_version);
    println!("Last Published: {}", report.last_publish_date.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Total Versions: {}", report.total_versions);
    println!("Downloads (Week): {}", format_number(report.downloads_last_week));
    println!("Downloads (Month): {}", format_number(report.downloads_last_month));
    println!("Maintainers: {}", report.maintainers_count);
    println!("Recent Activity: {}", if report.has_recent_activity { "✅ Yes" } else { "❌ No" });
    println!("Package Status: {}", if report.package_alive { "✅ ACTIVE" } else { "❌ INACTIVE" });
    
    if let Some(description) = &report.description {
        println!("Description: {}", description);
    }
    
    if let Some(homepage) = &report.homepage {
        println!("Homepage: {}", homepage);
    }
    
    if let Some(repo_url) = &report.repository_url {
        println!("Repository: {}", repo_url);
    }
    
    if let Some(license) = &report.license {
        println!("License: {}", license);
    }
    
    if let Some(keywords) = &report.keywords {
        if !keywords.is_empty() {
            println!("Keywords: {}", keywords.join(", "));
        }
    }

    Ok(())
}

fn print_field_output(report: &PackageReport, field_name: &str) -> Result<()> {
    let value = get_field_value(report, field_name)
        .with_context(|| format!("Field '{}' not found", field_name))?;
    
    match value {
        Value::String(s) => println!("{}", s),
        Value::Number(n) => println!("{}", n),
        Value::Bool(b) => println!("{}", b),
        Value::Array(arr) => {
            let strings: Vec<String> = arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            println!("{}", strings.join(", "));
        }
        Value::Null => println!("null"),
        _ => println!("{}", value),
    }
    
    Ok(())
}

fn get_field_value(report: &PackageReport, field_name: &str) -> Option<Value> {
    match field_name {
        "name" => Some(json!(report.name)),
        "latest_version" => Some(json!(report.latest_version)),
        "total_versions" => Some(json!(report.total_versions)),
        "last_publish_date" => Some(json!(report.last_publish_date.format("%Y-%m-%d %H:%M:%S UTC").to_string())),
        "downloads_last_week" => Some(json!(report.downloads_last_week)),
        "downloads_last_month" => Some(json!(report.downloads_last_month)),
        "maintainers_count" => Some(json!(report.maintainers_count)),
        "has_recent_activity" => Some(json!(report.has_recent_activity)),
        "package_alive" => Some(json!(report.package_alive)),
        "description" => Some(json!(report.description)),
        "homepage" => Some(json!(report.homepage)),
        "repository_url" => Some(json!(report.repository_url)),
        "license" => Some(json!(report.license)),
        "keywords" => Some(json!(report.keywords)),
        _ => None,
    }
}

fn format_number(num: u64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}k", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}