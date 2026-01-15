# Azure Key Vault Secret

resource "azurerm_key_vault_secret" "this" {
  name         = var.name
  value        = var.secret_value
  key_vault_id = var.key_vault_id
  tags         = var.tags
}

output "secret_id" {
  value = azurerm_key_vault_secret.this.id
}

output "secret_name" {
  value = azurerm_key_vault_secret.this.name
}
