package controller

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"x-ui/web/service/blockchain"
	"x-ui/web/session"
)

type BlockchainController struct {
	blockchainService *blockchain.BlockchainAuthService
}

func NewBlockchainController() *BlockchainController {
	return &BlockchainController{
		blockchainService: blockchain.NewBlockchainAuthService(),
	}
}

// TON钱包登录
func (c *BlockchainController) TonLogin(ctx *gin.Context) {
	var req blockchain.TonLoginRequest
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"success": false,
			"message": "Invalid request format: " + err.Error(),
		})
		return
	}

	// 验证TON登录
	resp, err := c.blockchainService.VerifyTonLogin(req)
	if err != nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"success": false,
			"message": "TON login failed: " + err.Error(),
		})
		return
	}

	// 创建会话
	if resp.Token != nil {
		sess := session.GetSession(ctx)
		sess.Set("login", true)
		sess.Set("username", resp.Address)
		sess.Set("blockchain_chain", resp.Chain)
		sess.Set("blockchain_token", *resp.Token)
		sess.Save()
	}

	ctx.JSON(http.StatusOK, gin.H{
		"success":    resp.Success,
		"message":    resp.Message,
		"address":    resp.Address,
		"chain":      resp.Chain,
		"expires_at": resp.ExpiresAt,
	})
}

// SUI钱包登录
func (c *BlockchainController) SuiLogin(ctx *gin.Context) {
	var req blockchain.SuiLoginRequest
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"success": false,
			"message": "Invalid request format: " + err.Error(),
		})
		return
	}

	// 验证SUI登录
	resp, err := c.blockchainService.VerifySuiLogin(req)
	if err != nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"success": false,
			"message": "SUI login failed: " + err.Error(),
		})
		return
	}

	// 创建会话
	if resp.Token != nil {
		sess := session.GetSession(ctx)
		sess.Set("login", true)
		sess.Set("username", resp.Address)
		sess.Set("blockchain_chain", resp.Chain)
		sess.Set("blockchain_token", *resp.Token)
		sess.Save()
	}

	ctx.JSON(http.StatusOK, gin.H{
		"success":    resp.Success,
		"message":    resp.Message,
		"address":    resp.Address,
		"chain":      resp.Chain,
		"expires_at": resp.ExpiresAt,
	})
}

// OKX钱包登录
func (c *BlockchainController) OkxLogin(ctx *gin.Context) {
	var req blockchain.OkxLoginRequest
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"success": false,
			"message": "Invalid request format: " + err.Error(),
		})
		return
	}

	// 验证OKX登录
	resp, err := c.blockchainService.VerifyOkxLogin(req)
	if err != nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"success": false,
			"message": "OKX login failed: " + err.Error(),
		})
		return
	}

	// 创建会话
	if resp.Token != nil {
		sess := session.GetSession(ctx)
		sess.Set("login", true)
		sess.Set("username", resp.Address)
		sess.Set("blockchain_chain", resp.Chain)
		sess.Set("blockchain_token", *resp.Token)
		sess.Save()
	}

	ctx.JSON(http.StatusOK, gin.H{
		"success":    resp.Success,
		"message":    resp.Message,
		"address":    resp.Address,
		"chain":      resp.Chain,
		"expires_at": resp.ExpiresAt,
	})
}

// 验证区块链会话
func (c *BlockchainController) VerifySession(ctx *gin.Context) {
	sess := session.GetSession(ctx)
	
	// 检查会话中是否有区块链令牌
	token := sess.Get("blockchain_token")
	if token == nil {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"valid": false,
			"message": "No blockchain session found",
		})
		return
	}

	// 验证令牌
	result, err := c.blockchainService.VerifyToken(token.(string))
	if err != nil {
		// 清除无效会话
		sess.Delete("blockchain_token")
		sess.Delete("blockchain_chain")
		sess.Save()
		
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"valid": false,
			"message": "Invalid blockchain token: " + err.Error(),
		})
		return
	}

	ctx.JSON(http.StatusOK, result)
}

// 区块链登出
func (c *BlockchainController) Logout(ctx *gin.Context) {
	sess := session.GetSession(ctx)
	sess.Delete("blockchain_token")
	sess.Delete("blockchain_chain")
	sess.Save()

	ctx.JSON(http.StatusOK, gin.H{
		"success": true,
		"message": "Blockchain logout successful",
	})
} 