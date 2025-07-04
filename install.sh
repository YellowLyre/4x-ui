#!/bin/bash

# 4x-ui 一键安装脚本
# 项目地址: https://github.com/YellowLyre/4x-ui
# 作者: YellowLyre
# 参考自3x-ui官方install.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
PLAIN='\033[0m'

cur_dir=$(pwd)

# 检查root权限
[[ $EUID -ne 0 ]] && echo -e "${RED}Fatal error: ${PLAIN} 请用root权限运行本脚本\n " && exit 1

# 检查系统类型
if [[ -f /etc/os-release ]]; then
    source /etc/os-release
    release=$ID
elif [[ -f /usr/lib/os-release ]]; then
    source /usr/lib/os-release
    release=$ID
else
    echo "无法检测系统类型，请联系作者！" >&2
    exit 1
fi

echo -e "${BLUE}检测到系统: $release${PLAIN}"

arch() {
    case "$(uname -m)" in
        x86_64 | x64 | amd64) echo 'amd64' ;;
        i*86 | x86) echo '386' ;;
        armv8* | armv8 | arm64 | aarch64) echo 'arm64' ;;
        armv7* | armv7 | arm) echo 'armv7' ;;
        armv6* | armv6) echo 'armv6' ;;
        armv5* | armv5) echo 'armv5' ;;
        s390x) echo 's390x' ;;
        *) echo -e "${YELLOW}不支持的CPU架构!${PLAIN}" && exit 1 ;;
    esac
}

# 安装基础依赖
install_base() {
    case "${release}" in
        ubuntu | debian | armbian) apt-get update && apt-get install -y wget curl tar tzdata ;;
        centos | rhel | almalinux | rocky | ol) yum -y update && yum install -y wget curl tar tzdata ;;
        fedora | amzn | virtuozzo) dnf -y update && dnf install -y wget curl tar tzdata ;;
        arch | manjaro | parch) pacman -Syu && pacman -Syu --noconfirm wget curl tar tzdata ;;
        opensuse-tumbleweed) zypper refresh && zypper -q install -y wget curl tar timezone ;;
        *) apt-get update && apt install -y wget curl tar tzdata ;;
    esac
}

# 克隆4x-ui仓库
clone_repo() {
    if [ -d "/opt/4x-ui" ]; then
        echo -e "${YELLOW}目录/opt/4x-ui已存在，跳过克隆。${PLAIN}"
    else
        git clone https://github.com/YellowLyre/4x-ui.git /opt/4x-ui
    fi
}

# 构建并启动
build_and_start() {
    cd /opt/4x-ui
    bash start-blockchain-auth.sh start
}

main() {
    echo -e "${BLUE}==== 4x-ui 一键安装开始 ====${PLAIN}"
    install_base
    clone_repo
    build_and_start
    echo -e "${GREEN}🎉 4x-ui 安装并启动完成！${PLAIN}"
    echo -e "访问面板: http://<你的服务器IP>:54321"
    echo -e "区块链登录: http://<你的服务器IP>:54321/blockchain-login"
}

main
