//! Dashboard rendu server-side par moli-ui (maud + SSE + ~500 octets de JS).
//! Aucun fichier statique, aucune dépendance rust-embed ni node_modules.

use axum::extract::State;
use axum::http::Uri;
use axum::response::Response;
use moli_runtime::Hub;

/// Fallback : toutes les requêtes non-API atterrissent ici.
pub(crate) async fn serve(State(hub): State<Hub>, uri: Uri) -> Response {
    moli_ui::serve(uri, hub).await
}
