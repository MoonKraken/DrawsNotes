module "surrealdb_security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "5.2.0"

  name        = "surrealdb-security-group"
  description = "Security group for the SurrealDB ECS cluster"
  vpc_id      = var.vpc_id

  ingress_with_source_security_group_id = [
    {
      from_port                = 8000
      to_port                  = 8000
      protocol                 = "tcp"
      source_security_group_id = var.app_security_group_id
    }
  ]

  egress_with_cidr_blocks = [
    {
      from_port   = 0
      to_port     = 0
      protocol    = "-1"
      cidr_blocks = "0.0.0.0/0"
    }
  ]
}

# First create the ECS cluster
module "surrealdb_ecs" {
  source  = "terraform-aws-modules/ecs/aws"
  version = "5.11.4"

  cluster_name = "surrealdb-cluster"
  tags = {
    Environment = "production"
  }
}

# Add the task definition
resource "aws_ecs_task_definition" "surrealdb_task" {
  family                   = "surrealdb"
  requires_compatibilities = ["FARGATE"]
  network_mode            = "awsvpc"
  cpu                     = 256
  memory                  = 512

  container_definitions = jsonencode([
    {
      name      = "surrealdb"
      image     = "surrealdb/surrealdb:latest"
      cpu       = 256
      memory    = 512
      essential = true
      command   = ["start", "--log", "debug"]
      portMappings = [
        {
          containerPort = 8000
          hostPort      = 8000
          protocol      = "tcp"
        }
      ]
    }
  ])
}

# Create ALB Security Group
module "alb_security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "5.2.0"

  name        = "surrealdb-alb-security-group"
  description = "Security group for the SurrealDB ALB"
  vpc_id      = var.vpc_id

  ingress_with_cidr_blocks = [
    {
      from_port   = 80
      to_port     = 80
      protocol    = "tcp"
      cidr_blocks = "0.0.0.0/0"  # Be more restrictive in production
    }
  ]

  egress_with_cidr_blocks = [
    {
      from_port   = 0
      to_port     = 0
      protocol    = "-1"
      cidr_blocks = "0.0.0.0/0"
    }
  ]
}

# Create the ALB
resource "aws_lb" "surrealdb" {
  name               = "surrealdb-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [module.alb_security_group.security_group_id]
  subnets            = var.public_subnets

  tags = {
    Environment = "production"
  }
}

# Create ALB target group
resource "aws_lb_target_group" "surrealdb" {
  name        = "surrealdb-target-group"
  port        = 8000
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    enabled             = true
    healthy_threshold   = 2
    interval            = 30
    matcher            = "200"
    path               = "/health"  # Adjust this to a valid health check endpoint
    port               = "traffic-port"
    timeout            = 5
    unhealthy_threshold = 2
  }
}

# Create ALB listener
resource "aws_lb_listener" "surrealdb" {
  load_balancer_arn = aws_lb.surrealdb.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.surrealdb.arn
  }
}

# Update the ECS service to use the ALB
resource "aws_ecs_service" "surrealdb_service" {
  name            = "surrealdb-service"
  cluster         = module.surrealdb_ecs.cluster_id
  launch_type     = "FARGATE"
  desired_count   = 1

  network_configuration {
    subnets         = var.private_subnets
    security_groups = [module.surrealdb_security_group.security_group_id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.surrealdb.arn
    container_name   = "surrealdb"
    container_port   = 8000
  }

  task_definition = aws_ecs_task_definition.surrealdb_task.arn
}