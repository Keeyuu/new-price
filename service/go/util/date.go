package util

import "time"

func GetDay() int64 {
	return (time.Now().Unix())
}
