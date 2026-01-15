# Azure Event Grid Topic

resource "azurerm_eventgrid_topic" "this" {
  name                = var.name
  location            = var.location
  resource_group_name = var.resource_group_name
  tags                = var.tags
}

output "topic_id" {
  value = azurerm_eventgrid_topic.this.id
}

output "topic_name" {
  value = azurerm_eventgrid_topic.this.name
}

output "endpoint" {
  value = azurerm_eventgrid_topic.this.endpoint
}
