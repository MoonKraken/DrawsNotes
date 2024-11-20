module "app_security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "5.2.0"

  name        = "app-security-group"
  description = "Security group for the app ECS cluster"
  vpc_id      = var.vpc_id

  egress_with_cidr_blocks = [
    {
      from_port   = 0
      to_port     = 0
      protocol    = "-1"
      cidr_blocks = "0.0.0.0/0"
    }
  ]
}

# ECS Cluster
module "draws_ecs" {
  source  = "terraform-aws-modules/ecs/aws"
  version = "5.11.4"

  cluster_name = "draws-cluster"
  tags = {
    Environment = "production"
  }
}

# Create an ECR Repository we can use to store the image used by the ECS Service
resource "aws_ecr_repository" "draws_ecr_repo" {
  name                 = "draws-ecr-repo"
  image_tag_mutability = "MUTABLE"
  force_delete        = true
  image_scanning_configuration {
    scan_on_push = true
  }
}

resource "aws_iam_role" "ecs_execution_role" {
  name = "ecs_execution_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ecs_execution_role_policy" {
  role       = aws_iam_role.ecs_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# Add the task definition
resource "aws_ecs_task_definition" "draws_task" {
  family                   = "draws"
  requires_compatibilities = ["FARGATE"]
  network_mode            = "awsvpc"
  cpu                     = 256
  memory                  = 512
  execution_role_arn       = aws_iam_role.ecs_execution_role.arn

  container_definitions = jsonencode([
    {
      name  = "draws"
      image = "${aws_ecr_repository.draws_ecr_repo.repository_url}:latest"  # Use the ECR repo URL
      portMappings = [
        {
          containerPort = 3000
          hostPort      = 3000
        }
      ]
      # Add other container settings as needed
    }
  ])
}

# Create ALB Security Group
module "alb_security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "5.2.0"

  name        = "draws-alb-security-group"
  description = "Security group for the Draws ALB"
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
resource "aws_lb" "draws" {
  name               = "draws-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [module.alb_security_group.security_group_id]
  subnets            = var.public_subnets

  tags = {
    Environment = "production"
  }
}

# Create ALB target group
resource "aws_lb_target_group" "draws" {
  name        = "draws-target-group"
  port        = 3000
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
resource "aws_lb_listener" "draws" {
  load_balancer_arn = aws_lb.draws.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.draws.arn
  }
}

# Update the ECS service to use the ALB
resource "aws_ecs_service" "draws_service" {
  name            = "draws-service"
  cluster         = module.draws_ecs.cluster_id
  launch_type     = "FARGATE"
  desired_count   = 1

  network_configuration {
    subnets         = var.private_subnets
    security_groups = [module.app_security_group.security_group_id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.draws.arn
    container_name   = "draws"
    container_port   = 3000
  }

  task_definition = aws_ecs_task_definition.draws_task.arn

  # Add this block to enable continuous deployment
  deployment_controller {
    type = "ECS"
  }

  # Optional: Add this if you want to force a new deployment when the image changes
  force_new_deployment = true
}