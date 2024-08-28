
/// <reference path='./.sst/platform/config.d.ts' />

export default $config({
  app(input) {
    return {
      name: '{{project-name}}',
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: '{{aws_region}}' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      AWS_REGION: '{{aws_region}}',
      LOG_LEVEL: process.env.LOG_LEVEL,
      AWS_ACCESS_KEY_ID: process.env.AWS_ACCESS_KEY_ID,
      AWS_SECRET_ACCESS_KEY: process.env.AWS_SECRET_ACCESS_KEY,
      MONGO_URI: process.env.MONGO_URI,
    }
    {% if s3_bucket %}
    const bucket = new sst.aws.Bucket('{{s3_bucket_name}}', {
      public: true
    });{% endif %}
    {% if sqs_queue %}
    const queue = new sst.aws.Queue('{{sqs_queue_name}}', {
      fifo: true
    });
    queue.subscribe({
      architecture: 'arm64',
      runtime: 'provided.al2023',
      handler: 'bootstrap',
      environment,
      bundle: 'target/lambda/{{sqs_queue_name}}-fifo',
    }){% endif %}
    {% if cron_job %}
    const cron = new sst.aws.Cron('{{cron_name}}', {
      schedule: 'rate(1 minute)',
      job: {
        environment,
        architecture: 'arm64',
        runtime: 'provided.al2023',
        handler: 'bootstrap',
        bundle: 'target/lambda/{{cron_name}}',  
      }
    });
    {% endif %}
    
    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '500 MB',
      timeout: '10 minutes',
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging: {
        retention: '1 week',
        format: 'json'
      },
      environment: {
        ...environment,{% if s3_bucket %}BUCKET_NAME: bucket.name,{% endif %}{% if sqs_queue %}QUEUE_URL: queue.url,{% endif %}
      },
      link: [{% if s3_bucket %}bucket,{% endif %}{% if sqs_queue %}queue,{% endif %}]
    });
    
    return {
      url: api.url,{% if s3_bucket %}bucket: bucket.name,{% endif %}{% if sqs_queue %}queue: queue.url,{% endif %}
    }
  },
});
