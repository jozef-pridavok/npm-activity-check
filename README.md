# NPM Activity Check

A fast command-line tool to check if NPM packages are actively maintained.

## Quick Start

```bash
# Basic usage
npm-activity-check react

# With JSON output
npm-activity-check react --format json

# Extract specific values
npm-activity-check react --format field:package_alive
# Output: true

npm-activity-check react --format field:latest_version
# Output: 18.2.0
```

## Installation

### From Source

```bash
git clone https://github.com/jozef-pridavok/npm-activity-check.git
cd npm-activity-check
cargo build --release
```

Binary will be at `target/release/npm-activity-check`

### Prerequisites

- Rust 1.70+

## Usage

### Basic Examples

```bash
# Check a package
npm-activity-check lodash

# Get JSON output
npm-activity-check lodash --format json

# Extract specific values
npm-activity-check lodash --format field:downloads_last_month
# Output: 45123456

npm-activity-check lodash --format field:package_alive  
# Output: true

npm-activity-check lodash --format field:latest_version
# Output: 4.17.21
```

### Available Fields

| Field | Description | Example |
|-------|-------------|---------|
| `name` | Package name | `lodash` |
| `latest_version` | Latest version | `4.17.21` |
| `total_versions` | Number of versions | `147` |
| `last_publish_date` | Latest publish date | `2021-05-07 16:15:12 UTC` |
| `downloads_last_week` | Weekly downloads | `4523112` |
| `downloads_last_month` | Monthly downloads | `45231123` |
| `maintainers_count` | Number of maintainers | `3` |
| `has_recent_activity` | Recent activity? | `true` |
| `package_alive` | Is package active? | `true` |
| `description` | Package description | `Lodash modular utilities.` |
| `homepage` | Package homepage | `https://lodash.com/` |
| `repository_url` | Repository URL | `git+https://github.com/lodash/lodash.git` |
| `license` | Package license | `MIT` |
| `keywords` | Package keywords | `modules, stdlib, util` |

### Configuration File

Create `config.toml`:

```toml
format = "json"
max_days = 120
min_weekly_downloads = 500
min_monthly_downloads = 2000
min_versions = 5
min_maintainers = 1
```

Use with:

```bash
npm-activity-check react --config-file config.toml
```

### History Tracking

Track changes over time:

```bash
# First run - saves current state
npm-activity-check react --history /tmp/react.json

# Later runs - compares with saved state
npm-activity-check react --history /tmp/react.json --check downloads_last_month
echo "Exit code: $?"
# Exit code: 0 = no change, N = change magnitude

# Use exit code in shell scripts
if npm-activity-check react --history /tmp/react.json --check package_alive; then
    echo "No change in package status"
else
    echo "Package status changed! (exit code: $?)"
fi
```

### Common Use Cases

#### Check if dependency is maintained
```bash
npm-activity-check express --format field:package_alive
```

#### Monitor for new versions
```bash
npm-activity-check vue --history /tmp/vue.json --check latest_version
if [ $? -eq 1 ]; then
    echo "🎉 New Vue version detected!"
    # Send notification, update CI, etc.
fi
```

#### Monitor package in script
```bash
#!/bin/bash
# Check if package is alive
if npm-activity-check my-package --format field:package_alive | grep -q "false"; then
    echo "WARNING: Package appears inactive!"
fi

# Monitor for download changes
npm-activity-check my-package --history /tmp/package.json --check downloads_last_month
if [ $? -gt 1000 ]; then
    echo "📈 Package downloads increased significantly!"
fi
```

#### Bulk analysis
```bash
# analyze-packages.sh
packages=("react" "vue" "angular" "svelte")

for package in "${packages[@]}"; do
    echo -n "$package: "
    if npm-activity-check $package --format field:package_alive | grep -q "true"; then
        echo "✅ ACTIVE"
    else
        echo "❌ INACTIVE"
    fi
done

# Monitor multiple packages for changes
for package in "${packages[@]}"; do
    npm-activity-check $package --history "/tmp/${package}.json" --check package_alive
    if [ $? -eq 1 ]; then
        echo "⚠️  ${package}: Status changed"
    fi
done
```

## How It Works

The tool analyzes packages using multiple criteria:

- **Recent activity** (last publish date)
- **Download popularity** (weekly/monthly downloads)
- **Package maturity** (total versions)
- **Maintenance** (number of maintainers)

A package is considered "alive" if it has:
- Recent activity (published within max_days), OR  
- Good download numbers AND sufficient versions/maintainers

## Command Line Options

```
npm-activity-check [OPTIONS] <PACKAGE>

Options:
  --format <FORMAT>              Output format: default, json, field:name
  --config-file <FILE>           Load settings from TOML file
  --history <FILE>               Save/load run history
  --check <FIELD>                Check field changes (sets exit code)
  --max-days <N>                 Maximum days since last publish (default: 90)
  --min-weekly-downloads <N>     Minimum weekly downloads (default: 1000)
  --min-monthly-downloads <N>    Minimum monthly downloads (default: 5000)
  --min-versions <N>             Minimum versions threshold (default: 10)
  --min-maintainers <N>          Minimum maintainers threshold (default: 1)
  --verbose                      Show detailed output
  --help                         Show help
```

### Exit Codes (with --check)

Exit code represents **actual change magnitude**:

- **0** = No change detected
- **Positive number** = Magnitude of change (depends on field type)

**Change calculation by field type:**

| Field Type | Change Measurement | Exit Code = Actual Change |
|------------|-------------------|--------------------------|
| Numbers (`downloads_last_month`) | Absolute difference | Exit code = |new - old| |
| Booleans (`package_alive`) | Status flip | 0 = same, 1 = different |
| Dates (`last_publish_date`) | **Days difference** | **Exit code = days between publishes** |
| Strings (`latest_version`) | Version change | 0 = same version, 1 = new version |

**Examples:**

```bash
# Numbers: Get absolute change
npm-activity-check react --history /tmp/react.json --check downloads_last_month
echo "Download change: $?"
# 0 = no change, 12345 = 12345 more/fewer downloads

# Versions: Detect new version
npm-activity-check react --history /tmp/react.json --check latest_version
echo "Version change status: $?"
# 0 = same version, 1 = new version available
```

## Data Sources

- Package metadata: https://registry.npmjs.org/
- Download statistics: https://api.npmjs.org/downloads/

## License

MIT License - see LICENSE file for details.