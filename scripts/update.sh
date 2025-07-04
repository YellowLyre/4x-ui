#!/bin/bash
# 4x-ui 自动升级脚本
cd /opt/4x-ui
git pull
bash start-blockchain-auth.sh restart
echo "[4x-ui] 升级并重启完成！" 