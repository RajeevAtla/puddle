# Prompt: Replicate My OpenCode + Herdr Setup on a Restricted Linux Cluster

You are configuring my Linux cluster account to reproduce, as closely as Linux permits, the OpenCode + Herdr setup from my Windows workstation. Perform the work end-to-end. Do not merely give me instructions unless a step requires my credentials, interactive authentication, or an administrator.

The source setup was inventoried on 2026-08-13. Treat the versions and configuration below as the desired state. The target is Linux, probably a shared cluster, and I may not have root or broad permissions.

## Non-negotiable operating constraints

1. Assume no `sudo`, no package-manager administration, no systemd user service, and no ability to edit `/etc`, `/usr`, `/opt`, shared modulefiles, or shared software.
2. Install only under my home directory unless I explicitly approve another writable location. Prefer XDG paths and `~/.local/bin`.
3. Do not weaken cluster security, bypass policy, change firewall/proxy settings, expose a listening service publicly, or modify shared resources.
4. Never print, persist in shell history, commit, or copy secrets. In particular, do not copy any source machine `auth.json`, `mcp-auth.json`, SQLite database, session history, Herdr socket, Herdr session file, logs, caches, or token values.
5. Preserve existing files. Before changing a config file, inspect it, merge rather than overwrite, and create a timestamped backup in the same directory with mode `0600` when it may contain sensitive settings.
6. Use `umask 077` while creating private config/auth files. Set directories to `0700` and secret-bearing files to `0600` where practical.
7. Detect the actual shell, CPU architecture, libc, home path, XDG paths, available module system, internet/proxy access, and writable filesystem before installing.
8. If outbound access, process spawning, Unix sockets, Node, browser downloads, or home-directory execution is blocked, do not fake success. Skip only the blocked component, record the exact command/error, and continue with independent components.
9. Do not run a long-lived interactive Herdr TUI from a non-interactive batch job or login automation. Use an interactive allocation/login session appropriate to the cluster policy.
10. Do not copy machine-specific paths from Windows. Translate all paths to Linux/XDG equivalents.
11. Do not silently substitute a model. The desired model is `openai/gpt-5.6-luna`. If this OpenCode build/account does not expose it, report the blocker and ask before changing the model field.
12. Do not use `opencode --auto` or globally allow every permission.

## Success criteria

The setup is complete when all feasible checks pass:

- `opencode --version` reports `1.18.18`, or you clearly report why exact pinning was impossible.
- `herdr --version` reports `0.8.0`.
- `herdr config check` reports `config: ok`.
- `herdr integration status` reports `opencode: current (v9)`.
- `herdr plugin list --json` reports enabled `herdr-file-viewer` version `1.15.0`.
- OpenCode validates and starts with model, LSPs, plugins, MCPs, and skills below.
- `opencode mcp list` reports GitHub and Playwright connected when credentials/network policy permit.
- OpenCode lists all 13 requested installed skills plus its built-in `customize-opencode` skill.
- When OpenCode is launched inside Herdr, Herdr detects `agent: opencode` and its state transitions among `working`, `blocked`, and `idle`.
- In that Herdr-hosted OpenCode session, `/herdr-status` succeeds and `/herdr-test` passes if the cluster permits nested panes/processes.
- `/quota_status` works, and telemetry begins writing local-only data after a test session.
- No secrets appear in the final report.

## Desired source state

- Source OS: Windows; target OS: Linux.
- OpenCode: `1.18.18` (`opencode-ai@1.18.18`).
- Herdr: `0.8.0`, preview channel, protocol 19.
- Node source machine: `26.5.0`; npm package requirements only require Node 22+ for quota. Use an available user-owned Node 22+; exact Node replication is secondary.
- Bun source machine: `1.4.0`. Bun is useful because `opencode-herdr` requires Bun >=1.1 and telemetry supports Bun, but do not block the whole setup if Bun cannot be installed.
- Default OpenCode model: `openai/gpt-5.6-luna`.
- OpenCode auth on source: OpenAI OAuth. Re-authenticate on Linux; do not copy credential files.
- OpenCode plugins:
  - `opencode-herdr@0.1.2`
  - `@slkiser/opencode-quota@4.5.0`
  - `opencode-telemetry@0.1.19`
