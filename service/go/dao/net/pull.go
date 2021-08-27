package net

import (
	"service/config"

	"github.com/gin-gonic/gin"
)

func get_all_code() {
	engine := gin.Default()
	engine.GET(config.Get().Net.FundCodeUrl, func(ctx *gin.Context) {
		if ctx.Err() != nil {
			println(ctx.Err())
			return
		}
	})
}
