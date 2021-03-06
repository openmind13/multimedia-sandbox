package main

import "github.com/BurntSushi/toml"

type Config struct {
	Host      string `toml:"host"`
	ApiKey    string `toml:"api_key"`
	ApiSecret string `toml:"api_secret"`
	Identity  string `toml:"identity"`
	Token     string `toml:"token"`
	RoomName  string `toml:"room_name"`
}

var (
	cfgPath = "./config.toml"
)

func GetConfig() (Config, error) {
	cfg := Config{}
	if _, err := toml.DecodeFile(cfgPath, &cfg); err != nil {
		return cfg, err
	}
	return cfg, nil
}