- OpenCode MCPs:
  - GitHub Copilot remote MCP at `https://api.githubcopilot.com/mcp/`, OAuth auto-detection disabled, bearer token from `GITHUB_MCP_TOKEN`.
  - Playwright MCP via `npx -y @playwright/mcp@latest`; source resolved latest was `0.0.79` on inventory date.
- Herdr native OpenCode integration: installed and current, integration version 9, managed file `~/.config/opencode/plugins/herdr-agent-state.js`.
- Herdr plugin: `smarzban/herdr-file-viewer`, version `1.15.0`, enabled. Installed source on the workstation had resolved commit `4a9aa2f339872a2a37f3d8882fbb91fca6b6a455`, which is a post-release documentation-only commit while the manifest remains 1.15.0.
- Herdr theme: terminal ANSI theme, no auto-switch.
- Herdr sidebar agent sorting: priority.
- Herdr file viewer keys on Linux:
  - `prefix+f` invokes `herdr-file-viewer.open-file-viewer`.
  - `prefix+shift+f` invokes `herdr-file-viewer.open-file-viewer-tab`.
- Herdr optional file renderers: `glow`, `delta`, and `bat`. These were absent on Windows but are recommended on Linux. Install only in user space and only if straightforward.
- OpenCode TUI theme: `vercel`.
- OpenCode's built-in `customize-opencode` skill is supplied by OpenCode itself. Verify it is available; do not create a duplicate skill file.
- No custom OpenCode agents or standalone global command files were present.
- The source project had its own application-specific `AGENTS.md`; that is project context, not global OpenCode/Herdr configuration. Do not install it globally or invent a replacement.
- The telemetry plugin generates project-local `.opencode/commands/telemetry-report.md` and `.opencode/commands/telemetry-inspect.md` on use/startup; do not hardcode the Windows paths from those generated files.

## Phase 1: preflight and plan

Inspect and report, without changing anything yet:

```bash
set -u
uname -a
uname -m
getconf GNU_LIBC_VERSION 2>/dev/null || true
printf 'HOME=%s\nXDG_CONFIG_HOME=%s\nXDG_DATA_HOME=%s\nXDG_CACHE_HOME=%s\nSHELL=%s\n' \
  "$HOME" "${XDG_CONFIG_HOME:-}" "${XDG_DATA_HOME:-}" "${XDG_CACHE_HOME:-}" "${SHELL:-}"
command -v module >/dev/null 2>&1 && module --version || true
for cmd in curl wget git node npm npx bun opencode herdr rustup cargo rust-analyzer ruff biome glow delta bat; do
  command -v "$cmd" >/dev/null 2>&1 && printf '%s=%s\n' "$cmd" "$(command -v "$cmd")" || printf '%s=MISSING\n' "$cmd"
done
test -w "$HOME" && echo 'HOME writable' || echo 'HOME NOT writable'
df -h "$HOME" 2>/dev/null || true
ulimit -a
```

Also inspect existing files/directories if present:

- `${XDG_CONFIG_HOME:-$HOME/.config}/opencode/`
- `${XDG_CONFIG_HOME:-$HOME/.config}/herdr/`
- `$HOME/.agents/skills/`
- Existing shell startup file appropriate to `$SHELL`.

Define and use:

```bash
export XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
export XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
export XDG_CACHE_HOME="${XDG_CACHE_HOME:-$HOME/.cache}"
export PATH="$HOME/.local/bin:$HOME/.npm-global/bin:$HOME/.bun/bin:$PATH"
umask 077
```

Do not append duplicate PATH lines. If the cluster uses environment modules, prefer an existing modern Node module over installing another Node. If Node is below 22, use the least invasive user-space option available: an existing newer module, then `mise`/`nvm`/Volta if already present, then a user-local official Node binary. Do not use `sudo npm -g`.

