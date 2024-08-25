
/// <reference path='./.sst/platform/config.d.ts' />

export default $config({
  app(input) {
    return {
      name: 'rust-sst-ion-template',
      removal: 'remove',
      home: 'aws',
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
    $transform(sst.aws.Function, (args, _) => {
      args.handler = 'bootstrap';
      args.architecture = 'arm64';
      args.runtime = 'provided.al2023';
      args.timeout = '10 minutes';
      args.memory = '500 MB';
      args.environment = {
        STAGE: stage,
        LOG_LEVEL: process.env.MONGO_URI,
        MONGO_URI: process.env.MONGO_URI
      };
    })

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      url: { cors: true, allowCredentials: true },
    });
    return {
      url: api.url
    }
  },
});
