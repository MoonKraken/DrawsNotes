variable "vpc_id" {
  description = "VPC ID where resources will be created"
  type        = string
}

variable "private_subnets" {
  description = "List of private subnet IDs"
  type        = list(string)
}

variable "public_subnets" {
  description = "List of public subnet IDs"
  type        = list(string)
}

variable "app_security_group_id" {
  description = "Security group ID of the application"
  type        = string
}