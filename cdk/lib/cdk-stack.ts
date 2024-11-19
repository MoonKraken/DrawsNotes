// cdk-stack.ts

import * as cdk from 'aws-cdk-lib';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import { Construct } from 'constructs';
import { SurrealDBCluster } from './surreal-db-cluster';
import { AppCluster } from './app-cluster';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Create a VPC
    const vpc = new ec2.Vpc(this, 'SurrealDBVpc', {
      maxAzs: 2,
    });

    // Create the SurrealDB cluster
    const surrealDBCluster = new SurrealDBCluster(this, 'SurrealDBCluster', { vpc });

    // Output the SurrealDB load balancer DNS name
    new cdk.CfnOutput(this, 'SurrealDBEndpoint', {
      value: surrealDBCluster.loadBalancerDnsName,
      description: 'SurrealDB Endpoint',
    });

    // Create the application cluster
    const appCluster = new AppCluster(this, 'AppCluster', {
      vpc,
      surrealDBUrl: `http://${surrealDBCluster.loadBalancerDnsName}:8000`,
    });

    // Allow inbound traffic from the application to SurrealDB
    surrealDBCluster.connections.allowFrom(appCluster.securityGroup, ec2.Port.tcp(8000));

    // Output the application load balancer DNS name
    new cdk.CfnOutput(this, 'AppEndpoint', {
      value: appCluster.loadBalancerDnsName,
      description: 'Application Endpoint',
    });
  }
}