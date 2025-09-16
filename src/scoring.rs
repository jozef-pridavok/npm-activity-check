use chrono::{DateTime, Utc};
use crate::config::Config;

pub struct PackageScorer;

impl PackageScorer {
    pub fn new() -> Self {
        Self
    }

    pub fn is_package_alive(
        &self,
        last_publish_date: &DateTime<Utc>,
        total_versions: usize,
        maintainers_count: usize,
        weekly_downloads: u64,
        monthly_downloads: u64,
        config: &Config,
    ) -> bool {
        let has_recent_activity = self.has_recent_activity(last_publish_date, config.max_days);
        let has_sufficient_downloads = self.has_sufficient_downloads(weekly_downloads, monthly_downloads, config);
        let has_sufficient_versions = total_versions >= config.min_versions;
        let has_sufficient_maintainers = maintainers_count >= config.min_maintainers;

        // Package is considered alive if:
        // 1. Has recent activity (published within max_days), OR
        // 2. Has good download numbers AND sufficient versions/maintainers
        has_recent_activity || (has_sufficient_downloads && has_sufficient_versions && has_sufficient_maintainers)
    }

    pub fn has_recent_activity(&self, last_publish_date: &DateTime<Utc>, max_days: i64) -> bool {
        let now = Utc::now();
        let threshold = now - chrono::Duration::days(max_days);
        *last_publish_date > threshold
    }

    pub fn has_sufficient_downloads(&self, weekly_downloads: u64, monthly_downloads: u64, config: &Config) -> bool {
        weekly_downloads >= config.min_weekly_downloads || monthly_downloads >= config.min_monthly_downloads
    }

    pub fn calculate_activity_score(
        &self,
        last_publish_date: &DateTime<Utc>,
        total_versions: usize,
        maintainers_count: usize,
        _weekly_downloads: u64,
        monthly_downloads: u64,
    ) -> f64 {
        let mut score = 0.0;

        // Recent activity score (0-40 points)
        let days_since_publish = (Utc::now() - *last_publish_date).num_days();
        let activity_score = if days_since_publish <= 30 {
            40.0
        } else if days_since_publish <= 90 {
            30.0
        } else if days_since_publish <= 180 {
            20.0
        } else if days_since_publish <= 365 {
            10.0
        } else {
            0.0
        };
        score += activity_score;

        // Download popularity score (0-30 points)
        let download_score = if monthly_downloads >= 100_000 {
            30.0
        } else if monthly_downloads >= 50_000 {
            25.0
        } else if monthly_downloads >= 10_000 {
            20.0
        } else if monthly_downloads >= 5_000 {
            15.0
        } else if monthly_downloads >= 1_000 {
            10.0
        } else if monthly_downloads >= 100 {
            5.0
        } else {
            0.0
        };
        score += download_score;

        // Version maturity score (0-15 points)
        let version_score = if total_versions >= 100 {
            15.0
        } else if total_versions >= 50 {
            12.0
        } else if total_versions >= 20 {
            10.0
        } else if total_versions >= 10 {
            7.0
        } else if total_versions >= 5 {
            5.0
        } else {
            2.0
        };
        score += version_score;

        // Maintainer score (0-15 points)
        let maintainer_score = if maintainers_count >= 10 {
            15.0
        } else if maintainers_count >= 5 {
            12.0
        } else if maintainers_count >= 3 {
            10.0
        } else if maintainers_count >= 2 {
            7.0
        } else if maintainers_count >= 1 {
            5.0
        } else {
            0.0
        };
        score += maintainer_score;

        score
    }
}