## Phase 2: install OpenCode and supporting CLIs in user space

Install exact OpenCode when feasible:

```bash
mkdir -p "$HOME/.local/bin" "$HOME/.npm-global"
npm config set prefix "$HOME/.npm-global"
npm install -g opencode-ai@1.18.18
opencode --version
```

If npm global installation is forbidden but direct downloads are allowed, use OpenCode's official user-local installer or a release binary under `~/.local/bin`. Do not install system-wide. Confirm the resulting version.

Install Bun user-locally if absent and policy permits. Keep it in `~/.bun`; add `~/.bun/bin` to PATH. Verify Bun >=1.1. The source version was 1.4.0, so use that exact version if the installer supports pinning without complexity; otherwise use a compatible current user-local Bun and disclose the version difference.

Install supporting tools only where relevant:

```bash
npm install -g @biomejs/biome@2.5.6 @playwright/cli@0.1.17
```

For Python LSP support, install `ruff==0.14.7` with an already-available user-space tool such as `uv tool install ruff==0.14.7` or `pipx install ruff==0.14.7`. Do not alter shared Python. If neither is available, omit Ruff and report it.

For Rust LSP support, use an existing `rust-analyzer` or install it via a user-owned rustup toolchain. Do not install a full Rust toolchain solely for the LSP if quotas or policy make that unreasonable; leave the config in place only when the executable exists.

The following OpenCode built-in LSPs must be disabled exactly: `pyright`, `typescript`, `eslint`, `oxlint`, and `deno`. Configure Ruff, Rust Analyzer, and Biome as shown later only if their executables are available. The source config did not disable unknown additional built-ins, so do not add speculative disables.

Playwright browser binaries may be large and cluster compute nodes may be headless or disallow downloads. Install `@playwright/cli` and the MCP package resolution first. Download Chromium only if a real browser task requires it and policy/storage permit. Never use `playwright install-deps` without root permission.

## Phase 3: install exact OpenCode plugins

Use OpenCode's global plugin installer where possible because it creates portable cache paths and updates config correctly:

```bash
opencode plugin opencode-herdr@0.1.2 -g
opencode plugin @slkiser/opencode-quota@4.5.0 -g
opencode plugin opencode-telemetry@0.1.19 -g
```

If OpenCode's plugin command rejects an explicit version, install the package into a user-owned config/package directory and reference its actual resolved module/path. Do not copy any source machine path under `C:/Users/<windows-user>/.cache/...`.

Important behavior to preserve:

- `opencode-herdr` adds provider targets such as `herdr/<adapter>/<nativeModel>` and slash commands `/herdr-status`, `/herdr-test`, `/herdr-delete`, `/herdr-pane`, and `/herdr-handover`.
- It idempotently installs `~/.config/opencode/skills/herdr/SKILL.md` when OpenCode loads.
- Use default plugin options: no `debug`, no `keepPanes`, no `keepJobs`, and no `handoverDefault` override. Do not set `OPENCODE_HERDR_DEBUG`, `OPENCODE_HERDR_KEEP_PANES`, or `OPENCODE_HERDR_KEEP_JOBS`.
- `@slkiser/opencode-quota` is both a server/OpenCode plugin and TUI plugin.
- `opencode-telemetry` is local-first and writes its metrics database under `~/.local/share/opencode-telemetry/data.db`. It must not send telemetry externally. Do not run `octm inspect --content` unless I explicitly ask, because that populates a prompt-content cache.

Expected package integrity values for supply-chain verification, if npm exposes them:

