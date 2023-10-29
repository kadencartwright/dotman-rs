package main

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

func TestLinkPaths(t *testing.T) {
	t.Log("It should Create a link properly")

	tf, err := os.CreateTemp("", "link_target")
	fileContents := []byte("test")
	tf.Write(fileContents)
	defer os.Remove(tf.Name())
	if err != nil {
		t.Fatal(err)
		return
	}

	td, err := os.MkdirTemp("", "link_dir")
	defer os.RemoveAll(td)
	linkPath := filepath.Join(td, "link")
	err = linkPaths(tf.Name(), linkPath)
	defer os.Remove(linkPath)
	if err != nil {
		t.Fatal(err)
		return
	}

	linkContents, err := os.ReadFile(linkPath)
	if err != nil {
		t.Fatal(err)
		return
	}
	if !bytes.Equal(fileContents, linkContents) {
		t.Fatalf("expected %s, got %s", fileContents, linkContents)
	}
}
func TestLinkIfAlreadyLinked(t *testing.T) {
	t.Log("It should non-destructively overrite the linkpath if exists")

	tf, err := os.CreateTemp("", "link_target")
	fileContents := []byte("test")
	tf.Write(fileContents)
	defer os.Remove(tf.Name())
	if err != nil {
		t.Fatal(err)
		return
	}

	td, err := os.MkdirTemp("", "link_dir")
	defer os.RemoveAll(td)
	linkPath := filepath.Join(td, "link")
	err = linkPaths(tf.Name(), linkPath)
	defer os.Remove(linkPath)
	if err != nil {
		t.Fatal(err)
		return
	}
	//run the  link again
	err = linkPaths(tf.Name(), linkPath)
	if err != nil {
		t.Fatal(err)
		return
	}

	linkContents, err := os.ReadFile(linkPath)
	if err != nil {
		t.Fatal(err)
		return
	}
	if !bytes.Equal(fileContents, linkContents) {
		t.Fatalf("expected %s, got %s", fileContents, linkContents)
	}
}
func TestDirectoryLink(t *testing.T) {
	//create a TestDirectory
	target, err := os.MkdirTemp("", "target_dir")
	defer os.RemoveAll(target)
	link, err := os.MkdirTemp("", "link_dir")
	defer os.RemoveAll(link)
	// try to symlink it to another location
	err = linkPaths(target, link)
	if err != nil {
		t.Fatal(err)
		return
	}
	//write a file into the original location
	// make sure file exists in linked location

	fileContents := []byte("test")
	err = os.WriteFile(filepath.Join(target, "testfile"), fileContents, 0666)
	defer os.Remove(filepath.Join(target, "testfile"))
	if err != nil {
		t.Fatal(err)
		return
	}

	linkContents, err := os.ReadFile(filepath.Join(link, "testfile"))
	if err != nil {
		t.Fatal(err)
		return
	}
	if !bytes.Equal(fileContents, linkContents) {
		t.Fatalf("expected %s, got %s", fileContents, linkContents)
	}

}

func TestCopyRecursively(t *testing.T) {
	t.Log("It should copy directories")
	//create a TestDirectory
	src, err := os.MkdirTemp("", "target_dir")
	t.Logf("created src dir %s", src)
	defer os.RemoveAll(src)
	if err != nil {
		t.Fatal(err.Error())
		return
	}
	dest, err := os.MkdirTemp("", "link_dir")
	t.Logf("created dest dir %s", dest)
	defer os.RemoveAll(dest)
	if err != nil {
		t.Fatal(err.Error())
		return
	}
	var filePaths []string
	for i := 0; i < 5; i++ {
		f, err := os.CreateTemp(src, "*")
		t.Logf("created test file %s", f.Name())
		if err != nil {
			t.Fatal(err.Error())
			return
		}
		filePaths = append(filePaths, f.Name())
	}
	err = copyRecursively(src, dest)
	if err != nil {
		t.Fatal(err.Error())
		return
	}
	dir, err := os.ReadDir(dest)
	if err != nil {
		t.Fatal(err.Error())
		return
	}
	if len(dir) != 5 {
		t.Fatal("failed to copy files")
		return
	}

}
func TestPathsAreEquivalent(t *testing.T) {

	t.Log("It should Create a link properly")

	tf, err := os.CreateTemp("", "link_target")
	fileContents := []byte("test")
	tf.Write(fileContents)
	defer os.Remove(tf.Name())
	if err != nil {
		t.Fatal(err)
		return
	}

	td, err := os.MkdirTemp("", "link_dir")
	defer os.RemoveAll(td)
	linkPath := filepath.Join(td, "link")
	err = linkPaths(tf.Name(), linkPath)
	defer os.Remove(linkPath)

	if err != nil {
		t.Fatal(err)
		return
	}

	eq := pathsAreEquivalent(tf.Name(), linkPath)
	if !eq {
		t.Fatalf("expected %t, got %t", true, eq)
	}

}
