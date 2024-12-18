package main

import (
	"github.com/kohdice/g/cmd"
)

const version = "latest"

var revision = "HEAD"

func main() {
	cmd.SetVersion(version)
	cmd.Execute()
}
