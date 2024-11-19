// app-cluster.ts

import * as cdk from 'aws-cdk-lib';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ecs_patterns from 'aws-cdk-lib/aws-ecs-patterns';
import { Construct } from 'constructs';

export interface AppClusterProps {
    vpc: ec2.Vpc;
    surrealDBUrl: string;
}

export class AppCluster extends Construct {
    public readonly loadBalancerDnsName: string;
    public readonly securityGroup: ec2.SecurityGroup;

    constructor(scope: Construct, id: string, props: AppClusterProps) {
        super(scope, id);

        // Create an ECS cluster for the application
        const appCluster = new ecs.Cluster(this, 'AppCluster', {
            vpc: props.vpc,
        });

        // Add EC2 capacity to the cluster
        appCluster.addCapacity('DefaultAutoScalingGroup', {
            instanceType: ec2.InstanceType.of(ec2.InstanceClass.T3, ec2.InstanceSize.MICRO),
            desiredCapacity: 2,
        });

        // Create a security group for the application
        const appSecurityGroup = new ec2.SecurityGroup(this, 'AppSecurityGroup', {
            vpc: props.vpc,
            description: 'Security group for the application',
            allowAllOutbound: true,
        });

        // Create a load-balanced EC2 service for the application
        const appService = new ecs_patterns.ApplicationLoadBalancedEc2Service(this, 'AppService', {
            cluster: appCluster,
            memoryLimitMiB: 512,
            taskImageOptions: {
                image: ecs.ContainerImage.fromRegistry('amazon/amazon-ecs-sample'),
                environment: {
                    SURREALDB_URL: props.surrealDBUrl,
                },
            },
            desiredCount: 2,
            publicLoadBalancer: true,
        });

        // Add the application security group to the EC2 service
        appService.service.connections.addSecurityGroup(appSecurityGroup);

        this.loadBalancerDnsName = appService.loadBalancer.loadBalancerDnsName;
        this.securityGroup = appSecurityGroup;
    }
}