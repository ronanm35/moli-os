/// Page d'accueil — dashboard principal.
use maud::{Markup, html};
use moli_core::Value;
use moli_runtime::{DeviceView, Snapshot};

use crate::layout::page;

/// Valeur d'un point dans le snapshot.
fn val<'a>(devices: &'a [DeviceView], device_id: &str, key: &str) -> Option<&'a Value> {
    devices
        .iter()
        .find(|d| d.device.id.as_str() == device_id)
        .and_then(|d| d.state.get(key))
        .map(|s| &s.value)
}

fn val_bool(devices: &[DeviceView], device_id: &str, key: &str) -> bool {
    matches!(val(devices, device_id, key), Some(Value::Bool(true)))
}

fn val_text<'a>(devices: &'a [DeviceView], device_id: &str, key: &str) -> &'a str {
    match val(devices, device_id, key) {
        Some(Value::Text(s)) => s,
        _ => "—",
    }
}

fn val_float(devices: &[DeviceView], device_id: &str, key: &str) -> Option<f64> {
    match val(devices, device_id, key) {
        Some(Value::Float(f)) => Some(*f),
        Some(Value::Int(i)) => Some(*i as f64),
        _ => None,
    }
}

/// Tuile d'un luminaire.
fn light_tile(d: &DeviceView) -> Markup {
    let id = d.device.id.as_str();
    let name = d.label.name.as_deref()
        .unwrap_or(d.device.native_name.as_ref());
    // Utilise directement l'état du snapshot
    let on = matches!(d.state.get("on").map(|s| &s.value), Some(Value::Bool(true)));
    let point = format!("{id}/on");
    html! {
        button
            id={"dev-" (id)}
            class=(if on { "light-tile on" } else { "light-tile" })
            data-p=(point)
            data-f="bool"
            onclick={"mact('" (point) "'," (if on {"false"} else {"true"}) ",'" (name) "')"}
        {
            div.icon { "💡" }
            span.name { (name) }
        }
    }
}

/// Carte climat (climatisation / chauffage).
fn climate_card(d: &DeviceView) -> Markup {
    let id = d.device.id.as_str();
    let name = d.label.name.as_deref()
        .unwrap_or(d.device.native_name.as_ref());
    let temp = match d.state.get("temperature").map(|s| &s.value) {
        Some(Value::Float(f)) => Some(*f),
        Some(Value::Int(i)) => Some(*i as f64),
        _ => None,
    };
    let set = match d.state.get("target_temperature").map(|s| &s.value) {
        Some(Value::Float(f)) => Some(*f),
        Some(Value::Int(i)) => Some(*i as f64),
        _ => None,
    };
    let temp_str = temp.map(|f| format!("{f:.1}°")).unwrap_or_else(|| "—".to_owned());
    let set_str  = set.map(|f| format!("{f:.0}°")).unwrap_or_else(|| "—".to_owned());
    html! {
        div.climate-card id={"dev-" (id)} {
            div {
                div.temp data-p={(id)"/temperature"} data-u="°" { (temp_str) }
                div.set  {
                    "→ "
                    span data-p={(id)"/target_temperature"} data-u="°" { (set_str) }
                }
                div.small.muted { (name) }
            }
            div.ctrl {
                button
                    title="Baisser"
                    onclick={"mact('" (id) "/target_temperature'," (set.unwrap_or(20.0) - 0.5) ")"}
                { "−" }
                button
                    title="Monter"
                    onclick={"mact('" (id) "/target_temperature'," (set.unwrap_or(20.0) + 0.5) ")"}
                { "+" }
            }
        }
    }
}

/// Héros météo (haut de page).
fn sky_hero(snap: &Snapshot) -> Markup {
    let devices = &snap.devices;
    let weather = devices.iter().find(|d| {
        d.device.id.as_str().contains("meteo") ||
        d.device.id.as_str().contains("weather") ||
        d.device.id.as_str().contains("forecast")
    });
    let (temp, desc) = if let Some(w) = weather {
        let id = w.device.id.as_str();
        let t = val_float(devices, id, "temperature")
            .map(|f| format!("{f:.0}°C"))
            .unwrap_or_else(|| "—".to_owned());
        let d = val_text(devices, id, "condition");
        (t, d.to_owned())
    } else {
        ("—".to_owned(), "Météo non configurée".to_owned())
    };

    html! {
        div.sky-hero {
            div.sky-row {
                div {
                    div.temp-main { (temp) }
                    div.sky-desc  { (desc) }
                }
            }
        }
    }
}

