########## 1. Mostrar YAML de los agentes ##########
echo "\n=== agent1_config.yaml ==="
cat python_agents/config/agent1_config.yaml

echo "\n=== agent2_config.yaml ==="
cat python_agents/config/agent2_config.yaml


########## 2. Echar un vistazo a las claves ##########
echo "\n=== Primera línea de agent1.key ==="
head -n 5 python_agents/keys/agent1.key

echo "\n=== Primera línea de agent2.key ==="
head -n 5 python_agents/keys/agent2.key


########## 3. Comprobar que rustcore arranca ##########
echo "\n=== Versión de Rust / Cargo ==="
rustc --version
cargo --version

echo "\n=== Intentando arrancar rustcore (5 s) ==="
# Ejecutamos en segundo plano, esperamos 5 s y lo matamos
cargo run --quiet &
PID=$!
sleep 5
kill $PID 2>/dev/null
wait $PID 2>/dev/null

echo "\n=== Fin del script ==="

