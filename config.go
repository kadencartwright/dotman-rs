package main

import (
	"os"
	"path/filepath"

	"github.com/pelletier/go-toml/v2"
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
	err := os.Chdir(cfg.BaseDirectory)
	if err != nil {
		errs = append(errs)
	}
	for _, pm := range cfg.PathMappings {
		err = pm.BackupHost()
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
	err := os.Chdir(cfg.BaseDirectory)
	if err != nil {
		errs = append(errs)
	}
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
	wd, err := os.Getwd()

	if err != nil {
		return err
	}
    hd,err:= os.UserHomeDir()
	qr := filepath.Join(wd, pm.RepoPath)
	qh := filepath.Join(hd, pm.HostPath)
	return linkPaths(qr, qh)
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
	err = toml.Unmarshal(file, &cfg)

	if err != nil {
		return nil, err
	}
	return &cfg, nil
}
