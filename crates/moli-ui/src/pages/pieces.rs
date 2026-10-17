/// Page Pièces — tous les appareils, regroupés par pièce.
use maud::{Markup, html};
use moli_core::Value;
use moli_runtime::{DeviceView, Snapshot};
use std::collections::BTreeMap;

use crate::layout::page;

fn device_card(d: &DeviceView) -> Markup {
    let id = d.device.id.as_str();
    let name = d.label.name.as_deref()
        .unwrap_or(d.device.native_name.as_ref());
    let unavail = d.online == Some(false);

    // Points principaux à afficher
    let has_on = d.device.points.iter().any(|p| p.key.as_ref() == "on");
    let has_temp = d.device.points.iter().any(|p| p.key.as_ref() == "temperature");
    let has_hum  = d.device.points.iter().any(|p| p.key.as_ref() == "humidity");
    let has_power = d.device.points.iter().any(|p| p.key.as_ref() == "power");
    let has_battery = d.device.points.iter().any(|p| p.key.as_ref() == "battery");

    let on_val = matches!(d.state.get("on").map(|s| &s.value), Some(Value::Bool(true)));

    html! {
        div
            id={"dev-" (id)}
            class=(if unavail { "card unavail" } else { "card" })
            style="padding:16px;border-radius:16px"
        {
            div style="display:flex;align-items:center;justify-content:space-between;gap:8px;margin-bottom:10px" {
                span style="font-size:14px;font-weight:650" { (name) }
                @if unavail {
                    span class="chip muted" style="padding:4px 10px;font-size:12px" { "Hors ligne" }
                } @else if has_on {
                    button
                        class=(if on_val { "chip warm" } else { "chip muted" })
                        style="padding:6px 12px;font-size:13px"
                        data-p={(id)"/on"} data-f="toggle"
                        aria-pressed=(if on_val {"true"} else {"false"})
                        onclick={"mact('" (id) "/on'," (if on_val {"false"} else {"true"}) ")"}
                    {
                        (if on_val { "Allumé" } else { "Éteint" })
                    }
                }
            }
            div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(80px,1fr));gap:8px" {
                @if has_temp {
                    div.sensor {
                        small { "Temp." }
                        b data-p={(id)"/temperature"} data-u="°C" {
                            (d.state.get("temperature")
                                .and_then(|s| if let Value::Float(f) = s.value { Some(format!("{f:.1}°C")) } else { None })
                                .unwrap_or_else(|| "—".to_owned()))
                        }
                    }
                }
                @if has_hum {
                    div.sensor {
                        small { "Humidité" }
                        b data-p={(id)"/humidity"} data-u=" %" {
                            (d.state.get("humidity")
                                .and_then(|s| if let Value::Float(f) = s.value { Some(format!("{f:.0} %")) } else { None })
                                .unwrap_or_else(|| "—".to_owned()))
                        }
                    }
                }
                @if has_power {
                    div.sensor {
                        small { "Puissance" }
                        b data-p={(id)"/power"} data-u=" W" {
                            (d.state.get("power")
                                .and_then(|s| match &s.value {
                                    Value::Float(f) => Some(format!("{f:.0} W")),
                                    Value::Int(i) => Some(format!("{i} W")),
                                    _ => None,
                                })
                                .unwrap_or_else(|| "— W".to_owned()))
                        }
                    }
                }
                @if has_battery {
                    div.sensor {
                        small { "Batterie" }
                        b data-p={(id)"/battery"} data-u=" %" {
                            (d.state.get("battery")
                                .and_then(|s| match &s.value {
                                    Value::Float(f) => Some(format!("{f:.0} %")),
                                    Value::Int(i) => Some(format!("{i} %")),
                                    _ => None,
                                })
                                .unwrap_or_else(|| "—".to_owned()))
                        }
                    }
                }
            }
        }
    }
}

pub fn render(snap: &Snapshot) -> Markup {
    // Grouper par pièce
    let mut rooms: BTreeMap<String, Vec<&DeviceView>> = BTreeMap::new();
    for d in &snap.devices {
        let room = d.label.room.clone()
            .unwrap_or_else(|| d.device.native_room.as_deref()
                .unwrap_or("Autres").to_owned());
        rooms.entry(room).or_default().push(d);
    }

    let content = html! {
        div style="display:grid;gap:28px" {
            @for (room, devs) in &rooms {
                section {
                    h2 style="font-size:17px;font-weight:700;margin-bottom:14px;
                               display:flex;align-items:center;gap:8px;color:var(--ink2)"
                    {
                        (room)
                        span class="muted small" style="font-weight:500" {
                            " · " (devs.len()) " appareil" (if devs.len() > 1 {"s"} else {""})
                        }
                    }
                    div.grid {
                        @for d in devs {
                            div.col4 {
                                (device_card(d))
                            }
                        }
                    }
                }
            }
            @if rooms.is_empty() {
                p.muted { "Aucun appareil configuré." }
            }
        }
    };

    page("Pièces", "/pieces", content)
}
