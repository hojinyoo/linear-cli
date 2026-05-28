use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::sync::OnceLock;

use crate::commands::{
    api, attachments, auth, bulk, cache, comments, cycles, documents, export, favorites, git,
    history, import, initiatives, issues, labels, metrics, milestones, notifications,
    project_updates, projects, relations, roadmaps, search, sprint, statuses, sync, teams,
    templates, time, triage, uploads, users, views, webhooks,
};
use crate::output::SortOrder;

/// Output format for command results
#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq)]
pub enum OutputFormat {
    /// Display results as formatted tables (default)
    #[default]
    Table,
    /// Display results as raw JSON
    Json,
    /// Display results as NDJSON (one JSON object per line)
    Ndjson,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq)]
pub enum ColorChoice {
    #[default]
    Auto,
    Always,
    Never,
}

/// Global options for agentic/scripting use
#[derive(Debug, Clone, Copy, Default)]
pub struct AgentOptions {
    /// Suppress decorative output (headers, separators, tips)
    pub quiet: bool,
    /// Only output IDs of created/updated resources
    pub id_only: bool,
    /// Preview without making changes (where supported)
    pub dry_run: bool,
    /// Auto-confirm all prompts (deletes, destructive operations)
    pub yes: bool,
}

static YES_MODE: OnceLock<bool> = OnceLock::new();

pub fn set_yes_mode(yes: bool) {
    let _ = YES_MODE.set(yes);
}

pub fn is_yes() -> bool {
    YES_MODE.get().copied().unwrap_or(false)
}

#[derive(Parser)]
#[command(name = "linear-cli")]
#[command(
    about = "A powerful CLI for Linear.app - manage issues, projects, and more from your terminal"
)]
#[command(version)]
#[command(after_help = r#"QUICK START:
    1. Get your API key from https://linear.app/settings/api
    2. Configure the CLI:
       printf '%s\n' "$LINEAR_API_KEY" | linear config set-key
    3. List your issues:
       linear issues list
    4. Create an issue:
       linear issues create "Fix bug" --team ENG --priority 2

COMMON FLAGS:
    --output table|json|ndjson    Output format (default: table)
    --color-mode auto|always|never   Color output control
    --no-color                    Disable color output
    --width N                     Max table column width
    --no-truncate                 Disable table truncation
    --quiet                       Reduce decorative output
    --format TEMPLATE             Template output (e.g. '{{identifier}} {{title}}')
    --filter field=value          Filter results (=, !=, ~= operators; dot paths; case-insensitive)
    --limit N                     Limit list/search results
    --page-size N                 Page size for list/search
    --after CURSOR                Pagination cursor (after)
    --before CURSOR               Pagination cursor (before)
    --all                         Fetch all pages
    --profile NAME                Use named profile
    --schema                      Print JSON schema version and exit
    --cache-ttl N                 Cache TTL in seconds
    --no-cache                    Disable cache usage
    --yes                         Auto-confirm all prompts