- `opencode-herdr@0.1.2`: `sha512-pn8mEjLz5FTAHvJt1H4FC7AwdmK0kXGZjPAy6j0W6ifl5GCKsPpcvzgWGUi4xDRmjt0skrUkxSiefU+h2e7/VQ==`
- `@slkiser/opencode-quota@4.5.0`: `sha512-ClJhsMstJS+10hi+s5jqdyrAhCczsI/Dtx5/2sv4Aut/SbmnKdaAaKeh110/q3epqfO3lULNelMT32X/jnjfpA==`
- `opencode-telemetry@0.1.19`: `sha512-JYdzFK1vHV7ID1lmn/+fAudFSPraF1nQkn37WxqpNAjuDV8tapH4McmnwuSCRemEuukxqsPptSvc7UNoXQQiOA==`
- `opencode-ai@1.18.18`: `sha512-J+5HFq8tf+wPBBpBpMPSNjSytF2/EkNWYfFZh4si1d9auFbQriqDyqZv+vFUsLWERfdMU32Eajwuiq3rKBvZLQ==`

Do not install `@dietrichgebert/ponytail` as an OpenCode plugin: this source setup uses Ponytail skill files plus always-on `instructions`, not the Ponytail runtime plugin.

## Phase 4: install the skills

OpenCode automatically scans both:

- `$XDG_CONFIG_HOME/opencode/skills/<skill>/SKILL.md`
- `$HOME/.agents/skills/<skill>/SKILL.md`

Install exactly these skills.

### OpenCode-global skills under `~/.config/opencode/skills`

- `ponytail`
- `ponytail-review`
- `ponytail-audit`
- `ponytail-debt`
- `ponytail-gain`
- `ponytail-help`
- `frontend-design`
- `karpathy-guidelines`
- `grill-me`
- `grilling`
- `herdr` (installed by `opencode-herdr`; verify it, do not maintain a second divergent copy)

### External shared skills under `~/.agents/skills`

- `interface-design`
- `playwright-cli`, including its `references/` directory

Use pinned upstream sources and sparse checkout/copy only the listed skill directories. Do not install every skill from these repositories.

#### Ponytail bundle, exact installed version

All six installed Ponytail skill files match tag `v4.9.0` from `https://github.com/DietrichGebert/ponytail.git` exactly. The tag resolves to commit `16f29800fd2681bdf24f3eb4ccffe38be3baec6b` (annotated tag object `0a4dd63ad4541f4f655c4108a295916f3c1d8fda`). Copy only `skills/ponytail*` to `$XDG_CONFIG_HOME/opencode/skills/`.

#### Grill skills, exact installed version

Use `https://github.com/mattpocock/skills.git` at commit `2ab958093e83e0ec752e6c1c5932da465bf23e0c` and copy:

- `skills/productivity/grill-me`
- `skills/productivity/grilling`

Preserve their `agents/openai.yaml` metadata if present. The installed `grilling` version asks questions one at a time, not the newer frontier-in-rounds behavior.

#### Frontend design skill

Use `https://github.com/anthropics/claude-plugins-official.git` at commit `d029127f7d29bdb8fd8902ac34dd7d5c8ba92b6e`, path `plugins/frontend-design/skills/frontend-design/`. The installed prose differs from that commit only in tiny punctuation/wording changes; copy the pinned skill and its `LICENSE.txt`, then prefer the source machine's exact file if it is available as an input artifact. Do not install the whole Claude plugin marketplace.

#### Karpathy guidelines

Use the content from `https://github.com/Chainlit/chainlit/blob/ec4eeaa4107f9ecb2665e339b99fd2b6079a3510/.shared/skills/karpathy-guidelines/SKILL.md`, but remove only the optional `license: MIT` frontmatter line to match the installed file. Blank-line formatting differences are immaterial. Do not copy unrelated Chainlit files.

#### Interface design

Use `https://github.com/Dammyjay93/interface-design.git` at commit `890d9295acdaf57891a20dc2499abfb4b8529784`, path `.claude/skills/interface-design/`, copied to `$HOME/.agents/skills/interface-design/`. This exact commit matches the installed `SKILL.md` and `agents/openai.yaml`.

#### Playwright CLI skill

