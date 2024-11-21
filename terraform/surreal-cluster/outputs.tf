output "surrealdb_service_endpoint" {
  value = aws_lb.surrealdb.dns_name
}