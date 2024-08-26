
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
    $transform(sst.aws.Function, (args, _) => {
      args.handler = 'bootstrap';
      args.architecture = 'arm64';
      args.runtime = 'provided.al2023';
      args.timeout = '10 minutes';
      args.memory = '500 MB';
      args.logging = {
        retention: '1 week',
        format: 'json'
      },
      args.environment = {
        STAGE: stage,
        AWS_REGION: '{{aws_region}}',
        LOG_LEVEL: process.env.LOG_LEVEL,
        AWS_ACCESS_KEY_ID: process.env.AWS_ACCESS_KEY_ID,
        AWS_SECRET_ACCESS_KEY: process.env.AWS_SECRET_ACCESS_KEY,
        MONGO_URI: process.env.MONGO_URI,
        {% if s3_bucket %}BUCKET_NAME: bucket.name,{% endif %}
        {% if sqs_queue %}QUEUE_URL: queue.url,{% endif %}
      };
      args.link = [
        {% if s3_bucket %}bucket,{% endif %}
        {% if sqs_queue %}queue,{% endif %}
      ]
    })

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      url: { cors: true, allowCredentials: true },
    });
    {% if s3_bucket %}
    const bucket = new sst.aws.Bucket('{{s3_bucket_name}}', {
      public: true
    });
    {% endif %}
    {% if sqs_queue %}
    const queue = new sst.aws.Queue('{{sqs_queue_name}}', {
      fifo: true
    });
    queue.subscribe({
      handler: 'bootstrap',
      bundle: 'target/lambda/example-fifo',
    })
    {% endif %}
    return {
      {% if s3_bucket %}bucket: bucket.name,{% endif %}
      {% if sqs_queue %}queue: queue.url,{% endif %}
      url: api.url
    }
  },
});