Use `https://github.com/microsoft/playwright.git` at commit `6846dc7d2e23a47ad87b254df0f4ac4338ef1981`, path `packages/playwright-core/src/tools/skills/playwright-cli/`, copied to `$HOME/.agents/skills/playwright-cli/`. This exact commit matches the installed `SKILL.md` and all nine installed reference files:

- `element-attributes.md`
- `playwright-tests.md`
- `request-mocking.md`
- `running-code.md`
- `session-management.md`
- `storage-state.md`
- `test-generation.md`
- `tracing.md`
- `video-recording.md`

The installed Playwright CLI package itself is `@playwright/cli@0.1.17`.

After copying, verify every skill directory contains `SKILL.md`, frontmatter names match folder names, files are readable, and no repository `.git` directories were copied into the skill locations.

## Phase 5: write/merge OpenCode configuration

Use Linux path `$XDG_CONFIG_HOME/opencode/opencode.jsonc`. Validate against `https://opencode.ai/config.json`. Merge this desired state with any existing unrelated settings instead of overwriting them. The source config package metadata contained `@opencode-ai/plugin@1.18.11`; if OpenCode creates `$XDG_CONFIG_HOME/opencode/package.json`, pin that dependency to `1.18.11` unless the plugin installer proves it requires a newer compatible patch, in which case preserve the working generated version and report the deviation.

The plugin installer may replace plugin entries with resolved `file://` or absolute Linux paths. Preserve those valid generated entries. The semantic desired plugin order is:

1. `opencode-herdr@0.1.2`
2. `@slkiser/opencode-quota@4.5.0`
3. `opencode-telemetry@0.1.19`

