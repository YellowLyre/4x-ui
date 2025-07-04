#!/bin/bash

# 区块链认证系统快速测试脚本
# 作者: YellowLyre

set -e

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🧪 开始测试区块链认证系统...${NC}"

# 1. 测试Rust微服务编译
echo -e "${BLUE}1. 测试 Rust 微服务编译...${NC}"
cd blockchain-auth
if cargo check --quiet; then
    echo -e "${GREEN}✅ Rust 微服务编译测试通过${NC}"
else
    echo -e "${RED}❌ Rust 微服务编译失败${NC}"
    exit 1
fi

# 2. 启动微服务进行健康检查
echo -e "${BLUE}2. 启动微服务并进行健康检查...${NC}"
if [ -f ./target/release/blockchain-auth ]; then
    # 启动服务
    ./target/release/blockchain-auth &
    SERVICE_PID=$!
    echo "微服务 PID: $SERVICE_PID"
    
    # 等待服务启动
    sleep 3
    
    # 健康检查
    if curl -s http://127.0.0.1:8080/health > /dev/null; then
        echo -e "${GREEN}✅ 微服务健康检查通过${NC}"
        
        # 测试健康检查响应
        echo -e "${BLUE}健康检查响应:${NC}"
        curl -s http://127.0.0.1:8080/health | jq '.' 2>/dev/null || curl -s http://127.0.0.1:8080/health
        
    else
        echo -e "${RED}❌ 微服务健康检查失败${NC}"
        kill $SERVICE_PID 2>/dev/null || true
        exit 1
    fi
    
    # 清理
    kill $SERVICE_PID 2>/dev/null || true
    sleep 1
else
    echo -e "${RED}❌ 找不到编译后的微服务可执行文件${NC}"
    exit 1
fi

cd ..

# 3. 测试Go主服务编译
echo -e "${BLUE}3. 测试 Go 主服务编译...${NC}"
if go build -o x-ui-test main.go; then
    echo -e "${GREEN}✅ Go 主服务编译测试通过${NC}"
    rm -f x-ui-test  # 清理测试文件
else
    echo -e "${RED}❌ Go 主服务编译失败${NC}"
    exit 1
fi

# 4. 验证关键文件存在
echo -e "${BLUE}4. 验证关键文件...${NC}"
REQUIRED_FILES=(
    "blockchain-auth/src/main.rs"
    "blockchain-auth/Cargo.toml"
    "web/controller/blockchain_controller.go"
    "web/service/blockchain/auth_service.go"
    "web/html/blockchain_login.html"
    "start-blockchain-auth.sh"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ $file${NC}"
    else
        echo -e "${RED}❌ $file 缺失${NC}"
        exit 1
    fi
done

# 5. 验证Go模块依赖
echo -e "${BLUE}5. 验证 Go 模块依赖...${NC}"
if go mod tidy && go mod verify; then
    echo -e "${GREEN}✅ Go 模块依赖验证通过${NC}"
else
    echo -e "${RED}❌ Go 模块依赖验证失败${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 所有测试通过！系统已准备就绪！${NC}"
echo ""
echo -e "${BLUE}📋 下一步操作:${NC}"
echo "1. 启动完整系统: ./start-blockchain-auth.sh start"
echo "2. 访问区块链登录: http://localhost:54321/blockchain-login"  
echo "3. 查看系统状态: ./start-blockchain-auth.sh status"
echo "4. 查看详细文档: cat BLOCKCHAIN_AUTH_README.md"
echo ""
echo -e "${BLUE}🚀 开始使用区块链钱包登录您的 4x-ui 面板！${NC}" 