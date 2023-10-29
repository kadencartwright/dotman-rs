package main

import (
	"github.com/pelletier/go-toml/v2"
	"testing"
)

const doc = `
base_directory = "something"
[[path_mappings]]
repo_path = "repo"
host_path = "host"
`

func TestUnmarshal(t *testing.T) {
	var cfg Config
	err := toml.Unmarshal([]byte(doc), &cfg)
	if err != nil {
		t.Fatal(err)
	}
	t.Log(cfg.BaseDirectory)
	if cfg.BaseDirectory != "something" {
		t.Fatalf("expected \"something\", got %s", cfg.BaseDirectory)
	}

	mapping := cfg.PathMappings[len(cfg.PathMappings)-1]
	if mapping.HostPath != "host" {
		t.Fatalf("expected \"host\", got %s", mapping.HostPath)
	}
	if mapping.RepoPath != "repo" {
		t.Fatalf("expected \"repo\", got %s", mapping.RepoPath)
	}

}
func TestMarshal(t *testing.T) {

}
