package test

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

// TestAllModulesValidate scans the repository for all Terraform modules 
// and runs 'terraform validate' on each one.
func TestAllModulesValidate(t *testing.T) {
	t.Parallel()

	// Find all directories containing .tf files
	modules, err := findAllTerraformModules(".")
	assert.NoError(t, err)

	for _, module := range modules {
		// Capture module path for the closure
		modulePath := module
		
		t.Run(modulePath, func(t *testing.T) {
			t.Parallel()

			opts := &terraform.Options{
				TerraformDir: modulePath,
				// Use -backend=false to skip remote state initialization
				BackendConfig: map[string]interface{}{},
			}

			// Run init and validate
			_, err := terraform.InitAndValidateE(t, opts)
			assert.NoError(t, err, "Module at %s failed validation", modulePath)
		})
	}
}

// findAllTerraformModules recursively searches for directories containing .tf files
func findAllTerraformModules(root string) ([]string, error) {
	var modules []string
	
	err := filepath.Walk(root, func(path string) (os.FileInfo, error) {
		// Skip .terraform directories and hidden files
		if info, err := os.Stat(path); err == nil && info.IsDir() {
			if strings.Contains(path, ".terraform") || strings.Contains(path, ".git") {
				return filepath.SkipDir, nil
			}
		}

		// If we find a .tf file, the current directory is a module
		if filepath.Ext(path) == ".tf" {
			dir := filepath.Dir(path)
			// Avoid duplicates
			if !contains(modules, dir) {
				modules = append(modules, dir)
			}
		}
		return nil, nil
	})

	return modules, err
}

func contains(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}
