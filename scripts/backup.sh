#!/bin/bash
# 4x-ui 自动备份脚本
BACKUP_DIR="/opt/4x-ui/backup"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR
cp /opt/4x-ui/config/* $BACKUP_DIR/ 2>/dev/null
cp /opt/4x-ui/database/* $BACKUP_DIR/ 2>/dev/null
cp /opt/4x-ui/blockchain-auth/blockchain_auth.db $BACKUP_DIR/blockchain_auth_$DATE.db 2>/dev/null
cd $BACKUP_DIR && tar czf 4x-ui-backup-$DATE.tar.gz *
echo "[4x-ui] 备份完成: $BACKUP_DIR/4x-ui-backup-$DATE.tar.gz" 