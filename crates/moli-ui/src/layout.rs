/// Shell HTML : `<head>`, header, nav, footer.
use maud::{DOCTYPE, Markup, html};

use crate::css::STYLES;
use crate::icons::icon;
use crate::js::SCRIPT;

/// Pages disponibles dans la navigation.
pub struct NavItem {
    pub href: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

pub const NAV: &[NavItem] = &[
    NavItem { href: "/",         label: "Accueil",      icon: "home" },
    NavItem { href: "/pieces",   label: "Pièces",       icon: "light" },
    NavItem { href: "/energie",  label: "Énergie",      icon: "bolt" },
    NavItem { href: "/cameras",  label: "Caméras",      icon: "bell" },
    NavItem { href: "/dehors",   label: "Dehors",       icon: "flower-outline" },
    NavItem { href: "/systeme",  label: "Système",      icon: "alert" },
];

/// Page complète : head + shell + contenu.
pub fn page(title: &str, current: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="fr" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                title { (title) " — Moli OS" }
                style { (maud::PreEscaped(STYLES)) }
                link
                    rel="preconnect"
                    href="https://fonts.googleapis.com";
                link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Figtree:wght@400..800&display=swap";
            }
            body {
                div #shell {
                    header #header {
                        div.logo {
                            (icon("home", 22))
                            "Moli"
                        }
                        nav {
                            @for item in NAV {
                                a href=(item.href)
                                  class=[if item.href == current { Some("on") } else { None }]
                                {
                                    (icon(item.icon, 16))
                                    span { (item.label) }
                                }
                            }
                        }
                        div.pill {
                            div #live-dot {}
                            "Live"
                        }
                    }
                    main #content {
                        (content)
                    }
                }
                script { (maud::PreEscaped(SCRIPT)) }
            }
        }
    }
}
