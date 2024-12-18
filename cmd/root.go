package cmd

import (
	"github.com/spf13/cobra"
)

const (
	description     = "A tool to assist with Git operations."
	longDescription = description + `

This application provides commands to simplify and enhance
Git workflows, making it easier to work with repositories.`
)

var rootCmd = &cobra.Command{
	Use:     "g",
	Version: "unknown",
	Short:   description,
	Long:    longDescription,
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func SetVersion(v string) {
	rootCmd.Version = v
}

func Execute() error {
	return rootCmd.Execute()
}
