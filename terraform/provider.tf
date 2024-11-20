provider "aws" {
  region = var.aws_region
}

variable "aws_region" {
  description = "AWS region to deploy resources in"
  default     = "us-west-2"
}
