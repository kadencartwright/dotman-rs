package main

import (
	"github.com/pelletier/go-toml/v2"
	"os"
	"path/filepath"
)

type Config struct {
	BaseDirectory string        `toml:"base_directory"`
	PathMappings  []PathMapping `toml:"path_mappings"`
}

type PathMapping struct {
	RepoPath string `toml:"repo_path"`
	HostPath string `toml:"host_path"`
}

// using the config values, links files fron their location in the repo to related host paths
func (cfg *Config) Link() []error {
	errs := []error{}
	errs = append(errs, os.Chdir(cfg.BaseDirectory))
	for _, pm := range cfg.PathMappings {
		err := pm.BackupHost()
		if err != nil {
			errs = append(errs, err)
		}
		err = pm.LinkPaths()
		if err != nil {
			errs = append(errs, err)
		}
	}
	if len(errs) > 0 {
		return errs
	}
	return nil
}

// using the config values, copies files from their location on the host into the repo
func (cfg *Config) Copy() []error {
	errs := []error{}
	errs = append(errs, os.Chdir(cfg.BaseDirectory))
	for _, pm := range cfg.PathMappings {
		err := pm.CopyRecursively()
		if err != nil {
			errs = append(errs, err)
		}
	}
	if len(errs) > 0 {
		return errs
	}
	return nil

}
func (pm *PathMapping) LinkPaths() error {
	return linkPaths(pm.RepoPath, pm.HostPath)
}
func (pm *PathMapping) BackupHost() error {
	return backup(pm.HostPath)
}
func (pm *PathMapping) CopyRecursively() error {
	return copyRecursively(pm.HostPath, pm.RepoPath)
}
func NewConfigFromFile(path string) (*Config, error) {
	wd, err := os.Getwd()
	if err != nil {
		return nil, err
	}
	file, err := os.ReadFile(filepath.Join(wd, path))
	if err != nil {
		return nil, err
	}
	cfg := Config{}
	err = toml.Unmarshal(file, cfg)

	if err != nil {
		return nil, err
	}
	return &cfg, nil
}