/// Page complète d'accueil.
pub fn render(snap: &Snapshot) -> Markup {
    let devices = &snap.devices;

    /* ── Lumières favorites ─────────────────────────────────── */
    let lights: Vec<&DeviceView> = devices
        .iter()
        .filter(|d| d.device.points.iter().any(|p| p.key.as_ref() == "on"))
        .take(9)
        .collect();

    let lit_count = lights
        .iter()
        .filter(|d| matches!(d.state.get("on").map(|s| &s.value), Some(Value::Bool(true))))
        .count();

    /* ── Climat ─────────────────────────────────────────────── */
    let climates: Vec<&DeviceView> = devices
        .iter()
        .filter(|d| {
            d.device.points.iter().any(|p| p.key.as_ref() == "temperature")
                && d.device.points.iter().any(|p| p.key.as_ref() == "target_temperature")
        })
        .take(4)
        .collect();

    /* ── Énergie ─────────────────────────────────────────────── */
    let power_dev = devices.iter()
        .find(|d| d.device.id.as_str().contains("energie"));
    let power_str = power_dev
        .and_then(|d| val_float(devices, d.device.id.as_str(), "power"))
        .map(|w| format!("{w:.0} W"))
        .unwrap_or_else(|| "— W".to_owned());
    let power_point = power_dev.map(|d| format!("{}/power", d.device.id));

    /* ── TV ─────────────────────────────────────────────────── */
    let tv_dev = devices.iter().find(|d| d.device.id.as_str().contains("tv"));
    let tv_on = tv_dev
        .map(|d| matches!(d.state.get("power").map(|s| &s.value), Some(Value::Bool(true))))
        .unwrap_or(false);
    let tv_point = tv_dev.map(|d| format!("{}/power", d.device.id));

    /* ── Dehors ─────────────────────────────────────────────── */
    let outdoor = devices.iter().find(|d| {
        let id = d.device.id.as_str();
        id.contains("outdoor") || id.contains("exterieur") ||
        d.label.room.as_deref() == Some("Extérieur")
    });
    let outdoor_id = outdoor.map(|d| d.device.id.as_str());
    let hum_str = outdoor_id
        .and_then(|id| val_float(devices, id, "humidity"))
        .map(|h| format!("{h:.0} %"))
        .unwrap_or_else(|| "—".to_owned());
    let uv_str = outdoor_id
        .and_then(|id| val_float(devices, id, "uv_index"))
        .map(|u| format!("{u:.0}"))
        .unwrap_or_else(|| "—".to_owned());

    /* ── Notices ─────────────────────────────────────────────── */
    let approvals_count = snap.approvals.len();

    let content = html! {
        div.accueil {

            /* Héros météo */
            (sky_hero(snap))

            /* Alertes / notices */
            section.notices aria-label="À savoir" {
                @if approvals_count > 0 {
                    a class="chip cool" href="/atelier" {
                        (approvals_count) " demande" (if approvals_count > 1 {"s"} else {""}) " en attente"
                    }
                } @else {
                    span class="chip good" {
                        "Tout est calme"
                    }
                }
            }

            /* Grille principale */
            div.grid {

                /* Lumières */
                section.card.col8 {
                    div.card-head {
                        h2 {
                            "Lumières"
                            span class="muted small" style="margin-left:6px;font-weight:550" {
                                @if lit_count > 0 {
                                    (lit_count) " allumée" (if lit_count > 1 {"s"} else {""})
                                } @else {
                                    "Tout éteint"
                                }
                            }
                        }
                        a class="more" href="/pieces" aria-label="Toutes les pièces" { "↗" }
                    }
                    div.tiles {
                        @for d in &lights {
                            (light_tile(d))
                        }
                        @if lights.is_empty() {
                            p class="muted small" { "Aucun luminaire configuré" }
                        }
                    }
                }

                /* Colonne latérale */
                div.col4 style="display:grid;gap:20px" {
                    /* Énergie rapide */
                    section.card {
                        div.card-head {
                            h2 { "Énergie" }
                            a class="more" href="/energie" { "↗" }
                        }
                        div style="font-size:28px;font-weight:800;line-height:1.1" {
                            span
                                data-p=[power_point]
                                data-u=" W"
                            { (power_str) }
                        }
                        p class="muted small" style="margin-top:4px" { "Consommation actuelle" }
                    }
                }

                /* Climat */
                @if !climates.is_empty() {
                    section.card.col7 {
                        div.card-head {
                            h2 { "Climatisation" }
                        }
                        div.clims {
                            @for d in &climates {
                                (climate_card(d))
                            }
                        }
                    }
                }

                /* Télécommande TV */
                a class="card remote-card col3" href="/salon" {
                    span.tv-obj {
                        span
                            class=(if tv_on { "screen on" } else { "screen" })
                            data-p=[tv_point]
                            data-f="bool"
                        {}
                        span.stand {}
                    }
                    span.remote-text {
                        strong { "Télécommande" }
                        span { "Salon" }
                    }
                    span.go-btn { "↗" }
                }

                /* Dehors */
                a class="card outside-card col4" href="/dehors" {
                    div.card-head {
                        h2 { "Dehors" }
                        span class="more" aria-hidden="true" { "↗" }
                    }
                    div.facts {
                        div {
                            small { "Humidité" }
                            b.num { (hum_str) }
                        }
                        div {
                            small { "UV" }
                            b.num { (uv_str) }
                        }
                    }
                }
            }
        }
    };

    page("Accueil", "/", content)
}
