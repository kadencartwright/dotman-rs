package main

import (
	"errors"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
)

func linkPaths(source, destination string) error {
	if pathsAreEquivalent(source, destination) {
		return nil
	}
	err := os.Symlink(source, destination)
	if os.IsExist(err) {
		os.RemoveAll(destination)
		return linkPaths(source, destination)
	}
	return err
}
func backup(path string) error {
    cd,err:=os.UserCacheDir()
	if err != nil {
		return err
	}
	backupDir := filepath.Join(cd, "dotman", "backup", path)
	err = os.MkdirAll(backupDir, os.ModePerm)
	if err != nil {
		println(err.Error())
		return err
	}
	return nil
}

func copyRecursively(source, destination string) error {
	//if both paths point to same location, return nil
	if pathsAreEquivalent(source, destination) {
		return nil
	}
	err := os.RemoveAll(filepath.Join(destination))
	if err != nil && !errors.Is(err, fs.ErrNotExist) {
		return err
	}
	cmd := exec.Command("cp", "-r", source, destination)
	err = cmd.Run()
	return err
}
func pathsAreEquivalent(path1, path2 string) bool {
	n1, err := filepath.EvalSymlinks(path1)
	if err != nil {
		return false
	}

	n2, err := filepath.EvalSymlinks(path2)
	if err != nil {
		return false
	}

	return n1 == n2
}