For more info on a command, run: linear <command> --help"#)]
pub struct Cli {
    /// Output format (table or json)
    #[arg(
        short,
        long,
        global = true,
        env = "LINEAR_CLI_OUTPUT",
        default_value = "table"
    )]
    pub output: OutputFormat,

    /// Suppress decorative output (headers, separators, tips) - for scripting
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Only output IDs of created/updated resources - for chaining commands
    #[arg(long, global = true)]
    pub id_only: bool,

    /// Color output: auto, always, or never
    #[arg(
        long = "color-mode",
        global = true,
        value_enum,
        default_value = "auto",
        conflicts_with = "no_color"
    )]
    pub color_mode: ColorChoice,

    /// Disable color output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Max column width for table output (default: 50)
    #[arg(long, global = true)]
    pub width: Option<usize>,

    /// Disable truncation for table output
    #[arg(long, global = true)]
    pub no_truncate: bool,

    /// Emit compact JSON without pretty formatting
    #[arg(long, global = true)]
    pub compact: bool,

    /// Limit JSON output to specific fields (comma-separated, supports dot paths)
    #[arg(long, global = true, value_delimiter = ',')]
    pub fields: Vec<String>,

    /// Sort JSON array output by a field (default: identifier/id when available)
    #[arg(long, global = true)]
    pub sort: Option<String>,

    /// Sort order for JSON array output
    #[arg(long, global = true, value_enum, default_value = "asc")]
    pub order: SortOrder,

    /// Override workspace profile for this invocation
    #[arg(long, global = true, env = "LINEAR_CLI_PROFILE")]
    pub profile: Option<String>,

    /// Output using a template (e.g. '{{identifier}} {{title}}')
    #[arg(long, global = true)]
    pub format: Option<String>,

    /// Filter results (field=value, field!=value, field~=value).
    /// Supports dot-notation for nested fields (e.g. state.name=Done).
    /// ~= is a case-insensitive "contains" match. All comparisons are case-insensitive.
    /// Multiple --filter flags are combined with AND logic.
    #[arg(long, global = true)]
    pub filter: Vec<String>,

    /// Exit with non-zero status when a list is empty
    #[arg(long, global = true)]
    pub fail_on_empty: bool,

    /// Max results to return for list/search commands
    #[arg(long, global = true)]
    pub limit: Option<usize>,

    /// Pagination cursor to start after
    #[arg(long, global = true)]
    pub after: Option<String>,

    /// Pagination cursor to end before
    #[arg(long, global = true)]
    pub before: Option<String>,

    /// Page size per request for list/search commands
    #[arg(long, global = true)]
    pub page_size: Option<usize>,

    /// Fetch all pages for list/search commands
    #[arg(long, global = true)]
    pub all: bool,

    /// Override cache TTL in seconds
    #[arg(long, global = true, env = "LINEAR_CLI_CACHE_TTL")]
    pub cache_ttl: Option<u64>,

    /// Disable cache usage for this invocation
    #[arg(long, global = true, env = "LINEAR_CLI_NO_CACHE")]
    pub no_cache: bool,

    /// Preview without making changes where supported
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Auto-confirm all prompts (deletes, destructive operations)
    #[arg(long, global = true, env = "LINEAR_CLI_YES")]
    pub yes: bool,

    /// Number of retries for failed API requests (with exponential backoff)
    #[arg(long, global = true, default_value = "0")]
    pub retry: u32,

    /// Print JSON schema version info and exit
    #[arg(long, global = true)]
    pub schema: bool,

    /// Disable pager for output (default: auto-detect from terminal)
    #[arg(long, global = true, env = "LINEAR_CLI_NO_PAGER")]
    pub no_pager: bool,

    /// Show common tasks and examples
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayOptions {
    pub width: Option<usize>,
    pub no_truncate: bool,
}

impl DisplayOptions {
    pub fn max_width(&self, default: usize) -> Option<usize> {
        if self.no_truncate {
            None
        } else {
            Some(self.width.unwrap_or(default))
        }
    }
}

pub static DISPLAY_OPTIONS: OnceLock<DisplayOptions> = OnceLock::new();

pub fn set_cli_state(display: DisplayOptions) {
    let _ = DISPLAY_OPTIONS.set(display);
}

