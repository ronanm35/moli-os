#!/bin/sh
# ============================================================
#  Moli OS — script d'installation (Alpine LXC / Debian / Ubuntu)
#  Usage :
#    curl -fsSL https://raw.githubusercontent.com/ronanm35/moli-os/main/install.sh | sh
#  Ou avec un binaire local :
#    MOLI_BIN=/chemin/vers/moli-os ./install.sh
# ============================================================
set -eu

BINARY_NAME="moli-os"
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/etc/moli"
REPO="ronanm35/moli-os"

info()  { printf '\033[0;32m[moli]\033[0m %s\n' "$*"; }
warn()  { printf '\033[1;33m[moli]\033[0m %s\n' "$*"; }
die()   { printf '\033[0;31m[moli] ERREUR\033[0m %s\n' "$*" >&2; exit 1; }

# ── Vérifications ──────────────────────────────────────────
[ "$(id -u)" -eq 0 ] || die "Ce script doit être exécuté en tant que root."
command -v curl >/dev/null 2>&1 || die "curl est requis (apk add curl)."

# ── Téléchargement du binaire ──────────────────────────────
if [ -n "${MOLI_BIN:-}" ]; then
    info "Utilisation du binaire fourni : $MOLI_BIN"
    [ -f "$MOLI_BIN" ] || die "Fichier introuvable : $MOLI_BIN"
    BINARY_PATH="$MOLI_BIN"
else
    info "Récupération de la dernière version depuis GitHub…"
    LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)
    [ -n "$LATEST" ] || die "Impossible de récupérer la version depuis GitHub."
    info "Version : $LATEST"

    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY_NAME}"
    info "Téléchargement : $URL"
    curl -fsSL "$URL" -o "$TMP/$BINARY_NAME" || die "Téléchargement échoué."
    BINARY_PATH="$TMP/$BINARY_NAME"
fi

# ── Installation du binaire ────────────────────────────────
info "Installation dans $INSTALL_DIR/$BINARY_NAME…"
install -m 0755 "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"

# ── Répertoire de données ──────────────────────────────────
info "Création du répertoire de données $DATA_DIR…"
mkdir -p "$DATA_DIR"
chmod 750 "$DATA_DIR"

# ── Configuration minimale ─────────────────────────────────
CONFIG_FILE="$DATA_DIR/home.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    info "Création d'une configuration par défaut dans $CONFIG_FILE…"
    cat > "$CONFIG_FILE" << 'EOF'
# Moli OS — configuration minimale.
# Éditez ce fichier puis relancez le service.

[server]
listen = "0.0.0.0:7070"
# allowed_hosts = ["192.168.1.42", "moli.local"]

# [z2m]
# url = "ws://localhost:1883"

# [hue]
# host = "192.168.1.10"
# key  = "votre-clef-hue"
EOF
    warn "Pensez à éditer $CONFIG_FILE avant de démarrer le service."
fi

# ── Service (systemd ou OpenRC) ────────────────────────────
if command -v systemctl >/dev/null 2>&1; then
    info "Installation du service systemd…"
    cat > /etc/systemd/system/moli-os.service << EOF
[Unit]
Description=Moli OS — domotique ultra-légère
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/$BINARY_NAME --config $CONFIG_FILE
Restart=on-failure
RestartSec=5s
NoNewPrivileges=true
PrivateTmp=true
ProtectHome=true
ProtectSystem=strict
ReadWritePaths=$DATA_DIR
MemoryMax=64M
TasksMax=64

[Install]
WantedBy=multi-user.target
EOF
    systemctl daemon-reload
    systemctl enable --now moli-os.service

elif command -v rc-update >/dev/null 2>&1; then
    info "Installation du service OpenRC (Alpine)…"
    cat > /etc/init.d/moli-os << EOF
#!/sbin/openrc-run
description="Moli OS — domotique ultra-légère"
command="$INSTALL_DIR/$BINARY_NAME"
command_args="--config $CONFIG_FILE"
command_background=true
pidfile="/run/moli-os.pid"
depend() { need net; }
EOF
    chmod +x /etc/init.d/moli-os
    rc-update add moli-os default
    rc-service moli-os start

else
    warn "Ni systemd ni OpenRC trouvé. Lancez manuellement :"
    warn "  $INSTALL_DIR/$BINARY_NAME --config $CONFIG_FILE"
fi

# ── Résultat ───────────────────────────────────────────────
printf '\n\033[0;32m✓ Moli OS installé avec succès !\033[0m\n\n'
printf '  Dashboard : http://localhost:7070\n'
printf '  Config    : %s\n' "$CONFIG_FILE"
printf '  Binaire   : %s/%s\n\n' "$INSTALL_DIR" "$BINARY_NAME"
