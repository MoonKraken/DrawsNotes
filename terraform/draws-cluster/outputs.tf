output "cluster_security_group_id" {
  description = "ID of the ECS cluster security group"
  value       = module.app_security_group.security_group_id
}

output "alb_security_group_id" {
  description = "ID of the ALB security group"
  value       = module.alb_security_group.security_group_id
}