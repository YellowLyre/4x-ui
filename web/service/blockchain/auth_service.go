package blockchain

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"x-ui/logger"
)

// BlockchainAuthService 区块链认证服务
type BlockchainAuthService struct {
	baseURL string
	client  *http.Client
}

// NewBlockchainAuthService 创建新的区块链认证服务实例
func NewBlockchainAuthService() *BlockchainAuthService {
	return &BlockchainAuthService{
		baseURL: "http://127.0.0.1:8080", // 区块链认证微服务地址
		client: &http.Client{
			Timeout: 30 * time.Second,
		},
	}
}

// TON登录请求结构
type TonLoginRequest struct {
	Proof   TonProof `json:"proof"`
	Address string   `json:"address"`
}

type TonProof struct {
	Timestamp uint64    `json:"timestamp"`
	Domain    TonDomain `json:"domain"`
	Signature string    `json:"signature"`
	Payload   string    `json:"payload"`
}

type TonDomain struct {
	Value string `json:"value"`
}

// SUI登录请求结构
type SuiLoginRequest struct {
	SignedTx  string `json:"signed_tx"`
	Address   string `json:"address"`
	Signature string `json:"signature"`
}

// OKX登录请求结构
type OkxLoginRequest struct {
	Signature string `json:"signature"`
	Timestamp string `json:"timestamp"`
	ApiKey    string `json:"api_key"`
	Address   string `json:"address"`
}

// 认证响应结构
type AuthResponse struct {
	Success   bool      `json:"success"`
	Token     *string   `json:"token"`
	Address   string    `json:"address"`
	Chain     string    `json:"chain"`
	ExpiresAt time.Time `json:"expires_at"`
	Message   string    `json:"message"`
}

// 令牌验证请求
type TokenVerifyRequest struct {
	Token string `json:"token"`
}

// TON登录验证
func (s *BlockchainAuthService) VerifyTonLogin(req TonLoginRequest) (*AuthResponse, error) {
	logger.Debug("Starting TON login verification for address:", req.Address)
	
	// 检查微服务是否可用
	if !s.isServiceAvailable() {
		return nil, fmt.Errorf("blockchain auth service is not available")
	}
	
	return s.makeAuthRequest("/auth/ton", req)
}

// SUI登录验证
func (s *BlockchainAuthService) VerifySuiLogin(req SuiLoginRequest) (*AuthResponse, error) {
	logger.Debug("Starting SUI login verification for address:", req.Address)
	
	if !s.isServiceAvailable() {
		return nil, fmt.Errorf("blockchain auth service is not available")
	}
	
	return s.makeAuthRequest("/auth/sui", req)
}

// OKX登录验证
func (s *BlockchainAuthService) VerifyOkxLogin(req OkxLoginRequest) (*AuthResponse, error) {
	logger.Debug("Starting OKX login verification for address:", req.Address)
	
	if !s.isServiceAvailable() {
		return nil, fmt.Errorf("blockchain auth service is not available")
	}
	
	return s.makeAuthRequest("/auth/okx", req)
}

// 验证JWT令牌
func (s *BlockchainAuthService) VerifyToken(token string) (*map[string]interface{}, error) {
	logger.Debug("Verifying blockchain auth token")
	
	if !s.isServiceAvailable() {
		return nil, fmt.Errorf("blockchain auth service is not available")
	}
	
	req := TokenVerifyRequest{Token: token}
	reqBody, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %v", err)
	}
	
	resp, err := s.client.Post(s.baseURL+"/auth/verify", "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		return nil, fmt.Errorf("failed to make request: %v", err)
	}
	defer resp.Body.Close()
	
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %v", err)
	}
	
	var result map[string]interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %v", err)
	}
	
	return &result, nil
}

// 检查微服务是否可用
func (s *BlockchainAuthService) isServiceAvailable() bool {
	resp, err := s.client.Get(s.baseURL + "/health")
	if err != nil {
		logger.Warning("Blockchain auth service health check failed:", err)
		return false
	}
	defer resp.Body.Close()
	
	return resp.StatusCode == http.StatusOK
}

// 通用认证请求方法
func (s *BlockchainAuthService) makeAuthRequest(endpoint string, req interface{}) (*AuthResponse, error) {
	reqBody, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %v", err)
	}
	
	resp, err := s.client.Post(s.baseURL+endpoint, "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		return nil, fmt.Errorf("failed to make request: %v", err)
	}
	defer resp.Body.Close()
	
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %v", err)
	}
	
	var authResp AuthResponse
	if err := json.Unmarshal(body, &authResp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %v", err)
	}
	
	if !authResp.Success {
		return &authResp, fmt.Errorf("authentication failed: %s", authResp.Message)
	}
	
	return &authResp, nil
} 