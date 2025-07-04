#!/bin/bash

# 4x-ui 区块链认证微服务启动脚本
# 作者: YellowLyre
# 版本: 1.0.0

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BLOCKCHAIN_AUTH_DIR="$PROJECT_ROOT/blockchain-auth"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    log_info "检查系统依赖..."
    
    # 检查Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo/Rust 未安装！请先安装 Rust 工具链"
        exit 1
    fi
    
    # 检查Go
    if ! command -v go &> /dev/null; then
        log_error "Go 未安装！请先安装 Go"
        exit 1
    fi
    
    log_success "依赖检查完成"
}

# 构建区块链认证微服务
build_blockchain_service() {
    log_info "构建区块链认证微服务..."
    
    cd "$BLOCKCHAIN_AUTH_DIR"
    
    # 检查是否需要安装依赖
    if [ ! -f "Cargo.lock" ]; then
        log_info "首次构建，下载依赖中..."
        cargo fetch
    fi
    
    # 构建项目
    if cargo build --release; then
        log_success "区块链认证微服务构建完成"
    else
        log_error "区块链认证微服务构建失败"
        exit 1
    fi
    
    cd "$PROJECT_ROOT"
}

# 启动区块链认证微服务
start_blockchain_service() {
    log_info "启动区块链认证微服务..."
    
    cd "$BLOCKCHAIN_AUTH_DIR"
    
    # 检查端口是否被占用
    if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null; then
        log_warning "端口 8080 已被占用，尝试终止现有进程..."
        pkill -f "blockchain-auth" || true
        sleep 2
    fi
    
    # 启动服务
    nohup ./target/release/blockchain-auth > blockchain-auth.log 2>&1 &
    BLOCKCHAIN_PID=$!
    
    # 等待服务启动
    sleep 3
    
    # 检查服务是否启动成功
    if curl -s http://127.0.0.1:8080/health > /dev/null; then
        log_success "区块链认证微服务启动成功 (PID: $BLOCKCHAIN_PID)"
        echo $BLOCKCHAIN_PID > blockchain-auth.pid
    else
        log_error "区块链认证微服务启动失败"
        cat blockchain-auth.log
        exit 1
    fi
    
    cd "$PROJECT_ROOT"
}

# 构建并启动4x-ui主服务
start_main_service() {
    log_info "构建并启动4x-ui主服务..."
    
    # 构建Go项目
    if go build -o x-ui main.go; then
        log_success "4x-ui主服务构建完成"
    else
        log_error "4x-ui主服务构建失败"
        exit 1
    fi
    
    # 启动主服务
    log_info "启动4x-ui主服务..."
    ./x-ui &
    MAIN_PID=$!
    
    # 等待主服务启动
    sleep 5
    
    log_success "4x-ui主服务启动成功 (PID: $MAIN_PID)"
    echo $MAIN_PID > x-ui.pid
}

# 停止服务
stop_services() {
    log_info "停止所有服务..."
    
    # 停止区块链认证微服务
    if [ -f "blockchain-auth.pid" ]; then
        BLOCKCHAIN_PID=$(cat blockchain-auth.pid)
        if kill -0 $BLOCKCHAIN_PID 2>/dev/null; then
            kill $BLOCKCHAIN_PID
            log_success "区块链认证微服务已停止"
        fi
        rm -f blockchain-auth.pid
    fi
    
    # 停止主服务
    if [ -f "x-ui.pid" ]; then
        MAIN_PID=$(cat x-ui.pid)
        if kill -0 $MAIN_PID 2>/dev/null; then
            kill $MAIN_PID
            log_success "4x-ui主服务已停止"
        fi
        rm -f x-ui.pid
    fi
    
    # 清理其他可能的进程
    pkill -f "blockchain-auth" || true
    pkill -f "x-ui" || true
}

# 检查服务状态
check_status() {
    log_info "检查服务状态..."
    
    # 检查区块链认证微服务
    if curl -s http://127.0.0.1:8080/health > /dev/null; then
        log_success "✅ 区块链认证微服务运行中"
    else
        log_error "❌ 区块链认证微服务未运行"
    fi
    
    # 检查主服务（假设运行在54321端口）
    if netstat -tuln | grep :54321 > /dev/null; then
        log_success "✅ 4x-ui主服务运行中"
    else
        log_error "❌ 4x-ui主服务未运行"
    fi
}

# 显示日志
show_logs() {
    echo -e "${BLUE}=== 区块链认证微服务日志 ===${NC}"
    if [ -f "$BLOCKCHAIN_AUTH_DIR/blockchain-auth.log" ]; then
        tail -n 20 "$BLOCKCHAIN_AUTH_DIR/blockchain-auth.log"
    else
        log_warning "日志文件不存在"
    fi
}

# 主函数
main() {
    case "${1:-start}" in
        "start")
            log_info "🚀 启动 4x-ui 区块链认证系统..."
            check_dependencies
            build_blockchain_service
            start_blockchain_service
            start_main_service
            log_success "🎉 系统启动完成！"
            echo ""
            echo "访问地址:"
            echo "  - 主面板: http://localhost:54321"
            echo "  - 区块链登录: http://localhost:54321/blockchain-login"
            echo "  - 认证API: http://localhost:8080/health"
            ;;
        "stop")
            stop_services
            ;;
        "restart")
            stop_services
            sleep 2
            main start
            ;;
        "status")
            check_status
            ;;
        "logs")
            show_logs
            ;;
        "build")
            check_dependencies
            build_blockchain_service
            log_success "构建完成"
            ;;
        *)
            echo "使用方法: $0 {start|stop|restart|status|logs|build}"
            echo ""
            echo "命令说明:"
            echo "  start   - 启动所有服务"
            echo "  stop    - 停止所有服务"
            echo "  restart - 重启所有服务"
            echo "  status  - 检查服务状态"
            echo "  logs    - 显示日志"
            echo "  build   - 仅构建项目"
            exit 1
            ;;
    esac
}

# 信号处理
trap 'stop_services; exit 0' SIGINT SIGTERM

# 运行主函数
main "$@" 