package model

type Code struct {
	Type     string      `bson:"type,omitempty" json:"type,omitempty"`
	Market   int         `bson:"market,omitempty" json:"market,omitempty"`
	Zsz      interface{} `bson:"zsz,omitempty" json:"zsz,omitempty"`
	Ltgb     interface{} `bson:"ltgb,omitempty" json:"ltgb,omitempty"`
	Code     string      `bson:"code,omitempty" json:"code,omitempty"`
	Zgb      interface{} `bson:"zgb,omitempty" json:"zgb,omitempty"`
	Name     string      `bson:"name,omitempty" json:"name,omitempty"`
	Bk       string      `bson:"bk,omitempty" json:"bk,omitempty"`
	Roe      float64     `bson:"roe,omitempty" json:"roe,omitempty"`
	Ltsz     interface{} `bson:"ltsz,omitempty" json:"ltsz,omitempty"`
	Ssdate   string      `bson:"ssdate,omitempty" json:"ssdate,omitempty"`
	UpdateAt int64       `bson:"update_at,omitempty" json:"update_at,omitempty"`
}
