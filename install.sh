#!/usr/bin/env bash
# ============================================================
#  Moli OS — script d'installation (LXC / VM / serveur bare)
#  Usage :
#    curl -fsSL https://raw.githubusercontent.com/rudy-io/moli-os/main/install.sh | bash
#  Ou après avoir téléchargé le binaire manuellement :
#    MOLI_BIN=/chemin/vers/moli-host ./install.sh
# ============================================================
set -euo pipefail

BINARY_NAME="moli-host"
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/etc/moli"
SERVICE_FILE="/etc/systemd/system/moli-os.service"
REPO="rudy-io/moli-os"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()  { echo -e "${GREEN}[moli]${NC} $*"; }
warn()  { echo -e "${YELLOW}[moli]${NC} $*"; }
die()   { echo -e "${RED}[moli] ERREUR${NC} $*" >&2; exit 1; }

# ── Vérifications ──────────────────────────────────────────
[[ $EUID -eq 0 ]] || die "Ce script doit être exécuté en tant que root (sudo $0)."
command -v systemctl &>/dev/null || die "systemd est requis."

# ── Téléchargement du binaire ──────────────────────────────
if [[ -n "${MOLI_BIN:-}" ]]; then
    info "Utilisation du binaire fourni : $MOLI_BIN"
    [[ -f "$MOLI_BIN" ]] || die "Fichier introuvable : $MOLI_BIN"
    BINARY_PATH="$MOLI_BIN"
else
    # Récupère la dernière release GitHub
    info "Récupération de la dernière version depuis GitHub…"
    LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)
    [[ -n "$LATEST" ]] || die "Impossible de récupérer la version depuis GitHub."
    info "Version : $LATEST"

    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY_NAME}-x86_64-linux-musl"
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
if [[ ! -f "$CONFIG_FILE" ]]; then
    info "Création d'une configuration par défaut dans $CONFIG_FILE…"
    cat > "$CONFIG_FILE" << 'EOF'
# Moli OS — configuration minimale.
# Éditez ce fichier puis relancez le service : systemctl restart moli-os

[server]
# Adresses supplémentaires pour joindre cette instance (en plus de localhost).
# allowed_hosts = ["192.168.1.42", "moli.local"]
port = 7070

# [z2m]
# Zigbee2MQTT WebSocket
# url = "ws://localhost:1883"

# [hue]
# Bridge Philips Hue
# host = "192.168.1.10"
# key  = "votre-clef-hue"
EOF
    warn "Pensez à éditer $CONFIG_FILE avant de démarrer le service."
fi

# ── Unité systemd ──────────────────────────────────────────
info "Installation de l'unité systemd $SERVICE_FILE…"
cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Moli OS — domotique ultra-légère
Documentation=https://github.com/${REPO}
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/$BINARY_NAME --config $CONFIG_FILE
Restart=on-failure
RestartSec=5s

# Sécurité
NoNewPrivileges=true
PrivateTmp=true
ProtectHome=true
ProtectSystem=strict
ReadWritePaths=$DATA_DIR

# Limites ressources
MemoryMax=64M
TasksMax=64

[Install]
WantedBy=multi-user.target
EOF

# ── Activation ─────────────────────────────────────────────
info "Activation et démarrage du service…"
systemctl daemon-reload
systemctl enable --now moli-os.service

# ── Résultat ───────────────────────────────────────────────
echo ""
echo -e "${GREEN}✓ Moli OS installé avec succès !${NC}"
echo ""
echo "  Dashboard : http://localhost:7070"
echo "  Logs      : journalctl -fu moli-os"
echo "  Config    : $CONFIG_FILE"
echo "  Binaire   : $INSTALL_DIR/$BINARY_NAME"
echo ""
warn "Si vous avez modifié la config, relancez : systemctl restart moli-os"