pub fn display_options() -> DisplayOptions {
    DISPLAY_OPTIONS.get().copied().unwrap_or_default()
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show common tasks and examples
    #[command(alias = "tasks")]
    Common,
    /// Show agent-focused capabilities and examples
    Agent,
    /// Check for and install the latest released version of linear-cli
    #[command(after_help = r#"EXAMPLES:
    linear-cli update
    linear-cli update --check"#)]
    Update {
        /// Check whether a newer release exists without installing it
        #[arg(long)]
        check: bool,
    },
    /// Manage issue attachments - list, create, update, delete, link URLs
    #[command(alias = "att")]
    #[command(after_help = r#"EXAMPLES:
    linear attachments list SCW-123          # List attachments on issue
    linear att get ATTACHMENT_ID             # View attachment details
    linear att create SCW-123 -T "Doc" -u https://example.com
    linear att link-url SCW-123 https://example.com
    linear att delete ATTACHMENT_ID --force  # Delete attachment"#)]
    Attachments {
        #[command(subcommand)]
        action: attachments::AttachmentCommands,
    },
    /// Authenticate and manage API keys
    #[command(after_help = r#"EXAMPLES:
    linear auth login                        # Store API key
    linear auth status                       # Show auth status
    linear auth logout                       # Remove current profile
    linear auth oauth                        # Authenticate via OAuth 2.0
    linear auth oauth --client-id MY_ID      # Use custom OAuth app
    linear auth revoke                       # Revoke OAuth tokens"#)]
    Auth {
        #[command(subcommand)]
        action: auth::AuthCommands,
    },
    /// Diagnose configuration and connectivity
    #[command(after_help = r#"EXAMPLES:
    linear doctor                            # Check config and auth
    linear doctor --check-api                # Validate API access
    linear doctor --fix                      # Auto-fix common issues"#)]
    Doctor {
        /// Validate API connectivity and auth
        #[arg(long)]
        check_api: bool,
        /// Auto-fix common issues (stale cache, missing config, invalid API key)
        #[arg(long)]
        fix: bool,
    },
    /// Execute raw GraphQL queries and mutations against the Linear API
    #[command(after_help = r#"EXAMPLES:
    linear api query '{ viewer { id name } }'
    linear api query -v teamId=abc '...'     # With variables
    linear api mutate -v title=Bug '...'     # Run mutations"#)]
    Api {
        #[command(subcommand)]
        action: api::ApiCommands,
    },
    /// Manage projects - list, create, update, delete projects
    #[command(alias = "p")]
    #[command(after_help = r#"EXAMPLES:
    linear projects list                    # List all projects
    linear p list --archived                # Include archived projects
    linear p get PROJECT_ID                 # View project details
    linear p create "Q1 Roadmap" -t ENG     # Create a project"#)]
    Projects {
        #[command(subcommand)]
        action: projects::ProjectCommands,
    },
    /// Manage project status updates - list, create, update, archive
    #[command(alias = "pu")]
    #[command(after_help = r#"EXAMPLES:
    linear project-updates list "My Project"   # List updates
    linear pu get UPDATE_ID                    # View update details
    linear pu create "My Project" -b "On track" # Create update
    linear pu archive UPDATE_ID                # Archive update"#)]
    ProjectUpdates {
        #[command(subcommand)]
        action: project_updates::ProjectUpdateCommands,
    },
    /// Manage issues - list, create, update, assign, track issues
    #[command(alias = "i")]
    #[command(after_help = r#"EXAMPLES:
    linear issues list                      # List all issues
    linear i list -t ENG -s "In Progress"   # Filter by team and status
    linear i get LIN-123                    # View issue details
    linear i create "Bug fix" -t ENG -p 2   # Create high priority issue
    linear i update LIN-123 -s Done         # Update issue status"#)]
    Issues {
        #[command(subcommand)]
        action: issues::IssueCommands,
    },
    /// Manage labels - create and organize project/issue labels
    #[command(alias = "l")]
    #[command(after_help = r##"EXAMPLES:
    linear labels list                      # List project labels
    linear l list --type issue              # List issue labels
    linear l create "Feature" --color "#10B981"
    linear l delete LABEL_ID --force"##)]
    Labels {
        #[command(subcommand)]
        action: labels::LabelCommands,
    },
    /// Manage teams - list and view team details
    #[command(alias = "t")]
    #[command(after_help = r#"EXAMPLES:
    linear teams list                       # List all teams
    linear t get ENG                        # View team details"#)]
    Teams {
        #[command(subcommand)]
        action: teams::TeamCommands,
    },
    /// Manage users - list workspace users and view profiles
    #[command(alias = "u")]
    #[command(after_help = r#"EXAMPLES:
    linear users list                       # List all users
    linear u list --team ENG                # List team members
    linear u me                             # View your profile"#)]
    Users {
        #[command(subcommand)]
        action: users::UserCommands,
    },
    /// Manage cycles - view sprint cycles and current cycle
    #[command(alias = "c")]
    #[command(after_help = r#"EXAMPLES:
    linear cycles list -t ENG               # List team cycles
    linear c current -t ENG                 # Show current cycle
    linear c create -t ENG --name "Sprint 5" # Create a cycle
    linear c update ID --name "Sprint 5b"   # Update cycle name"#)]
    Cycles {
        #[command(subcommand)]
        action: cycles::CycleCommands,
    },
    /// Manage comments - add and view issue comments
    #[command(alias = "cm")]
    #[command(after_help = r#"EXAMPLES:
    linear comments list ISSUE_ID           # List comments on issue
    linear cm create ISSUE_ID -b "LGTM!"    # Add a comment"#)]
    Comments {
        #[command(subcommand)]
        action: comments::CommentCommands,
    },
    /// Manage documents - create, update, delete documentation
    #[command(alias = "d")]
    #[command(after_help = r#"EXAMPLES:
    linear documents list                   # List all documents
    linear d get DOC_ID                     # View document
    linear d create "Design Doc" -p PROJ_ID # Create document
    linear d delete DOC_ID --force          # Delete document"#)]
    Documents {
        #[command(subcommand)]
        action: documents::DocumentCommands,
    },
    /// Search across Linear - find issues and projects
    #[command(alias = "s")]
    #[command(after_help = r#"EXAMPLES:
    linear search issues "auth bug"         # Search issues
    linear s projects "backend"             # Search projects"#)]
    Search {
        #[command(subcommand)]
        action: search::SearchCommands,
    },
    /// Sync operations - compare local folders with Linear
    #[command(alias = "sy")]
    #[command(after_help = r#"EXAMPLES:
    linear sync status                      # Compare local vs Linear
    linear sy push -t ENG                   # Create projects for folders
    linear sy push -t ENG --dry-run         # Preview without creating"#)]
    Sync {
        #[command(subcommand)]
        action: sync::SyncCommands,
    },
    /// Manage issue statuses - view workflow states
    #[command(alias = "st")]
    #[command(after_help = r#"EXAMPLES:
    linear statuses list -t ENG             # List team statuses
    linear st get "In Progress" -t ENG      # View status details"#)]
    Statuses {
        #[command(subcommand)]
        action: statuses::StatusCommands,
    },
    /// Git branch operations - checkout branches, create PRs
    #[command(alias = "g")]
    #[command(after_help = r#"EXAMPLES:
    linear git checkout LIN-123             # Checkout issue branch
    linear g branch LIN-123                 # Show branch name
    linear g pr LIN-123                     # Create GitHub PR
    linear g pr LIN-123 --draft             # Create draft PR"#)]
    Git {
        #[command(subcommand)]
        action: git::GitCommands,
    },
    /// Bulk operations - update multiple issues at once
    #[command(alias = "b")]
    #[command(after_help = r#"EXAMPLES:
    linear bulk update-state Done -i LIN-1,LIN-2  # Update multiple issues
    linear b assign me -i LIN-1,LIN-2             # Assign multiple issues
    linear b label bug -i LIN-1,LIN-2             # Add label to issues"#)]
    Bulk {
        #[command(subcommand)]
        action: bulk::BulkCommands,
    },
    /// Manage cache - clear cached data or view status
    #[command(alias = "ca")]
    #[command(after_help = r#"EXAMPLES:
    linear cache status                     # Show cache status
    linear ca clear                         # Clear all cache
    linear ca clear --type teams            # Clear only teams cache"#)]
    Cache {
        #[command(subcommand)]
        action: cache::CacheCommands,
    },
    /// Manage notifications - view and mark as read
    #[command(alias = "n")]
    #[command(after_help = r#"EXAMPLES:
    linear notifications list               # List unread notifications
    linear n count                          # Show unread count
    linear n read-all                       # Mark all as read
    linear n archive NOTIF_ID              # Archive a notification
    linear n archive-all                   # Archive all notifications"#)]
    Notifications {
        #[command(subcommand)]
        action: notifications::NotificationCommands,
    },
    /// Manage issue templates - create and use templates
    #[command(alias = "tpl")]
    #[command(after_help = r#"EXAMPLES:
    linear templates list                   # List all templates
    linear tpl create bug --team ENG --priority 2 --label bug
    linear tpl show bug                     # View template details"#)]
    Templates {
        #[command(subcommand)]
        action: templates::TemplateCommands,
    },
    /// Time tracking - log and view time entries
    #[command(alias = "tm")]
    #[command(after_help = r#"EXAMPLES:
    linear time log LIN-123 2h              # Log 2 hours on issue
    linear tm list --issue LIN-123          # List time entries"#)]
    Time {
        #[command(subcommand)]
        action: time::TimeCommands,
    },
    /// Fetch uploads from Linear with authentication
    #[command(alias = "up")]
    #[command(after_help = r#"EXAMPLES:
    linear uploads fetch URL                # Output to stdout (for piping)
    linear up fetch URL -f file.png         # Save to file
    linear up fetch URL | base64            # Pipe to another tool"#)]
    Uploads {
        #[command(subcommand)]
        action: uploads::UploadCommands,
    },
    /// Interactive mode - TUI for browsing and managing issues
    #[command(alias = "int")]
    #[command(after_help = r#"EXAMPLES:
    linear interactive                      # Launch interactive mode
    linear interactive --team ENG           # Preselect team

Use arrow keys to navigate, Enter to select, q to quit."#)]
    Interactive {
        /// Preselect team by key, name, or ID
        #[arg(short, long)]
        team: Option<String>,
    },
    /// Detect current Linear issue from git branch - for AI agents
    #[command(alias = "ctx")]
    #[command(after_help = r#"EXAMPLES:
    linear context                          # Show current issue from branch
    linear ctx --output json                # Get as JSON for parsing

Detects issue ID from branch names like:
  - lin-123-fix-bug
  - feature/LIN-456-new-feature
  - scw-789-some-task"#)]
    Context,
    /// Manage favorites - quick access to issues/projects
    #[command(alias = "fav")]
    #[command(after_help = r#"EXAMPLES:
    linear favorites list                   # List favorites
    linear fav add LIN-123                  # Add issue to favorites
    linear fav remove LIN-123               # Remove from favorites"#)]
    Favorites {
        #[command(subcommand)]
        action: favorites::FavoriteCommands,
    },
    /// Manage roadmaps - view and manage roadmap planning
    #[command(alias = "rm")]
    #[command(after_help = r#"EXAMPLES:
    linear roadmaps list                    # List all roadmaps
    linear rm get ROADMAP_ID                # View roadmap details
    linear rm create "Q1 Plan"              # Create a roadmap
    linear rm update ID -n "Q2 Plan"        # Update roadmap name"#)]
    Roadmaps {
        #[command(subcommand)]
        action: roadmaps::RoadmapCommands,
    },
    /// Manage initiatives - create, update, and track initiatives
    #[command(alias = "init")]
    #[command(after_help = r#"EXAMPLES:
    linear initiatives list                 # List all initiatives
    linear init get INITIATIVE_ID           # View initiative details
    linear init create "H1 Goals"           # Create an initiative
    linear init update ID -s "Active"       # Update initiative status"#)]
    Initiatives {
        #[command(subcommand)]
        action: initiatives::InitiativeCommands,
    },
    /// Triage inbox - manage unassigned issues
    #[command(alias = "tr")]
    #[command(after_help = r#"EXAMPLES:
    linear triage list                      # List triage issues
    linear tr claim LIN-123                 # Claim an issue
    linear tr snooze LIN-123 --duration 1w  # Snooze for a week"#)]
    Triage {
        #[command(subcommand)]
        action: triage::TriageCommands,
    },
    /// View metrics - velocity, burndown, progress
    #[command(alias = "mt")]
    #[command(after_help = r#"EXAMPLES:
    linear metrics cycle CYCLE_ID           # Cycle metrics
    linear mt project PROJECT_ID            # Project progress
    linear mt velocity TEAM --cycles 5      # Team velocity"#)]
    Metrics {
        #[command(subcommand)]
        action: metrics::MetricsCommands,
    },
    /// Manage project milestones - list, create, update, delete milestones
    #[command(alias = "ms")]
    #[command(after_help = r#"EXAMPLES:
    linear milestones list -p "My Project"  # List milestones
    linear ms get MILESTONE_ID              # View milestone details
    linear ms create "Beta Release" -p PROJ # Create milestone
    linear ms update ID --target-date +2w   # Update target date
    linear ms delete ID --force             # Delete milestone"#)]
    Milestones {
        #[command(subcommand)]
        action: milestones::MilestoneCommands,
    },
    /// Export issues to CSV, JSON, or Markdown
    #[command(alias = "exp")]
    #[command(after_help = r#"EXAMPLES:
    linear export csv --team ENG            # Export team issues to CSV
    linear exp csv -f issues.csv            # Export to file
    linear exp json --team ENG --pretty     # Export as pretty JSON
    linear exp markdown --team ENG          # Export as Markdown
    linear exp projects-csv -f projects.csv # Export projects to CSV"#)]
    Export {
        #[command(subcommand)]
        action: export::ExportCommands,
    },
    /// Import issues from CSV or JSON files
    #[command(alias = "im")]
    #[command(after_help = r#"EXAMPLES:
    linear import csv issues.csv -t ENG           # Import from CSV
    linear im csv issues.csv -t ENG --dry-run     # Preview without creating
    linear im json issues.json -t ENG             # Import from JSON"#)]
    Import {
        #[command(subcommand)]
        action: import::ImportCommands,
    },
    /// View issue history and activity
    #[command(alias = "hist")]
    #[command(after_help = r#"EXAMPLES:
    linear history issue LIN-123            # View issue activity
    linear hist issue LIN-123 --limit 50    # More entries"#)]
    History {
        #[command(subcommand)]
        action: history::HistoryCommands,
    },
    /// Manage custom views - create, apply, and manage saved views
    #[command(alias = "v")]
    #[command(after_help = r#"EXAMPLES:
    linear views list                       # List all custom views
    linear v list --shared                  # List shared views only
    linear v get "My View"                  # View details
    linear v create "Bug Triage" --shared   # Create a shared view
    linear v delete VIEW_ID --force         # Delete a view"#)]
    Views {
        #[command(subcommand)]
        action: views::ViewCommands,
    },
    /// Manage webhooks - create, update, delete, listen for events
    #[command(alias = "wh")]
    #[command(after_help = r#"EXAMPLES:
    linear webhooks list                    # List all webhooks
    linear wh create URL --events Issue     # Create webhook
    linear wh delete WEBHOOK_ID --force     # Delete webhook
    linear wh rotate-secret WEBHOOK_ID      # Rotate webhook secret
    linear wh listen --port 9000            # Listen for events locally"#)]
    Webhooks {
        #[command(subcommand)]
        action: webhooks::WebhookCommands,
    },
    /// Watch for updates (polling)
    #[command(after_help = r#"EXAMPLES:
    linear watch issue LIN-123             # Watch single issue
    linear watch issue LIN-123 --interval 30  # Poll every 30 seconds
    linear watch project PROJECT_ID        # Watch a project
    linear watch team ENG                  # Watch a team"#)]
    Watch {
        #[command(subcommand)]
        action: WatchCommands,
    },
    /// Manage issue relationships - parent/child, blocking, related
    #[command(alias = "rel")]
    #[command(after_help = r#"EXAMPLES:
    linear relations list LIN-123           # List issue relationships
    linear rel add LIN-1 -r blocks LIN-2    # LIN-1 blocks LIN-2
    linear rel parent LIN-2 LIN-1           # Set LIN-1 as parent of LIN-2
    linear rel unparent LIN-2               # Remove parent"#)]
    Relations {
        #[command(subcommand)]
        action: relations::RelationCommands,
    },
    /// Show current authenticated user (alias for `users me`)
    #[command(alias = "me")]
    Whoami,
    /// Mark the current branch's issue as Done
    #[command(after_help = r#"EXAMPLES:
    linear done                              # Mark current branch issue as Done
    linear done --status "In Progress"       # Set to specific status instead

Reads the current git branch, extracts the issue ID (e.g. feat/SCW-123-title → SCW-123),
and updates the issue status."#)]
    Done {
        /// Status to set (default: "Done")
        #[arg(short, long, default_value = "Done")]
        status: String,
    },
    /// Guided onboarding wizard - configure auth, team, and output format
    #[command(after_help = r#"EXAMPLES:
    linear setup                             # Run interactive setup wizard

Walks you through:
  1. Setting your Linear API key
  2. Choosing a default team
  3. Selecting output format (table or json)"#)]
    Setup,
    /// Sprint planning - manage cycle-based sprints
    #[command(alias = "sp")]
    #[command(after_help = r#"EXAMPLES:
    linear sprint status -t ENG            # Current sprint status
    linear sp progress -t ENG              # Sprint progress bar
    linear sp plan -t ENG                  # Next sprint's planned issues
    linear sp carry-over -t ENG --force    # Move incomplete issues to next cycle"#)]
    Sprint {
        #[command(subcommand)]
        action: sprint::SprintCommands,
    },
    /// Generate shell completions
    #[command(alias = "comp")]
    #[command(after_help = r#"EXAMPLES:
    linear completions bash > ~/.bash_completion.d/linear
    linear completions zsh > ~/.zfunc/_linear
    linear completions fish > ~/.config/fish/completions/linear.fish
    linear comp powershell > linear.ps1
    linear comp dynamic bash   # Dynamic completions with argument value hints
    linear comp dynamic zsh    # Dynamic completions for zsh"#)]
    Completions {
        #[command(subcommand)]
        action: CompletionCommands,
    },
    /// Internal: provide dynamic completion values (hidden from help)
    #[command(name = "_complete", hide = true)]
    Complete {
        /// What to complete: teams, projects, issues, statuses, users, labels
        #[arg(long = "type")]
        type_: String,
        /// Partial input to filter
        #[arg(long, default_value = "")]
        prefix: String,
        /// Team context for scoped completions (e.g. statuses)
        #[arg(long)]
        team: Option<String>,
    },
    /// Configure CLI settings - API keys and workspaces
    #[command(after_help = r#"EXAMPLES:
    printf '%s\n' "$LINEAR_API_KEY" | linear config set-key
    linear config get api-key               # Get API key (masked)
    linear config set profile work          # Switch profile
    linear config show                      # Show configuration
    printf '%s\n' "$LINEAR_API_KEY" | linear config workspace-add work
    linear config workspace-switch work     # Switch workspace"#)]
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set API key
    #[command(after_help = r#"EXAMPLE:
    printf '%s\n' "$LINEAR_API_KEY" | linear config set-key"#)]
    SetKey,
    /// Get a configuration value
    Get {
        /// Config key to retrieve (api-key, profile)
        key: ConfigGetKey,
        /// Output raw value without masking
        #[arg(long)]
        raw: bool,
    },
    /// Set a configuration value
    Set {
        /// Config key to set
        key: ConfigSetKey,
        /// Value to set
        value: String,
    },
    /// Show current configuration
    Show,
    /// Generate shell completions
    #[command(after_help = r#"EXAMPLES:
    linear config completions bash > ~/.bash_completion.d/linear
    linear config completions zsh > ~/.zfunc/_linear
    linear config completions fish > ~/.config/fish/completions/linear.fish
    linear config completions powershell > linear.ps1"#)]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Add a new workspace
    #[command(alias = "add")]
    #[command(after_help = r#"EXAMPLE:
    printf '%s\n' "$LINEAR_API_KEY" | linear config workspace-add personal"#)]
    WorkspaceAdd {
        /// Workspace name
        name: String,
    },
    /// List all workspaces
    #[command(alias = "list")]
    WorkspaceList,
    /// Switch to a different workspace
    #[command(alias = "use")]
    #[command(after_help = r#"EXAMPLE:
    linear config workspace-switch personal"#)]
    WorkspaceSwitch {
        /// Workspace name to switch to
        name: String,
    },
    /// Show current workspace
    #[command(alias = "current")]
    WorkspaceCurrent,
    /// Remove a workspace
    #[command(alias = "rm")]
    WorkspaceRemove {
        /// Workspace name to remove
        name: String,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ConfigGetKey {
    #[value(alias = "api_key")]
    ApiKey,
    Profile,
}

impl std::fmt::Display for ConfigGetKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKey => write!(f, "api-key"),
            Self::Profile => write!(f, "profile"),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ConfigSetKey {
    Profile,
}

impl std::fmt::Display for ConfigSetKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Profile => write!(f, "profile"),
        }
    }
}

#[derive(Subcommand)]
pub enum WatchCommands {
    /// Watch an issue for updates
    Issue {
        /// Issue identifier to watch
        id: String,
        /// Polling interval in seconds
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
    /// Watch a project for updates
    Project {
        /// Project ID to watch
        id: String,
        /// Polling interval in seconds
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
    /// Watch a team for updates
    Team {
        /// Team key or ID to watch
        team: String,
        /// Polling interval in seconds
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
}

#[derive(Subcommand)]
pub enum CompletionCommands {
    /// Generate static shell completions (command names and flags)
    Static {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Generate dynamic shell completions (argument values from Linear API)
    Dynamic {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}
