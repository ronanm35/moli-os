//! Moli OS — rendu HTML server-side.
//! Zéro framework JS, zéro node_modules.
//! Le binaire sert du HTML pur + SSE + ~500 octets de vanilla JS.

pub mod css;
pub mod icons;
pub mod js;
pub mod layout;
pub mod pages;

use axum::http::{StatusCode, Uri, header};
use axum::response::{Html, IntoResponse, Response};
use moli_runtime::Hub;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Rendu de la page demandée à partir du snapshot du Hub.
pub async fn serve(uri: Uri, hub: Hub) -> Response {
    let snap = hub.snapshot();
    let path = uri.path();

    let markup = match path {
        "/" | "/index.html" => pages::accueil::render(&snap),
        "/pieces"           => pages::pieces::render(&snap),
        "/systeme"          => pages::systeme::render(&snap, VERSION),
        // Pages non encore portées → redirection vers l'accueil
        _ => {
            return (
                StatusCode::TEMPORARY_REDIRECT,
                [(header::LOCATION, "/")],
            ).into_response();
        }
    };

    Html(markup.into_string()).into_response()
}
