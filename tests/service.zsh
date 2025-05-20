#!/usr/bin/env zsh
set -euo pipefail

########################
# 0. Directorios base
########################
PY_DIR=$PWD                 # = python_agents
VENV_DIR=$PY_DIR/venv
ROOT_DIR=$PY_DIR/..
RUST_DIR=$ROOT_DIR/rustcore
KEY_DIR=$PY_DIR/keys

########################
# 1. (Re)crear venv 3.11
########################
echo "\n▶️  1/5  Creando venv con Python 3.11"
rm -rf "$VENV_DIR"
python3.11 -m venv "$VENV_DIR"
source "$VENV_DIR/bin/activate"

########################
# 2. Instalar deps + base58
########################
echo "\n▶️  2/5  Instalando requirements + base58"
python -m pip install --upgrade pip  >/dev/null
python -m pip install -r requirements.txt base58 >/dev/null

########################
# 3. Convertir “x” → Base58 y mostrar
########################
echo "\n▶️  3/5  Convirtiendo claves públicas"
python - <<'PY'
import base64, base58, json, yaml, pathlib, textwrap
cfg = yaml.safe_load(open("config/agent1_config.yaml"))
did1 = cfg["did"]; did2 = yaml.safe_load(open("config/agent2_config.yaml"))["did"]
j1 = json.load(open("keys/agent1.key")); j2 = json.load(open("keys/agent2.key"))
b58_1 = base58.b58encode(base64.urlsafe_b64decode(j1["x"]+"==")).decode()
b58_2 = base58.b58encode(base64.urlsafe_b64decode(j2["x"]+"==")).decode()
print(textwrap.dedent(f"""
🚀  Pega estas líneas en rustcore/src/didcomm.rs:

    const A1_DID  : &str = "{did1}";
    const A1_PRIV : &str = "{j1['d']}";
    const A1_PUB  : &str = "{b58_1}";

    const A2_DID  : &str = "{did2}";
    const A2_PRIV : &str = "{j2['d']}";
    const A2_PUB  : &str = "{b58_2}";
"""))
PY

########################
# 4. Compilar y lanzar rustcore
########################
echo "\n▶️  4/5  Compilando rustcore"
cd "$RUST_DIR"
cargo build --quiet
echo "✅ Compilado – ejecutando en segundo plano…"
RUST_LOG=warn cargo run --quiet &
RUST_PID=$!
sleep 2 ; echo "   rustcore en http://127.0.0.1:3000  (PID $RUST_PID)"

########################
# 5. Instrucciones finales
########################
cat <<EOF

▶️  5/5  Abre dos terminales nuevas y corre:

# Terminal A  (receptor)
cd python_agents
source venv/bin/activate
python agent2.py

# Terminal B  (emisor)
cd python_agents
source venv/bin/activate
python agent1.py

Cuando veas:
  [Agent2] 📩 Received: ...
  [Agent1] ✅ Agent2 responded: ...
¡la demo DIDComm está funcionando!

Para detener rustcore:
  kill $RUST_PID
EOF

