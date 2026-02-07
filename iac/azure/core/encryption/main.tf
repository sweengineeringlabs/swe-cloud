# Azure Key Vault Key

resource "azurerm_key_vault_key" "this" {
  name         = var.name
  key_vault_id = var.key_vault_id
  key_type     = var.key_type
  key_size     = var.key_size

  key_opts = [
    "decrypt",
    "encrypt",
    "sign",
    "unwrapKey",
    "verify",
    "wrapKey",
  ]

  tags = var.tags
}

output "key_id" {
  value = azurerm_key_vault_key.this.id
}

output "key_name" {
  value = azurerm_key_vault_key.this.name
}
