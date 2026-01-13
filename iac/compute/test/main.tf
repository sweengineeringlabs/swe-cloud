# main.tf for the 'test' provider

resource "null_resource" "test_compute_instance" {
  triggers = {
    # These triggers will be displayed in the Terraform plan and apply output.
    # This demonstrates that the variables from the facade are being correctly
    # passed down to this module.
    instance_name = var.instance_name
    instance_type = var.instance_type
  }

  lifecycle {
    # This ensures the resource is shown as "created" every time.
    create_before_destroy = true
  }
}