Use this configuration shape, replacing `<XDG_CONFIG_HOME>` with the actual absolute Linux XDG config directory only in `instructions`; do not leave environment expressions or tildes there if OpenCode does not expand them:

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "lsp": {
    "pyright": { "disabled": true },
    "ruff": {
      "command": ["ruff", "server"],
      "extensions": [".py", ".pyi"]
    },
    "rust": {
      "command": ["rust-analyzer"]
    },
    "typescript": { "disabled": true },
    "eslint": { "disabled": true },
    "oxlint": { "disabled": true },
    "deno": { "disabled": true },
    "biome": {
      "command": ["biome", "lsp-proxy"],
      "extensions": [".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts"]
    }
  },
  "model": "openai/gpt-5.6-luna",
  "instructions": [
    "<XDG_CONFIG_HOME>/opencode/skills/herdr/SKILL.md",
    "<XDG_CONFIG_HOME>/opencode/skills/ponytail/SKILL.md",
    "<XDG_CONFIG_HOME>/opencode/skills/karpathy-guidelines/SKILL.md",
    "<XDG_CONFIG_HOME>/opencode/skills/grill-me/SKILL.md"
  ],
  "plugin": [
    "opencode-herdr@0.1.2",
    "@slkiser/opencode-quota@4.5.0",
    "opencode-telemetry@0.1.19"
  ],
  "mcp": {
    "github": {
      "type": "remote",
      "url": "https://api.githubcopilot.com/mcp/",
      "enabled": true,
      "oauth": false,
      "headers": {
        "Authorization": "Bearer {env:GITHUB_MCP_TOKEN}"
      }
    },
    "playwright": {
      "type": "local",
      "command": ["npx", "-y", "@playwright/mcp@latest"],
      "enabled": true
    }
  },
  "provider": {
    "openai": {
      "whitelist": ["gpt-5.6-sol", "gpt-5.6-terra", "gpt-5.6-luna"]
    },
    "opencode": {
      "whitelist": [
        "big-pickle",
        "deepseek-v4-flash-free",
        "mimo-v2.5-free",
        "laguna-s-2.1-free",
        "ling-3.0-flash-free",
        "north-mini-code-free",
        "nemotron-3-ultra-free"
      ]
    }
  }
}
```

Notes:

- Keep unavailable whitelist names because they mirror the source preference; report which are not currently returned by `opencode models`.
- On the source inventory, `ling-3.0-flash-free` and `north-mini-code-free` were configured but not returned by the current model listing. That is not a config error.
- If `ruff`, `rust-analyzer`, or `biome` is unavailable, do not leave a command that causes noisy startup failure. Either finish that user-local install or omit only that LSP block and report the deviation.
- Do not add broad permissions. Use OpenCode defaults unless existing cluster policy requires narrower rules.

Write `$XDG_CONFIG_HOME/opencode/tui.json` by merging this state:

```json
{
  "$schema": "https://opencode.ai/tui.json",
  "theme": "vercel",
  "plugin": ["@slkiser/opencode-quota@4.5.0"]
}
```

If the TUI plugin loader expects the unversioned package name after installation, preserve the generated value `@slkiser/opencode-quota`; exact package version must still be 4.5.0 in cache/package metadata.

Write `$XDG_CONFIG_HOME/opencode/opencode-quota/quota-toast.jsonc` exactly:

```jsonc
{
  "enabledProviders": "auto",
  "tuiCommandDisplay": "inline",
  "tuiSidebarPanel": {
    "enabled": false
  },
  "enableToast": false,
  "tuiCompactStatus": {
    "enabled": true,
    "homeBottom": true,
    "sessionPrompt": true
  }
}
```

No explicit `~/.config/opencode-telemetry/config.json` existed on the source. Use plugin defaults. Explain local data locations and privacy in the final report.

## Phase 6: credentials and MCPs

Do not migrate credentials from the Windows machine.

### OpenAI/OpenCode authentication

Launch OpenCode and use its `/connect` flow for OpenAI OAuth, or the current documented equivalent. This may require me to open a URL in a browser on another computer and paste/confirm a code. Stop and ask me only for that interactive step. Never show the resulting token.

### GitHub MCP token

The config expects `GITHUB_MCP_TOKEN`. Ask me to provide/configure it through an approved secret mechanism, not in chat if avoidable. Preferred options, in order:

1. Cluster secret manager/environment injection already used by my account.
2. A private shell startup fragment outside repositories, mode `0600`, sourced by my interactive shell.
3. Manual export in the terminal for the current session.

Do not write the token directly into `opencode.jsonc`. Verify only that it is set and nonempty, never print it. The source token length was observed but is irrelevant; do not validate by exact length.

If compute nodes cannot reach `api.githubcopilot.com`, leave the MCP configured but disabled only if repeated startup failures are disruptive; report the network restriction and the exact hostname that needs approval.

### Playwright MCP

Verify package resolution with a harmless metadata/version invocation. The MCP can start headlessly, but actual browser use may require browser assets and libraries unavailable on the cluster. Distinguish “MCP process connects” from “Chromium launches successfully.”

## Phase 7: install exact Herdr preview in user space

The source uses preview build tag `preview-2026-08-04-d78e3d3b5126`, commit `d78e3d3b51266a1ff80ae6858b894e782aac3e9f`.

Prefer exact release assets rather than `latest`:

- Linux x86_64 URL: `https://github.com/herdrdev/herdr/releases/download/preview-2026-08-04-d78e3d3b5126/herdr-linux-x86_64`
- Linux x86_64 SHA-256: `338afbfe03f0eb32efc52e30a5effc6c5ee772846bdb5e059f73d04b89b467d6`
- Linux aarch64 URL: `https://github.com/herdrdev/herdr/releases/download/preview-2026-08-04-d78e3d3b5126/herdr-linux-aarch64`
- Linux aarch64 SHA-256: `f79f3ad5b24dd1b6ba49ebee1b26d69ab5dcbeaf15be4fd7f3357ab0d17f6096`

Detect `uname -m`, download the matching binary to a temporary file under my home or `${TMPDIR:-/tmp}` if permitted, verify SHA-256 before execution, then install it as `~/.local/bin/herdr` with mode `0755`. If architecture is neither x86_64 nor aarch64, stop this component and report unsupported architecture.

Run:

```bash
herdr --version
herdr channel set preview
herdr channel show
```

