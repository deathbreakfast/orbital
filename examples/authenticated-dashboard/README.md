# authenticated-dashboard

**Teaches:** `provide_auth_context` + `init_auth_resource`, [`RequireAuthenticated`](../../orbital/src/routes.rs) route gate, and theme mode / density toggles persisted to `localStorage`.

**Topology:** Embedded demo session (in-memory `AuthSession` — not a real IdP). Swap the Sign in handler for your session server function in production.

## Prerequisites

```bash
# cargo install cargo-leptos   # once
export LEPTOS_OUTPUT_NAME=authenticated-dashboard
```

## Run

```bash
cargo leptos watch --split --project authenticated-dashboard
```

Open <http://127.0.0.1:3031/>:

1. Visit `/dashboard` while signed out → auth-required gate.
2. **Sign in** → reopen `/dashboard` → protected card.
3. Toggle **Dark** / **Compact** → refresh → prefs restore.

Compile-check:

```bash
cargo check -p authenticated-dashboard --features ssr
```

**Open first:** [`src/app.rs`](src/app.rs) → [`src/theme_prefs.rs`](src/theme_prefs.rs)

**Success:** gate blocks anonymous `/dashboard`; after Sign in the dashboard card renders; theme/density survive reload.

**Next step:** [`../server-paged-analytics`](../server-paged-analytics/) for server-paged `DataTable` + chart.
