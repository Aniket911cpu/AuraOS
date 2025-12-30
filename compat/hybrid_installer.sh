#!/bin/bash
# Aura OS "Hybrid Installer" (Prototype)
# Automates the creation of "Bottles" (Isolated Wine Prefixes) for legacy Windows apps.

# Configuration
AURA_ROOT="$HOME/.aura"
BOTTLES_DIR="$AURA_ROOT/bottles"
DB_API_URL="https://api.auraos.io/v1/recipes" # Mock API

# Colors for "Liquid Glass" CLI aesthetic
BLUE='\033[1;34m'
CYAN='\033[0;36m'
GREEN='\033[1;32m'
NC='\033[0m' # No Color

echo -e "${BLUE}:: Aura OS Hybrid Installer v0.1 ::${NC}"

# 1. Input Validation
exe_path="$1"
if [ -z "$exe_path" ]; then
    echo "Usage: ./hybrid_install.sh <path_to_exe>"
    exit 1
fi

if [ ! -f "$exe_path" ]; then
    echo "Error: File '$exe_path' not found."
    exit 1
fi

app_name=$(basename "$exe_path" .exe)
bottle_name=$(echo "$app_name" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
bottle_path="$BOTTLES_DIR/$bottle_name"

echo -e "${CYAN}-> Detected Application: $app_name${NC}"
echo -e "${CYAN}-> Target Bottle: $bottle_name${NC}"

# 2. Check Remote Database for Dependencies (Mock)
echo -e "${BLUE}-> Querying transmutation recipes for '$app_name'...${NC}"
# Simulating network delay
sleep 1

# Mock Logic: Check if it's a known app
dependencies=("corefonts" "dxvk")
if [[ "$app_name" == *"Steam"* ]]; then
    dependencies+=("vcrun2017" "arial")
    echo -e "${GREEN}   Found recipe for Steam!${NC}"
elif [[ "$app_name" == *"Notepad"* ]]; then
     echo -e "${GREEN}   Standard recipe applied.${NC}"
else
    echo -e "${CYAN}   No specific recipe found. Using Generic Bottle profile.${NC}"
fi

# 3. Create Bottle (Isolation)
if [ -d "$bottle_path" ]; then
    echo -e "${CYAN}-> Bottle '$bottle_name' already exists. Reusing...${NC}"
else
    echo -e "${BLUE}-> Blowing new glass Bottle at '$bottle_path'...${NC}"
    mkdir -p "$bottle_path"
    
    # 4. Initialize Wine Prefix
    echo -e "${BLUE}-> Infusing Wine Prefix...${NC}"
    
    # In a real environment, we would run:
    # WINEPREFIX="$bottle_path" wineboot --init
    
    # Mocking the creation
    mkdir -p "$bottle_path/drive_c/windows"
    touch "$bottle_path/system.reg"
    sleep 2 # Simulate processing
    echo -e "${GREEN}   Prefix initialized.${NC}"
fi

# 5. Install Dependencies (Winetricks Mock)
for dep in "${dependencies[@]}"; do
    echo -e "${BLUE}-> Injecting dependency: $dep${NC}"
    # WINEPREFIX="$bottle_path" winetricks -q "$dep"
    sleep 0.5
done

# 6. Execute Installer
echo -e "${GREEN}-> Launching Installer inside Liquid Glass container...${NC}"
echo -e "   [EXEC] WINEPREFIX=$bottle_path wine \"$exe_path\" /S"

# Mock Execution
# WINEPREFIX="$bottle_path" wine "$exe_path" /S
sleep 2

echo -e "${BLUE}:: Installation Sequence Complete ::${NC}"
echo -e "${CYAN}You can now launch '$app_name' from the Context Orb.${NC}"
exit 0
