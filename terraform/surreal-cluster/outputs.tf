output "surrealdb_service_endpoint" {
  value = "http://${aws_lb.surrealdb.dns_name}"
}