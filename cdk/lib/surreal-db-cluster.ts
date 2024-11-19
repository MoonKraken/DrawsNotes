// surreal-db-cluster.ts

import * as cdk from 'aws-cdk-lib';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecs_patterns from 'aws-cdk-lib/aws-ecs-patterns';
import * as efs from 'aws-cdk-lib/aws-efs';
import { Construct } from 'constructs';

export interface SurrealDBClusterProps {
    vpc: ec2.Vpc;
}

export class SurrealDBCluster extends Construct {
    public readonly loadBalancerDnsName: string;
    public readonly connections: ec2.Connections;

    constructor(scope: Construct, id: string, props: SurrealDBClusterProps) {
        super(scope, id);

        // Create an ECS cluster
        const cluster = new ecs.Cluster(this, 'SurrealDBCluster', {
            vpc: props.vpc,
        });

        // Add EC2 capacity to the cluster with exactly one instance
        cluster.addCapacity('SingleInstanceASG', {
            instanceType: ec2.InstanceType.of(ec2.InstanceClass.T3, ec2.InstanceSize.MICRO),
            desiredCapacity: 1,
            minCapacity: 1,
            maxCapacity: 1,
        });

        // Create an EFS file system
        const fileSystem = new efs.FileSystem(this, 'SurrealDBPersistentVolume', {
            vpc: props.vpc,
            lifecyclePolicy: efs.LifecyclePolicy.AFTER_14_DAYS,
            performanceMode: efs.PerformanceMode.GENERAL_PURPOSE,
            encrypted: true,
            removalPolicy: cdk.RemovalPolicy.RETAIN,
        });

        // Create an EFS access point
        const accessPoint = fileSystem.addAccessPoint('SurrealDBAccessPoint', {
            path: '/surrealdb',
            createAcl: {
                ownerUid: '1000',
                ownerGid: '1000',
                permissions: '755',
            },
            posixUser: {
                uid: '1000',
                gid: '1000',
            },
        });

        // Create a load-balanced EC2 service
        const loadBalancedEc2Service = new ecs_patterns.ApplicationLoadBalancedEc2Service(this, 'SurrealDBService', {
            cluster: cluster,
            memoryLimitMiB: 512,
            taskImageOptions: {
                image: ecs.ContainerImage.fromRegistry('surrealdb/surrealdb:latest'),
                containerPort: 8000,
                /*environment: {
                    SURREAL_USER: 'root',
                    SURREAL_PASS: 'your-secure-password',
                },*/
                command: ['start', '--bind', '0.0.0.0:8000', `surrealkv:///data/surreal.db`],

            },
            desiredCount: 1,
        });

        // Add volume to the task definition
        loadBalancedEc2Service.taskDefinition.addVolume({
            name: 'surrealdb-data',
            efsVolumeConfiguration: {
                fileSystemId: fileSystem.fileSystemId,
                transitEncryption: 'ENABLED',
                authorizationConfig: {
                    accessPointId: accessPoint.accessPointId,
                },
            },
        });

        // Mount the volume to the container
        loadBalancedEc2Service.taskDefinition.defaultContainer?.addMountPoints({
            sourceVolume: 'surrealdb-data',
            containerPath: '/data',
            readOnly: false,
        });

        this.loadBalancerDnsName = loadBalancedEc2Service.loadBalancer.loadBalancerDnsName;

        this.connections = loadBalancedEc2Service.service.connections;
    }
}