`herdr channel set preview` can install the latest preview. Re-check `herdr --version` immediately afterward. If it changed from the pinned build, reinstall the verified pinned binary to `~/.local/bin/herdr`; the channel state should remain preview. Exact binary takes priority over tracking a newer preview.

## Phase 8: configure Herdr for Linux

Use `$XDG_CONFIG_HOME/herdr/config.toml`, not Windows `%APPDATA%`. Merge this exact Linux-adapted state:

```toml
onboarding = false

[terminal]
default_shell = "/bin/bash"

[ui]
agent_panel_sort = "priority"

[[keys.command]]
key = "prefix+f"
type = "plugin_action"
command = "herdr-file-viewer.open-file-viewer"
description = "open file viewer in split"

[[keys.command]]
key = "prefix+shift+f"
type = "plugin_action"
command = "herdr-file-viewer.open-file-viewer-tab"
description = "open file viewer in tab"

[theme]
name = "terminal"
auto_switch = false
```

Use the actual executable path from `$SHELL` instead of `/bin/bash` only if it is a normal user shell suitable for interactive panes and exists on compute/login nodes. Do not set PowerShell. Do not use Windows action IDs ending in `-windows`.

Validate:

```bash
herdr config check
```

If a server is already running, reload rather than kill it:

```bash
herdr server reload-config
```

Do not run `herdr server stop` unless a protocol mismatch requires it and I approve losing pane processes.

## Phase 9: install Herdr file viewer

Install the Herdr plugin in user space:

```bash
herdr plugin install smarzban/herdr-file-viewer --ref 4a9aa2f339872a2a37f3d8882fbb91fca6b6a455 --yes
```

If the installed Herdr CLI does not accept `--ref` with a commit, use tag `v1.15.0`. Verify manifest version 1.15.0 and enabled state. The plugin can download a prebuilt x86_64 Linux musl binary; if it falls back to Cargo and Rust is unavailable, manually use the signed/checksummed release asset or report the blocker.

Reference release data:

- Repository: `https://github.com/smarzban/herdr-file-viewer`
- Release: `v1.15.0`
- Release commit: `a2368d701659813938f79e2f1e5aa4e9f4fb2b77`
- x86_64 Linux musl binary SHA-256: `11bd09834d660856275f8fd8ae5156f6ffc720d2b47712e79f05b5574b2ef00d`
- Minimum Herdr: 0.7.0

Verify:

```bash
herdr plugin list --json
herdr plugin action list
```

Expected Unix action IDs are `open-file-viewer` and `open-file-viewer-tab`.

Install optional renderers `glow`, `git-delta` (binary `delta`), and `bat` only through existing modules, Homebrew/Linuxbrew in user space, cargo-binstall, or direct verified release binaries. Do not require them; the viewer degrades gracefully. On Debian-like systems the binary may be named `batcat`; only create a private `~/.local/bin/bat` symlink if `batcat` exists and this does not conflict with an existing command.

## Phase 10: install the native Herdr OpenCode integration

This is required even though `opencode-herdr` is also installed. They do different jobs:

- `opencode-herdr` lets OpenCode create/control Herdr panes and exposes Herdr provider/slash commands.
- `herdr integration install opencode` installs the Herdr-managed OpenCode state reporter so Herdr knows when OpenCode is working, blocked, or idle and can associate native session IDs.

Run:

```bash
herdr integration install opencode
herdr integration status
```

Expected result: `opencode: current (v9)` and managed file:

```text
$XDG_CONFIG_HOME/opencode/plugins/herdr-agent-state.js
```

Do not edit that managed file. It should contain `HERDR_INTEGRATION_ID=opencode` and `HERDR_INTEGRATION_VERSION=9`. Herdr may overwrite it on reinstall/update; custom plugins must live beside it.

OpenCode auto-discovers JavaScript files under `~/.config/opencode/plugins/`, so no manual `plugin` array entry is needed for this native state reporter.

After installing any OpenCode config/plugin/skill file, fully quit and restart OpenCode; its configuration is loaded only at startup.

