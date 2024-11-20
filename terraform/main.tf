module "networking" {
  source = "./networking"
}

module "surrealdb" {
  source = "./surreal-cluster"

  vpc_id          = module.networking.vpc_id
  private_subnets = module.networking.private_subnets
  public_subnets = module.networking.public_subnets
  app_security_group_id = module.draws-cluster.cluster_security_group_id
}

module "draws-cluster" {
  source = "./draws-cluster"

  vpc_id          = module.networking.vpc_id
  private_subnets = module.networking.private_subnets
  public_subnets = module.networking.public_subnets
}
