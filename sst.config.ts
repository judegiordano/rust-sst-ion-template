
/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "rust-sst-ion-template",
      removal: "remove",
      home: "aws",
      providers: {
        aws: {
          region: 'us-east-1'
        }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;

    const api = new sst.aws.Function("api", {
      handler: "bootstrap",
      architecture: "arm64",
      bundle: "./target/lambda/api",
      runtime: 'provided.al2023',
      url: true,
      environment: {
        STAGE: stage,
        MONGO_URI: process.env.MONGO_URI
      }
    });
    return {
      url: api.url
    }
  },
});