## Phase 11: verify OpenCode outside and inside Herdr

### Static checks

Run and capture redacted output:

```bash
opencode --version
opencode auth list
opencode mcp list
opencode models openai
opencode models opencode
herdr --version
herdr channel show
herdr config check
herdr integration status
herdr plugin list --json
```

`opencode auth list` must show provider/type only; redact any credential material if the CLI unexpectedly emits it.

Start OpenCode once outside Herdr to catch config/schema/plugin errors. Then quit.

### Herdr-hosted check

From an interactive cluster shell/allocation, start Herdr normally. Do not run bare Herdr for discovery from an automation subprocess; this step is intentionally interactive.

Create or use a workspace in a disposable test directory under my home, launch `opencode`, and verify these environment variables exist inside the pane without exposing sensitive values:

- `HERDR_ENV=1`
- `HERDR_WORKSPACE_ID`
- `HERDR_TAB_ID`
- `HERDR_PANE_ID`
- `HERDR_SOCKET_PATH`

From a separate shell or via Herdr's CLI, inspect the pane by explicit/current ID:

```bash
herdr pane current --current
```

Expected pane fields include `agent: "opencode"` and a meaningful `agent_status`.

In OpenCode run:

```text
/herdr-status
/herdr-test
```

`/herdr-test` creates a temporary `oh-*` pane, starts a verified runtime, sends a simple sum prompt, waits, reads the answer, and closes the pane. It may use OpenCode itself as the verified runtime. If cluster policy disallows nested agents or process spawning, report that exact limitation and run `/herdr-delete` to clean up any plugin-owned `oh-*` panes.

Test the file viewer keybindings from Herdr. Confirm `prefix+f` opens the viewer in a split and `prefix+shift+f` opens/toggles its own tab. Do not close unrelated user panes.

## Phase 12: verify quota and telemetry

Restart OpenCode after plugin setup.

Run:

```text
/quota_status
/quota
/tokens_session
```

Expected quota UI behavior:

- No automatic toast.
- No sidebar panel.
- Compact status enabled on Home bottom and session prompt.
- Commands displayed inline.

Run one harmless OpenCode prompt, quit cleanly, then verify telemetry without exposing prompts:

```bash
octm report --days 1 --save
test -f "$XDG_DATA_HOME/opencode-telemetry/data.db"
```

If the `octm` binary is not on PATH because OpenCode installed the plugin only into its cache, either add a safe user-local shim/symlink to that exact package binary or use the package's generated slash commands. Do not hardcode ephemeral cache paths in project files if a stable CLI install solves it.

Do not copy source telemetry data. Start with a fresh Linux database.

## Phase 13: final audit and report

Inspect final files and permissions. Ensure no Windows paths, source username, plaintext tokens, sockets, caches, or session files were copied.

Report a compact table with:

- Component
- Desired version/state
- Actual version/state
- Verification command
- Result: pass, partial, skipped, or blocked
- Exact reason/remediation for anything not passing

List final paths:

- OpenCode config
- OpenCode TUI config
- quota config
- global skills root
- external skills root
- Herdr config
- Herdr binary
- Herdr plugin root/config dir
- telemetry database

List only secret variable names that must be set, never values. At minimum: `GITHUB_MCP_TOKEN`. Mention that OpenAI OAuth was re-established locally rather than copied.

Finally state whether all of these are true:

1. OpenCode starts cleanly.
2. Both MCPs connect or have documented cluster-policy blockers.
3. All 13 requested installed skills and built-in `customize-opencode` are discoverable.
4. Three OpenCode plugins are loaded at the requested versions.
5. Herdr is the exact preview build and config is valid.
6. Herdr file viewer 1.15.0 works with Unix action IDs.
7. Native Herdr OpenCode integration is current v9.
8. Herdr sees OpenCode state transitions.
9. Quota UI matches the source behavior.
10. Telemetry remains local and starts from a fresh database.

Do not claim complete replication if any check was not run.
