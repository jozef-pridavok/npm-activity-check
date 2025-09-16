use chrono::{DateTime, Utc};
use crate::config::Config;

pub struct PackageScorer;

impl PackageScorer {
    pub fn is_package_alive(
        last_publish_date: &DateTime<Utc>,
        total_versions: usize,
        maintainers_count: usize,
        weekly_downloads: u64,
        monthly_downloads: u64,
        config: &Config,
    ) -> bool {
        let has_recent_activity = Self::has_recent_activity(last_publish_date, config.max_days);
        let has_sufficient_downloads = Self::has_sufficient_downloads(weekly_downloads, monthly_downloads, config);
        let has_sufficient_versions = total_versions >= config.min_versions;
        let has_sufficient_maintainers = maintainers_count >= config.min_maintainers;

        // Package is considered alive if:
        // 1. Has recent activity (published within max_days), OR
        // 2. Has good download numbers AND sufficient versions/maintainers
        has_recent_activity || (has_sufficient_downloads && has_sufficient_versions && has_sufficient_maintainers)
    }

    pub fn has_recent_activity(last_publish_date: &DateTime<Utc>, max_days: i64) -> bool {
        let now = Utc::now();
        let threshold = now - chrono::Duration::days(max_days);
        *last_publish_date > threshold
    }

    pub fn has_sufficient_downloads(weekly_downloads: u64, monthly_downloads: u64, config: &Config) -> bool {
        weekly_downloads >= config.min_weekly_downloads || monthly_downloads >= config.min_monthly_downloads
    }

}