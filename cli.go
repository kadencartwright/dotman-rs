package main

import (
	"flag"
	"fmt"
	"os"
)

func runCli() {
	cmd := flag.NewFlagSet("dotman", flag.ExitOnError)
	configPath := cmd.String("file", "", "path to a Toml formatted dotman config file")
	cmd.Parse(os.Args[2:])
	if *configPath == "" {
		fmt.Println("missing required parameter '--file")
		os.Exit(1)
	}
	config, err := NewConfigFromFile(*configPath)
	if err != nil {
		fmt.Println("failed to parse config file")
		os.Exit(1)

	}
	switch os.Args[1] {
	case "copy":
		errs := config.Copy()
		if len(errs) != 0 {
			for _, v := range errs {
				fmt.Println(v.Error())
			}
			os.Exit(1)
		}
	case "link":
		errs := config.Link()
		if len(errs) != 0 {
			for _, v := range errs {
				fmt.Println(v.Error())
			}
			os.Exit(1)
		}
	default:
		fmt.Println("expected 'copy' or 'link' subcommands")
		os.Exit(1)
	}
}
