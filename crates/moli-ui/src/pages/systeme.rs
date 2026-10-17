/// Page Système — drivers, santé, version.
use maud::{Markup, html};
use moli_core::DriverStatus;
use moli_runtime::Snapshot;

use crate::icons::icon;
use crate::layout::page;

pub fn render(snap: &Snapshot, version: &str) -> Markup {
    let content = html! {
        div style="display:grid;gap:20px" {
            /* En-tête */
            section.card {
                div.card-head {
                    h2 { (icon("alert", 18)) "Moli OS " (version) }
                }
                div.grid {
                    div.col4 {
                        div.sensor {
                            small { "Appareils" }
                            b { (snap.devices.len()) }
                        }
                    }
                    div.col4 {
                        div.sensor {
                            small { "Drivers" }
                            b { (snap.drivers.len()) }
                        }
                    }
                    div.col4 {
                        div.sensor {
                            small { "Approbations" }
                            b { (snap.approvals.len()) }
                        }
                    }
                }
            }

            /* Drivers */
            section.card {
                div.card-head {
                    h2 { "Intégrations actives" }
                }
                div style="display:grid;gap:10px" {
                    @for driver in &snap.drivers {
                        div style="display:flex;align-items:center;justify-content:space-between;
                                   padding:12px 14px;background:var(--s2);border-radius:12px"
                        {
                            div {
                                div style="font-size:14px;font-weight:650" { (driver.integration) }
                                div.muted.small { (driver.kind) }
                            }
                            span style="padding:5px 12px;font-size:12px"
                                class=(match driver.status {
                                    DriverStatus::Running => "chip good",
                                    DriverStatus::Starting => "chip cool",
                                    DriverStatus::Backoff { .. } => "chip alert",
                                    DriverStatus::Waiting { .. } => "chip warm",
                                    DriverStatus::Stopped => "chip muted",
                                })
                            {
                                (match &driver.status {
                                    DriverStatus::Running => "Actif".to_owned(),
                                    DriverStatus::Starting => "Démarrage…".to_owned(),
                                    DriverStatus::Backoff { error, .. } => format!("Erreur : {error}"),
                                    DriverStatus::Waiting { reason } => format!("Attente : {reason}"),
                                    DriverStatus::Stopped => "Arrêté".to_owned(),
                                })
                            }
                        }
                    }
                    @if snap.drivers.is_empty() {
                        p.muted.small { "Aucun driver actif." }
                    }
                }
            }
        }
    };

    page("Système", "/systeme", content)
}
